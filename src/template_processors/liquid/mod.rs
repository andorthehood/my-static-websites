/// Liquid template processing module
///
/// This module provides functionality for processing Liquid-style templates,
/// including conditional tags and includes.
mod _if;
mod parse_include_tag;
mod process_includes;
mod processor;

pub use _if::process_liquid_conditional_tags;
pub use process_includes::process_liquid_includes;
pub use processor::process_liquid_tags;
