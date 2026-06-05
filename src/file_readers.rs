use crate::config::SiteConfig;
use crate::parsers::parse_content_with_front_matter;
use crate::types::{ContentCollection, ContentItem};
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

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
    load_and_parse_files_recursive(path, path, &mut results)?;

    results.sort_by(|a: &ContentItem, b| b["slug"].cmp(&a["slug"]));

    Ok(results)
}

fn load_and_parse_files_recursive(
    root_path: &Path,
    current_path: &Path,
    results: &mut ContentCollection,
) -> Result<()> {
    for entry in fs::read_dir(current_path).map_err(|e| {
        Error::new(
            e.kind(),
            format!("Failed to read directory '{}': {e}", current_path.display()),
        )
    })? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            load_and_parse_files_recursive(root_path, &path, results)?;
        } else if is_supported_content_file(&path) {
            let mut parsed_content = load_and_parse_file_with_front_matter(&path)?;
            let slug = slug_from_relative_path(root_path, &path)?;
            parsed_content.insert("slug".to_string(), slug);
            results.push(parsed_content);
        }
    }

    Ok(())
}

fn is_supported_content_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|extension| extension == "md" || extension == "liquid" || extension == "html")
}

fn slug_from_relative_path(root_path: &Path, file_path: &Path) -> Result<String> {
    let relative_path = file_path.strip_prefix(root_path).map_err(|e| {
        Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Failed to make '{}' relative to '{}': {e}",
                file_path.display(),
                root_path.display()
            ),
        )
    })?;

    let slug_path = path_without_content_extension(relative_path);
    Ok(slug_path
        .iter()
        .map(|component| component.to_string_lossy())
        .collect::<Vec<_>>()
        .join("/"))
}

fn path_without_content_extension(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    let slug_name = if let Some(name) = file_name.strip_suffix(".liquid") {
        if let Some((name_without_extension, _)) = name.rsplit_once('.') {
            name_without_extension
        } else {
            name
        }
    } else if let Some(name) = file_name.strip_suffix(".html") {
        name
    } else if let Some(name) = file_name.strip_suffix(".md") {
        name
    } else {
        file_name
    };

    path.with_file_name(slug_name)
}

pub fn load_site_config(site_name: &str, config: &SiteConfig) -> Result<ContentItem> {
    let config_path_str = format!(
        "{}/{site_name}/{}",
        config.sites_base_dir, config.config_file
    );
    let config_path = Path::new(&config_path_str);
    if config_path.exists() {
        load_and_parse_file_with_front_matter(config_path)
    } else {
        // Return default configuration if no config file exists
        let mut default_config = ContentItem::new();
        default_config.insert("title".to_string(), String::new());
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

    #[test]
    fn test_directory_loader_recurses_into_nested_pages() {
        let dir = tempdir().unwrap();
        let pages_path = dir.path().join("pages");
        let lists_path = pages_path.join("lists");
        fs::create_dir_all(&lists_path).unwrap();

        fs::write(
            pages_path.join("about.html"),
            "---\ntitle: About\n---\nAbout body",
        )
        .unwrap();
        fs::write(
            lists_path.join("games.html"),
            "---\ntitle: Games\n---\nGames body",
        )
        .unwrap();

        let parsed =
            load_and_parse_files_with_front_matter_in_directory(pages_path.to_str().unwrap())
                .unwrap();

        let slugs = parsed
            .iter()
            .map(|item| item.get("slug").unwrap().as_str())
            .collect::<Vec<_>>();

        assert!(slugs.contains(&"about"));
        assert!(slugs.contains(&"lists/games"));
    }

    #[test]
    fn test_directory_loader_preserves_nested_md_liquid_slug() {
        let dir = tempdir().unwrap();
        let pages_path = dir.path().join("pages");
        let cv_path = pages_path.join("profile");
        fs::create_dir_all(&cv_path).unwrap();
        fs::write(
            cv_path.join("resume.md.liquid"),
            "---\ntitle: CV\n---\nCV body",
        )
        .unwrap();

        let parsed =
            load_and_parse_files_with_front_matter_in_directory(pages_path.to_str().unwrap())
                .unwrap();

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].get("slug"), Some(&"profile/resume".to_string()));
    }
}
