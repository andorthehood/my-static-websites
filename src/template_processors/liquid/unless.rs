use super::utils::find_tag_block;
use crate::error::{Error, Result};
use std::collections::HashMap;

/// Processes Liquid unless tags in a template string.
///
/// This function handles {% unless condition %}content{% endunless %} tags by:
/// - Removing the content if the condition is true (variable exists and equals "true")
/// - Keeping the content if the condition is false (variable doesn't exist or doesn't equal "true")
///
/// # Arguments
/// * `template` - The template string containing unless tags
/// * `variables` - Map of variables for condition evaluation
///
/// # Returns
/// The processed template with unless tags evaluated
pub fn process_liquid_unless_tags(
    template: &str,
    variables: &HashMap<String, String>,
) -> Result<String> {
    let mut result = template.to_string();

    // Process all unless tags by collecting them first, then applying replacements in reverse order
    let mut replacements = Vec::new();
    let mut current_pos = 0;

    // Find all unless blocks
    while let Some(tag_block) = find_tag_block(&result, "{% unless", "{% endunless %}", current_pos)
    {
        // Extract condition from tag content
        let condition = tag_block.tag_content.trim();

        // Evaluate condition
        let condition_is_true = variables.get(condition).map_or(false, |v| v == "true");

        let replacement = if condition_is_true {
            String::new() // Remove content if condition is true
        } else {
            tag_block.inner_content // Keep content if condition is false
        };

        replacements.push((tag_block.start, tag_block.end, replacement));
        current_pos = tag_block.end;
    }

    // Apply replacements in reverse order to maintain correct positions
    super::utils::apply_replacements_in_reverse(&mut result, &replacements);

    // Check if there are any unclosed unless tags
    if result.contains("{% unless") {
        return Err(Error::Liquid("Missing {% endunless %} tag".to_string()));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unless_condition_false_keeps_content() {
        let mut variables = HashMap::new();
        variables.insert("forloop.last".to_string(), "false".to_string());

        let template = "A{% unless forloop.last %}, {% endunless %}B";
        let result = process_liquid_unless_tags(template, &variables).unwrap();

        assert_eq!(result, "A, B");
    }

    #[test]
    fn test_unless_condition_true_removes_content() {
        let mut variables = HashMap::new();
        variables.insert("forloop.last".to_string(), "true".to_string());

        let template = "A{% unless forloop.last %}, {% endunless %}B";
        let result = process_liquid_unless_tags(template, &variables).unwrap();

        assert_eq!(result, "AB");
    }

    #[test]
    fn test_unless_unknown_condition_keeps_content() {
        let variables = HashMap::new();

        let template = "A{% unless unknown %}, {% endunless %}B";
        let result = process_liquid_unless_tags(template, &variables).unwrap();

        assert_eq!(result, "A, B");
    }

    #[test]
    fn test_multiple_unless_tags() {
        let mut variables = HashMap::new();
        variables.insert("first".to_string(), "true".to_string());
        variables.insert("second".to_string(), "false".to_string());

        let template = "A{% unless first %}X{% endunless %}B{% unless second %}Y{% endunless %}C";
        let result = process_liquid_unless_tags(template, &variables).unwrap();

        assert_eq!(result, "ABYC");
    }

    #[test]
    fn test_unclosed_unless_tag() {
        let variables = HashMap::new();

        let template = "A{% unless condition %}B";
        let result = process_liquid_unless_tags(template, &variables);

        assert!(result.is_err());
    }
}
