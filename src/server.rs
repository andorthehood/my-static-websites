use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::fs;
use std::path::PathBuf;

fn handle_client(mut stream: TcpStream) {
    stream.set_read_timeout(Some(Duration::new(5, 0))).unwrap();

    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            // println!("Received a request: {}", request);

            // Parse the request to get the path
            let path = request.split_whitespace().nth(1).unwrap_or("/");
            let path = path.trim_start_matches('/');

            // Construct the file path
            let mut file_path = PathBuf::from("out");
            file_path.push(path);

            // If the path doesn't have an extension, assume it's .html
            if file_path.extension().is_none() {
                file_path.set_extension("html");
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

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            return;
        }
    }

    stream.flush().unwrap();
}

pub fn listen() {
    println!("Starting server on 127.0.0.1:2030");
    let listener = TcpListener::bind("127.0.0.1:2030").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}