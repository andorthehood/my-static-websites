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

    if args.len() > 1 {
        match args[1].as_str() {
            "generate" => {
                if args.len() < 3 {
                    eprintln!("Error: Site name is required for generate command.");
                    eprintln!("Usage: {} generate <site_name>", args[0]);
                    eprintln!("Example: {} generate lepkef.ing", args[0]);
                    std::process::exit(1);
                }
                let site_name = &args[2];
                generate(site_name)?;
            }
            "serve" => {
                listen();
            }
            _ => println!("Unknown command. Use 'generate <site_name>' or 'serve'."),
        }
    } else {
        println!("No command provided. Use 'generate <site_name>' or 'serve'.");
    }

    Ok(())
}
