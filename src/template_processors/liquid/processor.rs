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
/// * `conditions` - List of condition names that should evaluate to true (deprecated)
/// * `templates` - Map of template names to their content for includes
/// * `variables` - Mutable variables map for assign tags and for loop processing
///
/// # Returns
/// The processed template with all liquid tags evaluated
pub fn process_liquid_tags_with_assigns(
    template: &str,
    _conditions: &[String],
    templates: &HashMap<String, String>,
    variables: &mut HashMap<String, String>,
) -> Result<String> {
    // Process assigns first so variables are available to subsequent steps
    let processed_assigns = process_liquid_assign_tags(template, variables)?;

    // Expand for loops next so that any item-scoped references are transformed
    let processed_for_loops = process_liquid_for_loops(&processed_assigns, variables)?;

    // Process unless tags after loop expansion to handle forloop context and any remaining unless tags
    let processed_unless = process_liquid_unless_tags(&processed_for_loops, variables)?;

    // Process if-conditionals after loop expansion using variables for truthiness
    let processed_conditionals = process_liquid_conditional_tags(&processed_unless, variables)?;

    // Finally, resolve includes
    process_liquid_includes(&processed_conditionals, templates)
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

    #[test]
    fn test_for_loop_with_if_on_item_variable() {
        let mut variables = HashMap::new();
        variables.insert("users.0.name".to_string(), "Alice".to_string());
        variables.insert("users.0.active".to_string(), "true".to_string());
        variables.insert("users.1.name".to_string(), "Bob".to_string());
        variables.insert("users.1.active".to_string(), "false".to_string());

        let templates = HashMap::new();
        let conditions = Vec::new();

        let input =
            "{% for user in users %}{% if user.active %}{{ user.name }} {% endif %}{% endfor %}";
        let result =
            process_liquid_tags_with_assigns(input, &conditions, &templates, &mut variables)
                .unwrap();

        // Only the active user should be included
        let expected = "{{ users.0.name }} ";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_truthy_semantics_outside_loops() {
        let mut variables = HashMap::new();
        variables.insert("empty_flag".to_string(), "".to_string());
        variables.insert("false_flag".to_string(), "false".to_string());
        variables.insert("other_flag".to_string(), "hello".to_string());
        variables.insert("zero_flag".to_string(), "0".to_string());

        let templates = HashMap::new();
        let conditions = Vec::new();

        let input = "{% if empty_flag %}A{% endif %}{% if false_flag %}B{% endif %}{% if missing_flag %}C{% endif %}{% if other_flag %}D{% endif %}{% if zero_flag %}E{% endif %}";
        let result =
            process_liquid_tags_with_assigns(input, &conditions, &templates, &mut variables)
                .unwrap();

        // Only other_flag and zero_flag are truthy
        let expected = "DE";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_truthy_semantics_inside_loops() {
        let mut variables = HashMap::new();
        variables.insert("users.0.name".to_string(), "Alice".to_string());
        variables.insert("users.0.flag".to_string(), "".to_string());
        variables.insert("users.1.name".to_string(), "Bob".to_string());
        variables.insert("users.1.flag".to_string(), "false".to_string());
        variables.insert("users.2.name".to_string(), "Carol".to_string());
        variables.insert("users.2.flag".to_string(), "yes".to_string());

        let templates = HashMap::new();
        let conditions = Vec::new();

        let input =
            "{% for user in users %}{% if user.flag %}{{ user.name }} {% endif %}{% endfor %}";
        let result =
            process_liquid_tags_with_assigns(input, &conditions, &templates, &mut variables)
                .unwrap();

        // Only Carol's flag is truthy
        let expected = "{{ users.2.name }} ";
        assert_eq!(result, expected);
    }
}
