use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::fs;
use std::path::PathBuf;
use crate::error::{Error, Result};
use crate::types::{OUTPUT_DIR, DEFAULT_SERVER_HOST, DEFAULT_SERVER_PORT};

fn handle_client(mut stream: TcpStream) -> Result<()> {
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
            let mut file_path = PathBuf::from(OUTPUT_DIR);
            
            // If path is empty or just "/", serve index.html
            if path.is_empty() {
                file_path.push("index.html");
            } else {
                file_path.push(path);
                // If the path doesn't have an extension, assume it's .html
                if file_path.extension().is_none() {
                    file_path.set_extension("html");
                }
            }

            // Read the file and construct the response
            println!("File path: {:?}", file_path);
            let response = match fs::canonicalize(&file_path).and_then(|path| {
                println!("Trying to serve file: {:?}", path);
                fs::read_to_string(path)
            }) {
                Ok(contents) => format!("HTTP/1.1 200 OK\r\n\r\n{}", contents),
                Err(e) => format!("HTTP/1.1 404 Not Found\r\n\r\nFailed to read file: {}", e),
            };

            stream.write_all(response.as_bytes())?;
            stream.flush()?;
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            Err(Error::Io(e))
        }
    }
}

pub fn listen() -> Result<()> {
    let server_addr = format!("{}:{}", DEFAULT_SERVER_HOST, DEFAULT_SERVER_PORT);
    println!("Starting server on {}", server_addr);
    let listener = TcpListener::bind(&server_addr)?;
    println!("Server is ready and listening for connections!");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}
