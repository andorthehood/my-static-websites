// Core types and error handling
mod config;
mod error;
mod types;

// File operations
mod file_copier;
mod file_readers;
mod load_data;
mod write;

// Template processing
mod layout;
mod load_includes;
mod minifier;
mod parsers;
mod template_processors;

// Converters
mod converters;

// Generation and rendering
mod generate;
mod generate_category_pages;
mod generate_pagination_pages;
mod render_page;

// Development tools
mod server;
mod watch;

use config::SiteConfig;
use error::Result;
use generate::generate;
use server::listen;
use std::env;
use watch::watch;

fn print_usage() {
    eprintln!("Available commands:");
    eprintln!("  generate <site_name>  Generate the static site");
    eprintln!("  serve <site_name>     Start the development server for a site");
    eprintln!("  watch <site_name>     Watch for changes and regenerate");
    eprintln!("  watch <site_name> --ramdisk  Watch with RAM-based output (Linux only)");
}

fn handle_command(args: &[&str]) -> Result<()> {
    // Create and validate configuration
    let config = SiteConfig::from_environment();
    if let Err(error) = config.validate() {
        eprintln!("Configuration error: {error}");
        std::process::exit(1);
    }

    match args {
        ["generate", site_name] => {
            generate(site_name, &config)?;
        }
        ["generate"] => {
            eprintln!("Error: Site name is required for generate command.");
            eprintln!("Usage: {} generate <site_name>", args[0]);
            eprintln!("Example: {} generate lepkef.ing", args[0]);
            std::process::exit(1);
        }
        ["serve", site_name] => {
            listen(site_name, &config)?;
        }
        ["watch", site_name] => {
            watch(site_name, false, &config)?;
        }
        ["watch", site_name, "--ramdisk"] | ["watch", "--ramdisk", site_name] => {
            watch(site_name, true, &config)?;
        }
        [unknown_command] => {
            eprintln!("Error: Unknown command '{unknown_command}'");
            print_usage();
            std::process::exit(1);
        }
        _ => {
            eprintln!("Error: No command specified");
            print_usage();
            std::process::exit(1);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(std::string::String::as_str).collect();
    handle_command(&args[1..])
}
