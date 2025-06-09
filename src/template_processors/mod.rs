// Template processors module
// This module contains different template processing implementations

pub mod liquid;
pub mod markdown;
mod markdown_with_front_matter;

pub use markdown_with_front_matter::*;

// TODO: Add other template processors here
