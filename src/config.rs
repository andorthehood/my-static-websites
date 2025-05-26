use serde::Deserialize;
use std::fs;
use std::path::Path;
use crate::error::MyError; // Assuming MyError is in src/error.rs

#[derive(Deserialize, Debug)]
pub struct Config {
    pub posts_dir: String,
    pub pages_dir: String,
    pub includes_dir: String,
    pub layouts_dir: String,
    pub output_dir: String,
    pub style_path: String,
    pub static_dir: String,
    pub output_static_dir_name: String,
    pub content_image_dir_name: String,
    pub output_image_dir_name: String,
}

impl Config {
    pub fn load(file_path: &str) -> Result<Self, MyError> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(MyError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Configuration file not found: {}", file_path),
            )));
        }
        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents)
            .map_err(|e| MyError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse TOML configuration: {}", e),
            )))
    }
}

// Optional: Provide default values if config file is not found or some fields are missing
// This requires `#[serde(default)]` on the struct and `Default` trait implementation,
// or `#[serde(default = "path_to_default_fn")]` for specific fields.
// For simplicity, we'll make the config file mandatory for now.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        // Create a dummy config file for testing
        let dummy_config_content = r#"
            posts_dir = "_posts_test"
            pages_dir = "_pages_test"
            includes_dir = "_includes_test"
            layouts_dir = "_layouts_test"
            output_dir = "out_test"
            style_path = "style_test.css"
            static_dir = "static_test"
            output_static_dir_name = "static_out_test"
            content_image_dir_name = "images_content_test"
            output_image_dir_name = "images_out_test"
        "#;
        let test_config_path = "config.test.toml";
        fs::write(test_config_path, dummy_config_content).unwrap();

        let config = Config::load(test_config_path).expect("Failed to load test config");

        assert_eq!(config.posts_dir, "_posts_test");
        assert_eq!(config.pages_dir, "_pages_test");
        assert_eq!(config.output_dir, "out_test");
        assert_eq!(config.style_path, "style_test.css");
        assert_eq!(config.static_dir, "static_test");
        assert_eq!(config.output_static_dir_name, "static_out_test");

        // Clean up dummy file
        fs::remove_file(test_config_path).unwrap();
    }

    #[test]
    fn test_load_config_not_found() {
        match Config::load("non_existent_config.toml") {
            Err(MyError::Io(e)) => assert_eq!(e.kind(), std::io::ErrorKind::NotFound),
            _ => panic!("Expected NotFound error"),
        }
    }
}
