//! CSS minification module
//!
//! This module provides CSS minification capabilities by removing unnecessary whitespace,
//! comments, and optimizing hex colors while preserving CSS functionality.
//!
//! The module is organized into separate components:
//! - `comments`: CSS comment detection and removal
//! - `hex_colors`: Hex color optimization (with `x86_64` assembly optimization)
//! - `minifier`: Main minification orchestration
//! - `strings`: String literal handling
//! - `whitespace`: Complex whitespace preservation rules

mod comments;
mod hex_colors;
mod minifier;
mod should_preserve_space;
mod strings;
mod whitespace;

use crate::traits::Minifier;

// Re-export the main minify function
pub use minifier::minify_css;

/// CSS minifier implementation
pub struct CssMinifier;

impl CssMinifier {
    /// Create a new CSS minifier
    pub fn new() -> Self {
        Self
    }
}

impl Default for CssMinifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Minifier for CssMinifier {
    fn minify(&self, input: &str) -> String {
        minify_css(input)
    }

    fn content_type(&self) -> &str {
        "css"
    }
}

#[cfg(test)]
mod trait_tests {
    use super::*;
    use crate::traits::Minifier;

    #[test]
    fn test_css_minifier_trait() {
        let minifier = CssMinifier::new();
        assert_eq!(minifier.content_type(), "css");

        let input = "body { margin: 0; padding: 0; }";
        let result = minifier.minify(input);
        
        // Basic minification should work
        assert!(result.len() <= input.len());
        assert!(result.contains("margin:0"));
    }
}
