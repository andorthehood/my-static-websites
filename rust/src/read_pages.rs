use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("_posts");

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();

                        match fs::read_to_string(path) {
                            Ok(contents) => {
                                println!("File contents:\n{}", contents);
                            }
                            Err(e) => {
                                println!("Failed to read file: {}", e);
                            }

                        }
                    },
                    Err(e) => println!("Error reading entry: {}", e),
                }
            }
        },
        Err(e) => println!("Error reading directory: {}", e),
    }
}