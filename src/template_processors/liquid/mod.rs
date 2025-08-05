/// Liquid template processing module
///
/// This module provides functionality for processing Liquid-style templates,
/// including conditional tags, includes, for loops, assign tags, unless tags, and variable substitution.
mod _if;
mod assign;
mod for_loop;
mod nested_access;
mod parse_include_tag;
mod process_includes;
mod processor;
mod remove;
mod replace_variables;
mod unless;
mod validation;

pub use _if::process_liquid_conditional_tags;
pub use assign::process_liquid_assign_tags;
pub use for_loop::process_liquid_for_loops;
pub use processor::process_liquid_tags_with_assigns;
pub use remove::remove_liquid_variables;
pub use replace_variables::replace_template_variables;
// unless::process_liquid_unless_tags is now used internally by for_loop processing
