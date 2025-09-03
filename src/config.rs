//! Application-wide configuration values.

use std::env;

/// Site configuration structure containing all configurable options
#[derive(Debug, Clone)]
pub struct SiteConfig {
    /// Output directory for generated site
    pub output_dir: String,
    /// Base directory for sites
    pub sites_base_dir: String,
    /// Posts subdirectory name
    pub posts_subdir: String,
    /// Pages subdirectory name
    pub pages_subdir: String,
    /// Includes subdirectory name
    pub includes_subdir: String,
    /// Layouts subdirectory name
    pub layouts_subdir: String,
    /// Assets subdirectory name
    pub assets_subdir: String,
    /// Data subdirectory name
    pub data_subdir: String,
    /// Main layout file name
    pub main_layout: String,
    /// Configuration file name
    pub config_file: String,
    /// Default posts per page for pagination
    pub default_posts_per_page: usize,
    /// Server host
    pub server_host: String,
    /// Server port
    pub server_port: u16,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            output_dir: "out".to_string(),
            sites_base_dir: "./sites".to_string(),
            posts_subdir: "posts".to_string(),
            pages_subdir: "pages".to_string(),
            includes_subdir: "includes".to_string(),
            layouts_subdir: "layouts".to_string(),
            assets_subdir: "assets".to_string(),
            data_subdir: "data".to_string(),
            main_layout: "main.html".to_string(),
            config_file: "config.md".to_string(),
            default_posts_per_page: 5,
            server_host: "localhost".to_string(),
            server_port: 2030,
        }
    }
}

impl SiteConfig {
    /// Create a new configuration with defaults
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from environment variables, falling back to defaults
    pub fn from_environment() -> Self {
        let mut config = Self::default();

        if let Ok(value) = env::var("LEPKEFING_OUTPUT_DIR") {
            config.output_dir = value;
        }
        if let Ok(value) = env::var("LEPKEFING_SITES_BASE_DIR") {
            config.sites_base_dir = value;
        }
        if let Ok(value) = env::var("LEPKEFING_POSTS_SUBDIR") {
            config.posts_subdir = value;
        }
        if let Ok(value) = env::var("LEPKEFING_PAGES_SUBDIR") {
            config.pages_subdir = value;
        }
        if let Ok(value) = env::var("LEPKEFING_INCLUDES_SUBDIR") {
            config.includes_subdir = value;
        }
        if let Ok(value) = env::var("LEPKEFING_LAYOUTS_SUBDIR") {
            config.layouts_subdir = value;
        }
        if let Ok(value) = env::var("LEPKEFING_ASSETS_SUBDIR") {
            config.assets_subdir = value;
        }
        if let Ok(value) = env::var("LEPKEFING_DATA_SUBDIR") {
            config.data_subdir = value;
        }
        if let Ok(value) = env::var("LEPKEFING_MAIN_LAYOUT") {
            config.main_layout = value;
        }
        if let Ok(value) = env::var("LEPKEFING_CONFIG_FILE") {
            config.config_file = value;
        }
        if let Ok(value) = env::var("LEPKEFING_DEFAULT_POSTS_PER_PAGE") {
            if let Ok(parsed) = value.parse::<usize>() {
                config.default_posts_per_page = parsed;
            }
        }
        if let Ok(value) = env::var("LEPKEFING_SERVER_HOST") {
            config.server_host = value;
        }
        if let Ok(value) = env::var("LEPKEFING_SERVER_PORT") {
            if let Ok(parsed) = value.parse::<u16>() {
                config.server_port = parsed;
            }
        }

        config
    }

