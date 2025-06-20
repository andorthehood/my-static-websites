use super::_if::process_liquid_conditional_tags;
use super::process_includes::process_liquid_includes;
use crate::error::Result;
use std::collections::HashMap;

/// Process all Liquid tags in a template string
///
/// This is a convenience function that processes both conditional tags and includes
/// in a single pass.
///
/// # Arguments
/// * `template` - The template string to process
/// * `conditions` - List of condition names that should evaluate to true
/// * `templates` - Map of template names to their content for includes
///
/// # Returns
/// The processed template with both conditionals and includes evaluated
pub fn process_liquid_tags(
    template: &str,
    conditions: &[String],
    templates: &HashMap<String, String>,
) -> Result<String> {
    let processed_conditionals = process_liquid_conditional_tags(template, conditions);
    process_liquid_includes(&processed_conditionals, templates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_liquid_tags() {
        let mut templates = HashMap::new();
        templates.insert(
            "header.liquid".to_string(),
            "Hello, {{ name }}!".to_string(),
        );

        let conditions = vec!["show_greeting".to_string()];

        let input = "{% if show_greeting %}{% include header.liquid name:\"World\" %}{% endif %}";
        let result = process_liquid_tags(input, &conditions, &templates).unwrap();
        assert_eq!(result, "Hello, World!");
    }
}
