use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::Hasher;
use std::io::{self, Read, Write};
use std::path::Path;

use crate::minifier::css::minify_css;

pub fn copy_file_with_versioning(source_path: &str, destination_dir: &str) -> io::Result<String> {
    let source_path = Path::new(source_path);
    let destination_dir = Path::new(destination_dir);

    // Ensure the destination directory exists
    fs::create_dir_all(destination_dir)?;

    // Read the contents of the source file for hashing and processing
    let mut file = File::open(source_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Check if this is a CSS file and minify if so
    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();

    let processed_contents = if extension.to_lowercase() == "css" {
        // Convert to string, minify, then back to bytes
        let css_string = String::from_utf8_lossy(&contents);
        let minified_css = minify_css(&css_string);
        minified_css.into_bytes()
    } else {
        contents
    };

    // Compute a simple hash of the processed contents
    let mut hasher = DefaultHasher::new();
    hasher.write(&processed_contents);
    let hash = hasher.finish();

    // Split the file name and extension, then reassemble with the hash
    let file_stem = source_path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();
    let new_file_name = format!("{file_stem}-{hash:x}.{extension}");

    let destination_path = destination_dir.join(&new_file_name);

    // Write the processed contents to the destination
    let mut dest_file = File::create(&destination_path)?;
    dest_file.write_all(&processed_contents)?;

    Ok(new_file_name)
}
