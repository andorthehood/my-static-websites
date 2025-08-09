use super::utils::find_tag_block;

/// Processes Liquid conditional tags in a template string.
///
/// This function handles {% if condition %}content{% endif %} tags by:
/// - Keeping the content if the condition is in the provided conditions list
/// - Removing the entire tag if the condition is not in the list
///
/// # Arguments
/// * `template` - The template string containing conditional tags
/// * `conditions` - List of condition names that should evaluate to true
///
/// # Returns
/// The processed template with conditional tags evaluated
pub fn process_liquid_conditional_tags(template: &str, conditions: &[String]) -> String {
    let mut result = template.to_string();
    let mut current_pos = 0;
    let mut replacements = Vec::new();

    // Early return for empty template or no conditions to process
    if template.is_empty() {
        return result;
    }

    // Find and process all conditional tags
    while let Some(tag_block) = find_tag_block(&result, "{% if", "{% endif %}", current_pos) {
        let condition = tag_block.tag_content.trim().to_string();
        let replacement = if conditions.contains(&condition) {
            tag_block.inner_content
        } else {
            String::new()
        };

        replacements.push((tag_block.start, tag_block.end, replacement));
        current_pos = tag_block.end;
    }

    // Apply replacements in reverse order to maintain correct positions
    super::utils::apply_replacements_in_reverse(&mut result, &replacements);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_if_tags() {
        let input = "{% if something %}lorem ipsum dolor sit amet{% endif %} and some other text {% if another %}this should stay{% endif %}";
        let conditions = vec!["another".to_string()];
        let expected_output = " and some other text this should stay";
        let output = process_liquid_conditional_tags(input, &conditions);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_process_if_tags_with_multiple_conditions() {
        let input = "{% if something %}lorem ipsum dolor sit amet{% endif %} and some other text {% if another %}this should stay{% endif %} {% if yet_another %}this should also stay{% endif %}";
        let conditions = vec!["another".to_string(), "yet_another".to_string()];
        let expected_output = " and some other text this should stay this should also stay";
        let output = process_liquid_conditional_tags(input, &conditions);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_process_if_tags_with_no_conditions() {
        let input = "{% if something %}lorem ipsum dolor sit amet{% endif %} and some other text {% if another %}this should stay{% endif %}";
        let conditions: Vec<String> = vec![];
        let expected_output = " and some other text ";
        let output = process_liquid_conditional_tags(input, &conditions);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_process_if_tags_with_all_conditions() {
        let input = "{% if something %}lorem ipsum dolor sit amet{% endif %} and some other text {% if another %}this should stay{% endif %}";
        let conditions = vec!["something".to_string(), "another".to_string()];
        let expected_output = "lorem ipsum dolor sit amet and some other text this should stay";
        let output = process_liquid_conditional_tags(input, &conditions);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_empty_template() {
        let input = "";
        let conditions = vec!["something".to_string()];
        let output = process_liquid_conditional_tags(input, &conditions);
        assert_eq!(output, "");
    }
}
