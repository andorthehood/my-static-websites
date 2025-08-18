use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::Hasher;
use std::io::{self, Read, Write};
use std::path::Path;

use crate::converters::scss::scss_to_css_with_inline_imports;
use crate::converters::typescript::strip_typescript_types;
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

    // Check file extension to determine if processing is needed
    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();

    // Process contents and decide output extension (for TS -> JS)
    let (processed_contents, output_extension) = match extension.to_lowercase().as_str() {
        "css" => {
            let css_string = String::from_utf8_lossy(&contents);
            let minified_css = minify_css(&css_string);
            (minified_css.into_bytes(), "css")
        }
        "scss" => {
            let inlined = scss_to_css_with_inline_imports(source_path)?;
            let minified_css = minify_css(&inlined);
            (minified_css.into_bytes(), "css")
        }
        "js" => {
            let js_string = String::from_utf8_lossy(&contents);
            let minified_js = minify_js(&js_string);
            (minified_js.into_bytes(), "js")
        }
        "ts" => {
            let ts_string = String::from_utf8_lossy(&contents);
            let stripped = strip_typescript_types(&ts_string);
            let minified_js = minify_js(&stripped);
            (minified_js.into_bytes(), "js")
        }
        _ => (contents, extension),
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

    // If original extension is ts, use js for output
    let new_file_name = format!("{file_stem}-{hash:x}.{output_extension}");

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
        let ts_content = "const el: HTMLElement | null = document.querySelector<HTMLElement>('a');";

        let css_file = source_dir.join("style.CSS");
        let js_file = source_dir.join("script.JS");
        let ts_file = source_dir.join("router.TS");

        fs::write(&css_file, css_content).unwrap();
        fs::write(&js_file, js_content).unwrap();
        fs::write(&ts_file, ts_content).unwrap();

        // Copy all files
        let css_result =
            copy_file_with_versioning(css_file.to_str().unwrap(), dest_dir.to_str().unwrap());
        let js_result =
            copy_file_with_versioning(js_file.to_str().unwrap(), dest_dir.to_str().unwrap());
        let ts_result =
            copy_file_with_versioning(ts_file.to_str().unwrap(), dest_dir.to_str().unwrap());

        assert!(css_result.is_ok());
        assert!(js_result.is_ok());
        assert!(ts_result.is_ok());

        // Verify outputs
        let css_filename = css_result.unwrap();
        let js_filename = js_result.unwrap();
        let ts_filename = ts_result.unwrap();

        assert!(css_filename.ends_with(".css"));
        assert!(js_filename.ends_with(".js"));
        assert!(ts_filename.ends_with(".js"));

        // Verify TS to JS transformation removed type patterns
        let ts_content_result = fs::read_to_string(dest_dir.join(&ts_filename)).unwrap();
        assert!(ts_content_result.contains("const el=document.querySelector('a')"));
    }

    #[test]
    fn test_copy_ts_file_with_type_stripping_and_minification() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");
        fs::create_dir_all(&source_dir).unwrap();

        let ts_content = r"
interface X { a: string }
const links: HTMLAnchorElement[] = [];
function f(x: number): Promise<void> { return new Promise<void>((resolve)=>resolve()); }
const a = document.querySelector<HTMLElement>('a');
const b = (a as HTMLElement)!;
        ";
        let source_file = source_dir.join("router.ts");
        fs::write(&source_file, ts_content).unwrap();

        let result =
            copy_file_with_versioning(source_file.to_str().unwrap(), dest_dir.to_str().unwrap());
        assert!(result.is_ok());
        let new_filename = result.unwrap();
        assert!(new_filename.starts_with("router-"));
        assert!(new_filename.ends_with(".js"));

        let copied = fs::read_to_string(dest_dir.join(&new_filename)).unwrap();
        // Interface should be removed, types stripped, and content minified
        assert!(!copied.contains("interface X"));
        assert!(!copied.contains("HTMLAnchorElement"));
        assert!(!copied.contains("Promise<void>"));
        assert!(!copied.contains("<HTMLElement>"));
        assert!(!copied.contains(" as "));
        assert!(!copied.contains("!;"));
    }

    #[test]
    fn test_copy_scss_file_with_inlining_and_minification() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");
        let partials_dir = source_dir.join("partials");
        fs::create_dir_all(&partials_dir).unwrap();

        // Create SCSS files
        fs::write(partials_dir.join("_button.scss"), ".btn { color: red; }\n").unwrap();
        fs::write(
            source_dir.join("typography.scss"),
            "h1 { font-weight: 700; }\n",
        )
        .unwrap();
        fs::write(
            source_dir.join("main.scss"),
            "@use './partials/button';\n@import './typography.scss';\nbody { margin: 0; }\n",
        )
        .unwrap();

        // Copy with versioning and conversion
        let result = copy_file_with_versioning(
            source_dir.join("main.scss").to_str().unwrap(),
            dest_dir.to_str().unwrap(),
        );

        assert!(result.is_ok());
        let new_filename = result.unwrap();
        assert!(new_filename.starts_with("main-"));
        assert!(new_filename.ends_with(".css"));

        let copied = fs::read_to_string(dest_dir.join(&new_filename)).unwrap();
        assert!(copied.contains(".btn{color:red;}"));
        assert!(copied.contains("h1{font-weight:700;}"));
        assert!(copied.contains("body{margin:0;}"));
        assert!(!copied.contains("@use"));
        assert!(!copied.contains("@import"));
    }

    #[test]
    fn test_copy_ts_preserves_url_in_string_literal() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");
        fs::create_dir_all(&source_dir).unwrap();

        let ts_content = r#"
(function(){
	setInterval(function(){
		const el=document.getElementById('clippy-gif');
		el.src='https://static.llllllllllll.com/andor/assets/clippy/swaying.gif?c=' + Date.now();
	},8000);
})();
		"#;
        let source_file = source_dir.join("url.ts");
        fs::write(&source_file, ts_content).unwrap();

        let result =
            copy_file_with_versioning(source_file.to_str().unwrap(), dest_dir.to_str().unwrap());
        assert!(result.is_ok());
        let new_filename = result.unwrap();
        assert!(new_filename.ends_with(".js"));

        let copied = fs::read_to_string(dest_dir.join(&new_filename)).unwrap();
        assert!(
            copied.contains("https://static.llllllllllll.com/andor/assets/clippy/swaying.gif?c=")
        );
    }
}
