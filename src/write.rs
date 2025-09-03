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

pub fn write_json_to_file(
    path: &str,
    content: &str,
    title: Option<&str>,
    css: Option<&str>,
) -> io::Result<()> {
    let path = Path::new(path);

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    // Escape JSON strings properly
    let escaped_content = escape_json_string(content);
    let escaped_title = title.map_or_else(|| "null".to_string(), escape_json_string);
    let escaped_css = css.map_or_else(
        || "null".to_string(),
        |c| format!("\"{}\"", escape_json_string(c)),
    );

    let json_content = format!(
        "{{\n  \"content\": \"{}\",\n  \"title\": {},\n  \"css\": {}\n}}",
        escaped_content,
        if title.is_some() {
            format!("\"{escaped_title}\"")
        } else {
            "null".to_string()
        },
        escaped_css
    );

    let mut file = File::create(path)?;
    file.write_all(json_content.as_bytes())?;

    Ok(())
}

fn escape_json_string(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            c if c.is_control() => format!("\\u{:04x}", c as u32),
            c => c.to_string(),
        })
        .collect()
}
