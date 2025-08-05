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

    // Keep processing until no more unless tags found
    loop {
        let start_tag = "{% unless ";
        let end_tag = "{% endunless %}";

        if let Some(start_pos) = result.find(start_tag) {
            // Find the end of the opening tag
            let condition_start = start_pos + start_tag.len();
            let condition_end = result[condition_start..]
                .find(" %}")
                .or_else(|| result[condition_start..].find("%}"))
                .ok_or_else(|| Error::Liquid("Malformed unless tag".to_string()))?
                + condition_start;

            let opening_end = result[condition_end..]
                .find("%}")
                .ok_or_else(|| Error::Liquid("Unclosed unless opening tag".to_string()))?
                + condition_end
                + 2;

            // Find the closing tag
            let content_start = opening_end;
            let closing_start = result[content_start..]
                .find(end_tag)
                .ok_or_else(|| Error::Liquid("Missing {% endunless %} tag".to_string()))?
                + content_start;
            let closing_end = closing_start + end_tag.len();

            // Extract condition and content
            let condition = result[condition_start..condition_end].trim();
            let content = &result[content_start..closing_start];

            // Evaluate condition
            let condition_is_true = variables.get(condition).map_or(false, |v| v == "true");

            let replacement = if condition_is_true {
                String::new() // Remove content if condition is true
            } else {
                content.to_string() // Keep content if condition is false
            };

            // Replace the entire unless block
            result.replace_range(start_pos..closing_end, &replacement);
        } else {
            break;
        }
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
