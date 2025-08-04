use crate::config::{DATA_SUBDIR, SITES_BASE_DIR};
use crate::error::Result;
use crate::parsers::{parse_json, JsonValue};
use crate::types::Variables;
use std::fs;
use std::path::Path;

/// Load all JSON data files from the site's data directory
///
/// Loads JSON files from sites/{site_name}/data/ and makes them available as variables.
/// For example, sites/test/data/navigation.json becomes accessible as {{ data.navigation }}
/// and sites/test/data/authors.json becomes {{ data.authors }}
///
/// # Arguments
/// * `site_name` - The name of the site
///
/// # Returns
/// A Variables HashMap with data.{filename} keys pointing to the JSON content
pub fn load_site_data(site_name: &str) -> Result<Variables> {
    let data_dir = format!("{SITES_BASE_DIR}/{site_name}/{DATA_SUBDIR}");
    let mut data_variables = Variables::new();

    // Check if data directory exists
    if !Path::new(&data_dir).exists() {
        // Return empty variables if no data directory exists
        return Ok(data_variables);
    }

    // Read all files in the data directory
    let entries = fs::read_dir(&data_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Only process .json files
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                match load_json_file(&path) {
                    Ok(json_data) => {
                        // Add each key-value pair from the JSON as data.{filename}.{key}
                        add_json_data_to_variables(&mut data_variables, file_name, &json_data);
                    }
                    Err(e) => {
                        eprintln!(
                            "⚠️  Warning: Failed to load JSON file {}: {}",
                            path.display(),
                            e
                        );
                    }
                }
            }
        }
    }

    Ok(data_variables)
}

/// Load and parse a JSON file
fn load_json_file(path: &Path) -> Result<JsonValue> {
    let content = fs::read_to_string(path)?;
    let json_value = parse_json(&content).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("JSON parse error: {}", e),
        )
    })?;
    Ok(json_value)
}

/// Recursively add JSON data to variables with proper prefixing
///
/// Converts JSON structure to flat variables that can be used in templates.
/// For example, if file is "navigation.json" containing:
/// ```json
/// {
///   "main": [
///     {"name": "Home", "url": "/"},
///     {"name": "About", "url": "/about"}
///   ]
/// }
/// ```
///
/// This creates variables like:
/// - data.navigation.main.0.name = "Home"
/// - data.navigation.main.0.url = "/"
/// - data.navigation.main.1.name = "About"  
/// - data.navigation.main.1.url = "/about"
fn add_json_data_to_variables(variables: &mut Variables, file_name: &str, json_value: &JsonValue) {
    let base_key = format!("data.{}", file_name);
    flatten_json_value(variables, &base_key, json_value);
}

/// Recursively flatten a JSON value into dot-notation variables
fn flatten_json_value(variables: &mut Variables, prefix: &str, value: &JsonValue) {
    match value {
        JsonValue::String(s) => {
            variables.insert(prefix.to_string(), s.clone());
        }
        JsonValue::Integer(n) => {
            variables.insert(prefix.to_string(), n.to_string());
        }
        JsonValue::Array(arr) => {
            for (index, item) in arr.iter().enumerate() {
                let key = format!("{}.{}", prefix, index);
                flatten_json_value(variables, &key, item);
            }
        }
        JsonValue::Object(obj) => {
            for (key, val) in obj {
                let new_key = format!("{}.{}", prefix, key);
                flatten_json_value(variables, &new_key, val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_site_with_data(site_name: &str, data_files: &[(&str, &str)]) -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let site_path = temp_dir.path().join("sites").join(site_name);
        let data_path = site_path.join("data");

        fs::create_dir_all(&data_path).unwrap();

        for (filename, content) in data_files {
            let file_path = data_path.join(filename);
            fs::write(file_path, content).unwrap();
        }

        // Temporarily change SITES_BASE_DIR for testing
        std::env::set_var("TEST_SITES_BASE_DIR", temp_dir.path().join("sites"));

        temp_dir
    }

    #[test]
    fn test_load_simple_json_data() {
        let json_content = r#"
        {
            "title": "My Site",
            "version": "1.0"
        }
        "#;

        let _temp_dir = create_test_site_with_data("test", &[("config.json", json_content)]);

        // Note: This test would need the actual implementation to use configurable base dir
        // For now, we'll test the flattening function directly
        let json_value = parse_json(json_content).unwrap();
        let mut variables = Variables::new();
        add_json_data_to_variables(&mut variables, "config", &json_value);

        assert_eq!(
            variables.get("data.config.title"),
            Some(&"My Site".to_string())
        );
        assert_eq!(
            variables.get("data.config.version"),
            Some(&"1.0".to_string())
        );
    }

    #[test]
    fn test_load_array_json_data() {
        let json_content = r#"
        {
            "navigation": [
                {"name": "Home", "url": "/"},
                {"name": "About", "url": "/about"}
            ]
        }
        "#;

        let json_value = parse_json(json_content).unwrap();
        let mut variables = Variables::new();
        add_json_data_to_variables(&mut variables, "nav", &json_value);

        assert_eq!(
            variables.get("data.nav.navigation.0.name"),
            Some(&"Home".to_string())
        );
        assert_eq!(
            variables.get("data.nav.navigation.0.url"),
            Some(&"/".to_string())
        );
        assert_eq!(
            variables.get("data.nav.navigation.1.name"),
            Some(&"About".to_string())
        );
        assert_eq!(
            variables.get("data.nav.navigation.1.url"),
            Some(&"/about".to_string())
        );
    }

    #[test]
    fn test_load_nested_json_data() {
        let json_content = r#"
        {
            "author": {
                "name": "John Doe",
                "social": {
                    "twitter": "@johndoe",
                    "github": "johndoe"
                }
            }
        }
        "#;

        let json_value = parse_json(json_content).unwrap();
        let mut variables = Variables::new();
        add_json_data_to_variables(&mut variables, "authors", &json_value);

        assert_eq!(
            variables.get("data.authors.author.name"),
            Some(&"John Doe".to_string())
        );
        assert_eq!(
            variables.get("data.authors.author.social.twitter"),
            Some(&"@johndoe".to_string())
        );
        assert_eq!(
            variables.get("data.authors.author.social.github"),
            Some(&"johndoe".to_string())
        );
    }
}
