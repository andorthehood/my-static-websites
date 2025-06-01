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
use generate::generate;
use std::env;
use std::io::Result;
use server::listen;
mod generate_pagination_pages;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.iter().map(|s| s.as_str()).collect::<Vec<_>>().as_slice() {
        [_, "generate", site_name] => {
            generate(site_name)?;
        }
        [_, "generate"] => {
            eprintln!("Error: Site name is required for generate command.");
            eprintln!("Usage: {} generate <site_name>", args[0]);
            eprintln!("Example: {} generate lepkef.ing", args[0]);
            std::process::exit(1);
        }
        [_, "serve"] => {
            listen();
        }
        [_, unknown_cmd] => {
            println!("Unknown command '{}'. Use 'generate <site_name>' or 'serve'.", unknown_cmd);
        }
        [_] => {
            println!("No command provided. Use 'generate <site_name>' or 'serve'.");
        }
        _ => {
            println!("Too many arguments. Use 'generate <site_name>' or 'serve'.");
        }
    }

    Ok(())
}
