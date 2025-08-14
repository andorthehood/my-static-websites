//! CSS minification module
//! 
//! This module provides CSS minification capabilities by removing unnecessary whitespace,
//! comments, and optimizing hex colors while preserving CSS functionality.
//! 
//! The module is organized into separate components:
//! - `comments`: CSS comment detection and removal
//! - `hex_colors`: Hex color optimization (with x86_64 assembly optimization)
//! - `minifier`: Main minification orchestration
//! - `strings`: String literal handling
//! - `whitespace`: Complex whitespace preservation rules

mod comments;
mod hex_colors;
mod minifier;
mod strings;
mod whitespace;

// Re-export the main minify function
pub use minifier::minify_css;