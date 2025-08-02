/// Liquid template processing module
///
/// This module provides functionality for processing Liquid-style templates,
/// including conditional tags, includes, and variable substitution.
mod _if;
mod nested_access;
mod parse_include_tag;
mod process_includes;
mod processor;
mod remove;
mod replace_variables;
mod validation;

pub use _if::process_liquid_conditional_tags;
pub use processor::process_liquid_tags;
pub use remove::remove_liquid_variables;
pub use replace_variables::replace_template_variables;
