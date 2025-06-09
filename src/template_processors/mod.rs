// Template processors module
// This module contains different template processing implementations

pub mod liquid;
pub mod markdown;
pub mod handlebars;
mod markdown_with_front_matter;
mod processor;

pub use markdown_with_front_matter::*;
pub use processor::process_template_tags;

// TODO: Add other template processors here
