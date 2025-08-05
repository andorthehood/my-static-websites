use super::_if::process_liquid_conditional_tags;
use super::assign::process_liquid_assign_tags;
use super::for_loop::process_liquid_for_loops;
use super::process_includes::process_liquid_includes;
use super::unless::process_liquid_unless_tags;
use crate::error::Result;
use std::collections::HashMap;

/// Process all Liquid tags in a template string, including assign tags
///
/// This function processes conditional tags, assign tags, for loops, unless tags, and includes
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
    // Unless tags are now processed during for loop expansion for forloop context
    // Still process any remaining unless tags that are outside for loops
    let processed_unless = process_liquid_unless_tags(&processed_for_loops, variables)?;
    process_liquid_includes(&processed_unless, templates)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_for_loop_with_unless_forloop_last() {
        let mut variables = HashMap::new();
        variables.insert("items.0".to_string(), "apple".to_string());
        variables.insert("items.1".to_string(), "banana".to_string());
        variables.insert("items.2".to_string(), "cherry".to_string());

        let templates = HashMap::new();
        let conditions = Vec::new();

        let input = "{% for item in items %}{{ item }}{% unless forloop.last %}, {% endunless %}{% endfor %}";
        let result =
            process_liquid_tags_with_assigns(input, &conditions, &templates, &mut variables)
                .unwrap();

        // Should have commas for first two items but not the last
        let expected = "{{ items.0 }}, {{ items.1 }}, {{ items.2 }}";
        assert_eq!(result, expected);
    }
}
