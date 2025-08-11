use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::Hasher;
use std::io::{self, Read, Write};
use std::path::Path;

use crate::minifier::css::minify_css;
use crate::minifier::js::minify_js;

pub fn copy_file_with_versioning(source_path: &str, destination_dir: &str) -> io::Result<String> {
    let source_path = Path::new(source_path);
    let destination_dir = Path::new(destination_dir);

    // Ensure the destination directory exists
    fs::create_dir_all(destination_dir)?;

    // Read the contents of the source file for hashing and processing
    let mut file = File::open(source_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Check file extension to determine if minification is needed
    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();

    let processed_contents = match extension.to_lowercase().as_str() {
        "css" => {
            // Convert to string, minify, then back to bytes
            let css_string = String::from_utf8_lossy(&contents);
            let minified_css = minify_css(&css_string);
            minified_css.into_bytes()
        }
        "js" => {
            // Convert to string, minify, then back to bytes
            let js_string = String::from_utf8_lossy(&contents);
            let minified_js = minify_js(&js_string);
            minified_js.into_bytes()
        }
        _ => contents,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_copy_css_file_with_minification() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");
        fs::create_dir_all(&source_dir).unwrap();

        // Create a CSS file with unnecessary whitespace
        let css_content = "body   {   margin:   0;   padding:   0;   }";
        let source_file = source_dir.join("style.css");
        fs::write(&source_file, css_content).unwrap();

        // Copy with versioning and minification
        let result =
            copy_file_with_versioning(source_file.to_str().unwrap(), dest_dir.to_str().unwrap());

        assert!(result.is_ok());
        let new_filename = result.unwrap();

        // Verify the file was created with a hash in the name
        assert!(new_filename.starts_with("style-"));
        assert!(new_filename.ends_with(".css"));

        // Read the copied file and verify it's minified
        let copied_file = dest_dir.join(&new_filename);
        let copied_content = fs::read_to_string(copied_file).unwrap();
        assert_eq!(copied_content, "body{margin:0;padding:0;}");
    }

    #[test]
    fn test_copy_js_file_with_minification() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");
        fs::create_dir_all(&source_dir).unwrap();

        // Create a JS file with unnecessary whitespace and comments
        let js_content = "function   test(  ) {   // A comment\n    return   42;   }";
        let source_file = source_dir.join("script.js");
        fs::write(&source_file, js_content).unwrap();

        // Copy with versioning and minification
        let result =
            copy_file_with_versioning(source_file.to_str().unwrap(), dest_dir.to_str().unwrap());

        assert!(result.is_ok());
        let new_filename = result.unwrap();

        // Verify the file was created with a hash in the name
        assert!(new_filename.starts_with("script-"));
        assert!(new_filename.ends_with(".js"));

        // Read the copied file and verify it's minified
        let copied_file = dest_dir.join(&new_filename);
        let copied_content = fs::read_to_string(copied_file).unwrap();
        assert_eq!(copied_content, "function test(){return 42;}");
    }

    #[test]
    fn test_copy_non_minifiable_file() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");
        fs::create_dir_all(&source_dir).unwrap();

        // Create a text file (no minification should occur)
        let txt_content = "This is a regular text file with   spaces.";
        let source_file = source_dir.join("readme.txt");
        fs::write(&source_file, txt_content).unwrap();

        // Copy with versioning (no minification)
        let result =
            copy_file_with_versioning(source_file.to_str().unwrap(), dest_dir.to_str().unwrap());

        assert!(result.is_ok());
        let new_filename = result.unwrap();

        // Verify the file was created with a hash in the name
        assert!(new_filename.starts_with("readme-"));
        assert!(new_filename.ends_with(".txt"));

        // Read the copied file and verify it's unchanged
        let copied_file = dest_dir.join(&new_filename);
        let copied_content = fs::read_to_string(copied_file).unwrap();
        assert_eq!(copied_content, txt_content);
    }

    #[test]
    fn test_file_hashing_consistency() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");
        fs::create_dir_all(&source_dir).unwrap();

        // Create a CSS file
        let css_content = "body { margin: 0; }";
        let source_file = source_dir.join("test.css");
        fs::write(&source_file, css_content).unwrap();

        // Copy the same file twice
        let result1 =
            copy_file_with_versioning(source_file.to_str().unwrap(), dest_dir.to_str().unwrap());
        let result2 =
            copy_file_with_versioning(source_file.to_str().unwrap(), dest_dir.to_str().unwrap());

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        // The filenames should be identical (same hash for same content)
        assert_eq!(result1.unwrap(), result2.unwrap());
    }

    #[test]
    fn test_case_insensitive_extension_handling() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");
        fs::create_dir_all(&source_dir).unwrap();

        // Create files with uppercase extensions
        let css_content = "body   {   margin:   0;   }";
        let js_content = "function   test(  ) {   return   42;   }";

        let css_file = source_dir.join("style.CSS");
        let js_file = source_dir.join("script.JS");

        fs::write(&css_file, css_content).unwrap();
        fs::write(&js_file, js_content).unwrap();

        // Copy both files
        let css_result =
            copy_file_with_versioning(css_file.to_str().unwrap(), dest_dir.to_str().unwrap());
        let js_result =
            copy_file_with_versioning(js_file.to_str().unwrap(), dest_dir.to_str().unwrap());

        assert!(css_result.is_ok());
        assert!(js_result.is_ok());

        // Verify both files are minified despite uppercase extensions
        let css_filename = css_result.unwrap();
        let js_filename = js_result.unwrap();

        let css_content_result = fs::read_to_string(dest_dir.join(&css_filename)).unwrap();
        let js_content_result = fs::read_to_string(dest_dir.join(&js_filename)).unwrap();

        assert_eq!(css_content_result, "body{margin:0;}");
        assert_eq!(js_content_result, "function test(){return 42;}");
    }
}
