use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::fs;
use std::path::PathBuf;
use crate::error::MyError;
use crate::config::Config;

fn handle_client(mut stream: TcpStream, output_dir: &str) -> Result<(), MyError> {
    stream.set_read_timeout(Some(Duration::new(5, 0)))?;

    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            // println!("Received a request: {}", request);

            // Parse the request to get the path
            let path = request.split_whitespace().nth(1).unwrap_or("/");
            let path = path.trim_start_matches('/');

            // Construct the file path
            let mut file_path = PathBuf::from(output_dir);
            file_path.push(path);

            // If the path doesn't have an extension, assume it's .html
            if file_path.extension().is_none() {
                file_path.set_extension("html");
            }

            // Read the file and construct the response
            println!("File path: {:?}", file_path);
            // fs::canonicalize can return an error, so we need to handle it.
            // fs::read_to_string can also return an error.
            // We'll map these to MyError.
            let contents = fs::canonicalize(&file_path)
                .map_err(MyError::from) // Convert std::io::Error to MyError
                .and_then(|path| {
                    println!("Trying to serve file: {:?}", path);
                    fs::read_to_string(path).map_err(MyError::from) // Convert std::io::Error to MyError
                });

            let response = match contents {
                Ok(contents) => format!("HTTP/1.1 200 OK\r\n\r\n{}", contents),
                Err(e) => {
                    // Log the error or handle it as appropriate
                    eprintln!("Error reading file {:?}: {}", file_path, e);
                    format!("HTTP/1.1 404 Not Found\r\n\r\nFailed to read file") // Simplified error message
                }
            };

            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
        Err(e) => {
            // This error is from stream.read(), which is an std::io::Error
            // We can convert it to MyError and return it, or handle it specifically
            eprintln!("Failed to read from stream: {}", e);
            return Err(MyError::from(e)); // Convert and return
        }
    }

    Ok(())
}

pub fn listen() -> Result<(), MyError> {
    // Load configuration
    let config = Config::load("config.toml")?;
    let output_dir_for_server = config.output_dir.clone(); // Clone for use in closure

    println!("Starting server on 127.0.0.1:2030, serving from {}/", output_dir_for_server);
    let listener = TcpListener::bind("127.0.0.1:2030")?;

    for stream in listener.incoming() {
        let stream = stream?;
        // Pass the output directory to handle_client
        // Need to clone output_dir_for_server if handle_client were to be threaded,
        // but for sequential it's fine. However, to avoid borrowing issues if
        // handle_client's signature changes later or if it's moved,
        // passing a reference or ensuring handle_client takes &str is good.
        // Current handle_client takes &str, so passing &output_dir_for_server is fine.
        if let Err(e) = handle_client(stream, &output_dir_for_server) {
            eprintln!("Error handling client: {}", e);
        }
    }
    Ok(())
}