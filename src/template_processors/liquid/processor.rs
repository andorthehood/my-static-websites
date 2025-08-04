use super::_if::process_liquid_conditional_tags;
use super::assign::process_liquid_assign_tags;
use super::for_loop::process_liquid_for_loops;
use super::process_includes::process_liquid_includes;
use crate::error::Result;
use std::collections::HashMap;

/// Process all Liquid tags in a template string
///
/// This is a convenience function that processes conditional tags, for loops, and includes
/// in the correct order.
///
/// # Arguments
/// * `template` - The template string to process
/// * `conditions` - List of condition names that should evaluate to true
/// * `templates` - Map of template names to their content for includes
/// * `variables` - Variables for for loop processing (to determine collection sizes)
///
/// # Returns
/// The processed template with conditionals, for loops, and includes evaluated
pub fn process_liquid_tags(
    template: &str,
    conditions: &[String],
    templates: &HashMap<String, String>,
    variables: &HashMap<String, String>,
) -> Result<String> {
    let processed_conditionals = process_liquid_conditional_tags(template, conditions);
    let processed_for_loops = process_liquid_for_loops(&processed_conditionals, variables)?;
    process_liquid_includes(&processed_for_loops, templates)
}

/// Process all Liquid tags in a template string, including assign tags
///
/// This function processes conditional tags, assign tags, for loops, and includes
/// in the correct order. Assign tags can modify the variables map.
///
/// # Arguments
/// * `template` - The template string to process
/// * `conditions` - List of condition names that should evaluate to true
/// * `templates` - Map of template names to their content for includes
/// * `variables` - Mutable variables map for assign tags and for loop processing
///
/// # Returns
/// The processed template with all liquid tags evaluated
pub fn process_liquid_tags_with_assigns(
    template: &str,
    conditions: &[String],
    templates: &HashMap<String, String>,
    variables: &mut HashMap<String, String>,
) -> Result<String> {
    let processed_conditionals = process_liquid_conditional_tags(template, conditions);
    let processed_assigns = process_liquid_assign_tags(&processed_conditionals, variables)?;
    let processed_for_loops = process_liquid_for_loops(&processed_assigns, variables)?;
    process_liquid_includes(&processed_for_loops, templates)
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
        let variables = HashMap::new();

        let input = "{% if show_greeting %}{% include header.liquid name:\"World\" %}{% endif %}";
        let result = process_liquid_tags(input, &conditions, &templates, &variables).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_process_liquid_tags_with_assigns() {
        let mut variables = HashMap::new();
        variables.insert("users.0.name".to_string(), "Alice".to_string());
        variables.insert("users.0.active".to_string(), "true".to_string());
        variables.insert("users.1.name".to_string(), "Bob".to_string());
        variables.insert("users.1.active".to_string(), "false".to_string());

        let templates = HashMap::new();
        let conditions = Vec::new();

        let input = "{% assign active_users = users | where: \"active\", \"true\" %}Found {{ active_users.0.name }}";
        let result =
            process_liquid_tags_with_assigns(input, &conditions, &templates, &mut variables)
                .unwrap();

        assert_eq!(result, "Found {{ active_users.0.name }}");
        assert_eq!(
            variables.get("active_users.0.name"),
            Some(&"Alice".to_string())
        );
    }
}
