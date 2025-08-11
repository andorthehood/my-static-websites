use crate::minifier::html::minify_html;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn write_html_to_file(path: &str, content: &str) -> io::Result<()> {
    let path = Path::new(path);

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    // Check if this is an HTML file and minify if so
    let final_content = if path.extension().and_then(|ext| ext.to_str()) == Some("html") {
        minify_html(content)
    } else {
        content.to_string()
    };

    let mut file = File::create(path)?;
    file.write_all(final_content.as_bytes())?;

    Ok(())
}
