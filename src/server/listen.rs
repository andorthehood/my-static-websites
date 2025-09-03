use crate::config::SiteConfig;
use crate::error::Result;
use super::handle_client::handle_client;
use std::net::TcpListener;

pub fn listen(site_name: &str, config: &SiteConfig) -> Result<()> {
    let server_addr = format!("{}:{}", config.server_host, config.server_port);
    println!("Starting server on http://{server_addr}");
    let listener = TcpListener::bind(&server_addr)?;
    println!("Server is ready and listening for connections!");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream, site_name, config) {
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
