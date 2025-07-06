// Core types and error handling
mod config;
mod error;
mod types;

// File operations
mod file_copier;
mod file_readers;
mod write;

// Template processing
mod layout;
mod load_includes;
mod parsers;
mod template_processors;

// Generation and rendering
mod content_processor;
mod generate;
mod generate_pagination_pages;
mod index_page;
mod render_page;
mod rss_feed;

// Development tools
mod server;
mod watch;

use error::Result;
use generate::generate;
use server::listen;
use std::env;
use watch::watch;

fn print_usage() {
    eprintln!("Available commands:");
    eprintln!("  generate <site_name>  Generate the static site");
    eprintln!("  serve                 Start the development server");
    eprintln!("  watch <site_name>     Watch for changes and regenerate");
    eprintln!("  watch <site_name> --ramdisk  Watch with RAM-based output (Linux only)");
}

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
            listen()?;
        }
        ["watch", site_name] => {
            watch(site_name, false)?;
        }
        ["watch", site_name, "--ramdisk"] | ["watch", "--ramdisk", site_name] => {
            watch(site_name, true)?;
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
