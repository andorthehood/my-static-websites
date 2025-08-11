use super::utils::find_tag_block;
use crate::error::{Error, Result};
use std::collections::HashMap;

/// Processes Liquid conditional tags in a template string.
///
/// This function handles {% if condition %}content{% endif %} tags by:
/// - Keeping the content if the condition is truthy based on variables
/// - Removing the content if the condition is falsy
///
/// Truthiness: any value present in `variables` that is not empty and not equal to "false".
/// Missing variables are falsy.
///
/// # Arguments
/// * `template` - The template string containing conditional tags
/// * `variables` - Map of variables for condition evaluation
///
/// # Returns
/// The processed template with conditional tags evaluated
pub fn process_liquid_conditional_tags(
    template: &str,
    variables: &HashMap<String, String>,
) -> Result<String> {
    let mut result = template.to_string();
    let mut current_pos = 0;
    let mut replacements = Vec::new();

    if template.is_empty() {
        return Ok(result);
    }

    // Find and process all conditional tags
    while let Some(tag_block) = find_tag_block(&result, "{% if", "{% endif %}", current_pos) {
        let condition = tag_block.tag_content.trim();
        let is_truthy = variables
            .get(condition)
            .map(|v| {
                let t = v.trim();
                !t.is_empty() && t != "false"
            })
            .unwrap_or(false);

        let replacement = if is_truthy {
            tag_block.inner_content
        } else {
            String::new()
        };

        replacements.push((tag_block.start, tag_block.end, replacement));
        current_pos = tag_block.end;
    }

    // Apply replacements in reverse order to maintain correct positions
    super::utils::apply_replacements_in_reverse(&mut result, &replacements);

    // Check if there are any unclosed if tags
    if result.contains("{% if") {
        return Err(Error::Liquid("Missing {% endif %} tag".to_string()));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_if_tags() {
        let input = "{% if something %}lorem ipsum dolor sit amet{% endif %} and some other text {% if another %}this should stay{% endif %}";
        let mut variables = HashMap::new();
        variables.insert("another".to_string(), "true".to_string());
        let expected_output = " and some other text this should stay";
        let output = process_liquid_conditional_tags(input, &variables).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_process_if_tags_with_multiple_conditions() {
        let input = "{% if something %}lorem ipsum dolor sit amet{% endif %} and some other text {% if another %}this should stay{% endif %} {% if yet_another %}this should also stay{% endif %}";
        let mut variables = HashMap::new();
        variables.insert("another".to_string(), "yes".to_string());
        variables.insert("yet_another".to_string(), "1".to_string());
        let expected_output = " and some other text this should stay this should also stay";
        let output = process_liquid_conditional_tags(input, &variables).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_process_if_tags_with_no_conditions() {
        let input = "{% if something %}lorem ipsum dolor sit amet{% endif %} and some other text {% if another %}this should stay{% endif %}";
        let variables: HashMap<String, String> = HashMap::new();
        let expected_output = " and some other text ";
        let output = process_liquid_conditional_tags(input, &variables).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_process_if_tags_with_all_conditions() {
        let input = "{% if something %}lorem ipsum dolor sit amet{% endif %} and some other text {% if another %}this should stay{% endif %}";
        let mut variables = HashMap::new();
        variables.insert("something".to_string(), "true".to_string());
        variables.insert("another".to_string(), "nonempty".to_string());
        let expected_output = "lorem ipsum dolor sit amet and some other text this should stay";
        let output = process_liquid_conditional_tags(input, &variables).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_empty_template() {
        let input = "";
        let mut variables = HashMap::new();
        variables.insert("something".to_string(), "true".to_string());
        let output = process_liquid_conditional_tags(input, &variables).unwrap();
        assert_eq!(output, "");
    }
}
