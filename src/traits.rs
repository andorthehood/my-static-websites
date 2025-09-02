//! Trait definitions for abstract interfaces
//!
//! This module defines trait-based interfaces for the core processing components
//! to enable dependency injection, easier testing with mocks, and cleaner
//! separation of concerns.

use crate::error::Result;
use crate::types::{ContentItem, TemplateIncludes, Variables};
use std::collections::HashMap;
use std::path::Path;

/// Trait for template processing operations
///
/// Defines the interface for processing templates with support for liquid tags,
/// markdown conversion, and variable substitution.
pub trait TemplateProcessor {
    /// Process template tags in input string with optional features
    ///
    /// # Arguments
    /// * `input` - The input string containing template tags
    /// * `variables` - Variables for template processing
    /// * `includes` - Optional liquid includes for {% include %} tags
    /// * `content_item` - Optional content metadata for markdown processing and additional variables
    ///
    /// # Returns
    /// * `Result<String>` - The processed template or an error if processing fails
    fn process_template_tags(
        &self,
        input: &str,
        variables: &Variables,
        includes: Option<&TemplateIncludes>,
        content_item: Option<&ContentItem>,
    ) -> Result<String>;
}

/// Trait for asset conversion operations
///
/// Defines the interface for converting assets from one format to another,
/// such as TypeScript to JavaScript or SCSS to CSS.
pub trait AssetConverter {
    /// Convert asset content from input format to output format
    ///
    /// # Arguments
    /// * `input` - The input content to convert
    /// * `source_path` - Optional path to the source file for context
    ///
    /// # Returns
    /// * `Result<String>` - The converted content or an error if conversion fails
    fn convert(&self, input: &str, source_path: Option<&Path>) -> Result<String>;

    /// Get the file extensions this converter supports
    ///
    /// # Returns
    /// * `Vec<&str>` - List of supported file extensions (e.g., ["ts", "tsx"])
    fn supported_extensions(&self) -> Vec<&str>;

    /// Get the output file extension for converted files
    ///
    /// # Returns
    /// * `&str` - Output file extension (e.g., "js" for TypeScript converter)
    fn output_extension(&self) -> &str;
}

/// Trait for minification operations
///
/// Defines the interface for minifying content by removing unnecessary
/// whitespace, comments, and other optimizations while preserving functionality.
pub trait Minifier {
    /// Minify the input content
    ///
    /// # Arguments
    /// * `input` - The input content to minify
    ///
    /// # Returns
    /// * `String` - The minified content
    fn minify(&self, input: &str) -> String;

    /// Get the content type this minifier supports
    ///
    /// # Returns
    /// * `&str` - The content type (e.g., "html", "css", "js")
    fn content_type(&self) -> &str;
}

/// Trait for file processing operations
///
/// Defines a general interface for processing files, which can be implemented
/// by any component that needs to transform file content.
pub trait FileProcessor {
    /// Process a file's content
    ///
    /// # Arguments
    /// * `content` - The file content to process
    /// * `file_path` - Path to the file being processed
    /// * `context` - Optional processing context (variables, metadata, etc.)
    ///
    /// # Returns
    /// * `Result<String>` - The processed content or an error if processing fails
    fn process_file(
        &self,
        content: &str,
        file_path: &Path,
        context: Option<&HashMap<String, String>>,
    ) -> Result<String>;

    /// Check if this processor can handle the given file
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to check
    ///
    /// # Returns
    /// * `bool` - True if this processor can handle the file
    fn can_process(&self, file_path: &Path) -> bool;
}