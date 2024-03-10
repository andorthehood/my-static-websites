use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn write_html_to_file(path: &str, content: &str) -> io::Result<()> {
    let path = Path::new(path);

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}