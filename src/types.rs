//! Common type aliases used throughout the application.

use std::collections::HashMap;

// Output directory constants
pub const OUTPUT_DIR: &str = "out";
pub const OUTPUT_POSTS_DIR: &str = "out/posts";

// Server configuration
pub const DEFAULT_SERVER_PORT: u16 = 2030;
pub const DEFAULT_SERVER_HOST: &str = "127.0.0.1";

// Pagination configuration
pub const DEFAULT_POSTS_PER_PAGE: usize = 5;

/// Represents a single content item (post or page) with front matter and metadata
pub type ContentItem = HashMap<String, String>;

/// Collection of content items (posts or pages)
pub type ContentCollection = Vec<ContentItem>;

/// Template includes and partials loaded from the includes directory
pub type TemplateIncludes = HashMap<String, String>;

/// Template variables used for rendering (site config, layout variables, etc.)
pub type Variables = HashMap<String, String>;

/// Posts organized by year for pagination and archives
pub type PostsByYear = HashMap<String, ContentCollection>;
