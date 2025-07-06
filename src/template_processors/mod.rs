// Template processors module
// This module contains different template processing implementations

pub mod handlebars;
pub mod liquid;
pub mod markdown;
mod processor;

pub use processor::{process_content, process_template_tags};
