use super::config::{DEFAULT_SERVER_HOST, DEFAULT_SERVER_PORT};
use super::handle_client::handle_client;
use crate::error::Result;
use std::net::TcpListener;

pub fn listen(site_name: &str) -> Result<()> {
    let server_addr = format!("{DEFAULT_SERVER_HOST}:{DEFAULT_SERVER_PORT}");
    println!("Starting server on http://{server_addr}");
    let listener = TcpListener::bind(&server_addr)?;
    println!("Server is ready and listening for connections!");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream, site_name) {
                    eprintln!("Error handling client: {e}");
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {e}");
            }
        }
    }
    Ok(())
}
