mod file_copier;
mod file_readers;
mod front_matter;
mod generate;
mod handlebars;
mod layout;
mod load_includes;
mod markdown;
mod markdown_with_front_matter;
mod write;
mod server;
mod liquid;
mod error;
mod config;
use generate::generate;
use std::env;
use server::listen;
use error::MyError;
mod generate_pagination_pages;

fn main() -> Result<(), MyError> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "generate" => {
                generate()?;
            }
            "serve" => {
                listen()?;
            }
            _ => println!("Unknown command. Use 'generate' or 'serve'."),
        }
    } else {
        println!("No command provided. Use 'generate' or 'serve'.");
    }

    Ok(())
}