    /// Validate the configuration values
    pub fn validate(&self) -> Result<(), String> {
        if self.output_dir.is_empty() {
            return Err("Output directory cannot be empty".to_string());
        }
        if self.sites_base_dir.is_empty() {
            return Err("Sites base directory cannot be empty".to_string());
        }
        if self.posts_subdir.is_empty() {
            return Err("Posts subdirectory cannot be empty".to_string());
        }
        if self.pages_subdir.is_empty() {
            return Err("Pages subdirectory cannot be empty".to_string());
        }
        if self.includes_subdir.is_empty() {
            return Err("Includes subdirectory cannot be empty".to_string());
        }
        if self.layouts_subdir.is_empty() {
            return Err("Layouts subdirectory cannot be empty".to_string());
        }
        if self.assets_subdir.is_empty() {
            return Err("Assets subdirectory cannot be empty".to_string());
        }
        if self.data_subdir.is_empty() {
            return Err("Data subdirectory cannot be empty".to_string());
        }
        if self.main_layout.is_empty() {
            return Err("Main layout cannot be empty".to_string());
        }
        if self.config_file.is_empty() {
            return Err("Configuration file cannot be empty".to_string());
        }
        if self.default_posts_per_page == 0 {
            return Err("Default posts per page must be greater than 0".to_string());
        }
        if self.server_host.is_empty() {
            return Err("Server host cannot be empty".to_string());
        }
        if self.server_port == 0 {
            return Err("Server port must be greater than 0".to_string());
        }

        Ok(())
    }
}

// Keep the old constants for backward compatibility during transition
#[allow(dead_code)]
pub const OUTPUT_DIR: &str = "out";
pub const SITES_BASE_DIR: &str = "./sites";
#[allow(dead_code)]
pub const POSTS_SUBDIR: &str = "posts";
#[allow(dead_code)]
pub const PAGES_SUBDIR: &str = "pages";
#[allow(dead_code)]
pub const INCLUDES_SUBDIR: &str = "includes";
pub const LAYOUTS_SUBDIR: &str = "layouts";
#[allow(dead_code)]
pub const ASSETS_SUBDIR: &str = "assets";
#[allow(dead_code)]
pub const DATA_SUBDIR: &str = "data";
#[allow(dead_code)]
pub const MAIN_LAYOUT: &str = "main.html";
#[allow(dead_code)]
pub const CONFIG_FILE: &str = "config.md";
#[allow(dead_code)]
pub const DEFAULT_POSTS_PER_PAGE: usize = 5;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_config_default() {
        let config = SiteConfig::default();
        assert_eq!(config.output_dir, "out");
        assert_eq!(config.sites_base_dir, "./sites");
        assert_eq!(config.posts_subdir, "posts");
        assert_eq!(config.pages_subdir, "pages");
        assert_eq!(config.includes_subdir, "includes");
        assert_eq!(config.layouts_subdir, "layouts");
        assert_eq!(config.assets_subdir, "assets");
        assert_eq!(config.data_subdir, "data");
        assert_eq!(config.main_layout, "main.html");
        assert_eq!(config.config_file, "config.md");
        assert_eq!(config.default_posts_per_page, 5);
        assert_eq!(config.server_host, "localhost");
        assert_eq!(config.server_port, 2030);
    }

    #[test]
    fn test_site_config_new() {
        let config = SiteConfig::new();
        assert_eq!(config.output_dir, "out");
        assert_eq!(config.server_port, 2030);
    }

    #[test]
    fn test_site_config_validation_success() {
        let config = SiteConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_site_config_validation_empty_output_dir() {
        let mut config = SiteConfig::default();
        config.output_dir = String::new();
        assert!(config.validate().is_err());
        assert_eq!(
            config.validate().unwrap_err(),
            "Output directory cannot be empty"
        );
    }

    #[test]
    fn test_site_config_validation_zero_posts_per_page() {
        let mut config = SiteConfig::default();
        config.default_posts_per_page = 0;
        assert!(config.validate().is_err());
        assert_eq!(
            config.validate().unwrap_err(),
            "Default posts per page must be greater than 0"
        );
    }

    #[test]
    fn test_site_config_validation_zero_port() {
        let mut config = SiteConfig::default();
        config.server_port = 0;
        assert!(config.validate().is_err());
        assert_eq!(
            config.validate().unwrap_err(),
            "Server port must be greater than 0"
        );
    }

    #[test]
    fn test_site_config_from_environment_defaults() {
        // Test without any environment variables set - should use defaults
        let config = SiteConfig::from_environment();
        assert_eq!(config.output_dir, "out");
        assert_eq!(config.server_port, 2030);
    }
}
