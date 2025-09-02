// Template processors module
// This module contains different template processing implementations

pub mod liquid;
pub mod markdown;
mod processor;

pub use processor::DefaultTemplateProcessor;
