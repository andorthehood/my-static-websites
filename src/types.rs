//! Common type aliases used throughout the application.

use std::collections::HashMap;

/// Represents a single content item (post or page) with front matter and metadata
pub type ContentItem = HashMap<String, String>;

/// Collection of content items (posts or pages)
pub type ContentCollection = Vec<ContentItem>;

/// Template includes and partials loaded from the includes directory
pub type TemplateIncludes = HashMap<String, String>;

/// Template variables used for rendering (site config, layout variables, etc.)
pub type Variables = HashMap<String, String>;
