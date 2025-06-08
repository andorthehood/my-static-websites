/// Liquid template processing module
/// 
/// This module provides functionality for processing Liquid-style templates,
/// including conditional tags and includes.

mod _if;
mod include;
mod processor;

pub use _if::process_liquid_conditional_tags;
pub use include::process_liquid_includes;
pub use processor::process_liquid_tags;

// Re-export commonly used types
pub use std::collections::HashMap;
