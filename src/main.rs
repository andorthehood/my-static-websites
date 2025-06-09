mod error;
mod file_copier;
mod file_readers;
mod front_matter;
mod generate;
mod generate_pagination_pages;
mod handlebars;
mod index_page;
mod layout;
mod template_processors;
mod load_includes;

mod server;
mod template_processor;
mod write;

use generate::generate;
use std::env;
use error::Result;
use server::listen;

fn handle_command(args: &[&str]) -> Result<()> {
    match args {
        ["generate", site_name] => {
            generate(site_name)?;
        }
        ["generate"] => {
            eprintln!("Error: Site name is required for generate command.");
            eprintln!("Usage: {} generate <site_name>", args[0]);
            eprintln!("Example: {} generate lepkef.ing", args[0]);
            std::process::exit(1);
        }
        ["serve"] => {
            listen();
        }
        [unknown_cmd] => {
            println!("Unknown command '{}'. Use 'generate <site_name>' or 'serve'.", unknown_cmd);
        }
        [] => {
            println!("No command provided. Use 'generate <site_name>' or 'serve'.");
        }
        _ => {
            println!("Too many arguments. Use 'generate <site_name>' or 'serve'.");
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    handle_command(&args[1..])
}
