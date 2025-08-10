use crate::config::{CONFIG_FILE, SITES_BASE_DIR};
use crate::parsers::parse_content_with_front_matter;
use crate::types::{ContentCollection, ContentItem};
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

pub fn load_and_parse_file_with_front_matter(file_path: &Path) -> Result<ContentItem> {
    let content = fs::read_to_string(file_path).map_err(|e| {
        Error::new(
            e.kind(),
            format!(
                "Failed to read file '{file_path}': {e}",
                file_path = file_path.display()
            ),
        )
    })?;
    let mut parsed_content = parse_content_with_front_matter(&content);

    if let Some(file_stem) = file_path.file_stem().and_then(|s| s.to_str()) {
        // For files like "resume.md.liquid", the extension is "liquid" and file_stem is "resume.md".
        // We want the slug to be just "resume" in that case.
        let slug = if file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext == "liquid")
        {
            if let Some(dot_index) = file_stem.rfind('.') {
                &file_stem[..dot_index]
            } else {
                file_stem
            }
        } else {
            file_stem
        };
        parsed_content.insert("slug".to_string(), slug.to_string());
    }

    // Add file type to content for rendering pipeline
    if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
        parsed_content.insert("file_type".to_string(), extension.to_string());
    }

    // Also store the full source file name (with extensions) for output extension inference
    if let Some(source_name) = file_path.file_name().and_then(|s| s.to_str()) {
        parsed_content.insert("source_file_name".to_string(), source_name.to_string());
    }

    Ok(parsed_content)
}

pub fn load_and_parse_files_with_front_matter_in_directory(
    dir_path: &str,
) -> Result<ContentCollection> {
    let path = Path::new(dir_path);

    if !path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Directory '{dir_path}' does not exist. Make sure your site has the required directory structure."),
        ));
    }

    let mut results = Vec::new();

    for entry in fs::read_dir(path).map_err(|e| {
        Error::new(
            e.kind(),
            format!("Failed to read directory '{dir_path}': {e}"),
        )
    })? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                if extension == "md" || extension == "liquid" {
                    let parsed_content = load_and_parse_file_with_front_matter(&path)?;
                    results.push(parsed_content);
                }
            }
        }
    }

    results.sort_by(|a: &ContentItem, b| b["slug"].cmp(&a["slug"]));

    Ok(results)
}

pub fn load_site_config(site_name: &str) -> Result<ContentItem> {
    let config_path_str = format!("{SITES_BASE_DIR}/{site_name}/{CONFIG_FILE}");
    let config_path = Path::new(&config_path_str);
    if config_path.exists() {
        load_and_parse_file_with_front_matter(config_path)
    } else {
        // Return default configuration if no config file exists
        let mut default_config = ContentItem::new();
        default_config.insert("title".to_string(), String::new());
        default_config.insert("index_filename".to_string(), "index.html".to_string());
        Ok(default_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_and_parse_files_with_front_matter_in_directory_missing() {
        let result = load_and_parse_files_with_front_matter_in_directory("/definitely/not/exist");
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_load_and_parse_file_with_front_matter_unreadable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("secret.md");

        // Create file then remove read permissions to simulate unreadable
        {
            let mut f = File::create(&file_path).unwrap();
            writeln!(f, "---\ntitle: test\n---\nbody").unwrap();
        }
        // On Unix, set mode to write-only (no read)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o222);
            fs::set_permissions(&file_path, perms).unwrap();
        }

        let result = load_and_parse_file_with_front_matter(&file_path);
        assert!(result.is_err());

        // Cleanup to allow tempdir drop
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o666);
            fs::set_permissions(&file_path, perms).unwrap();
        }
    }

    #[test]
    fn test_slug_extraction_from_md_liquid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("resume.md.liquid");
        let content = "---\ntitle: CV\n---\ncontent";
        fs::write(&file_path, content).unwrap();

        let parsed = load_and_parse_file_with_front_matter(&file_path).unwrap();
        assert_eq!(parsed.get("slug"), Some(&"resume".to_string()));
        assert_eq!(parsed.get("file_type"), Some(&"liquid".to_string()));
        assert_eq!(
            parsed.get("source_file_name"),
            Some(&"resume.md.liquid".to_string())
        );
    }
}
