/// Represents a conditional tag in the template
#[derive(Debug)]
struct ConditionalTag {
    /// Start position of the entire tag in the template
    start: usize,
    /// End position of the entire tag in the template
    end: usize,
    /// The condition being checked (e.g., "something" in {% if something %})
    condition: String,
    /// The content inside the if block
    content: String,
}

/// Finds and parses a conditional tag starting at the given position
fn find_conditional_tag(template: &str, start_pos: usize) -> Option<ConditionalTag> {
    const IF_TAG_START: &str = "{% if ";
    const IF_TAG_END: &str = "%}";
    const ENDIF_TAG: &str = "{% endif %}";

    // Find the start of the if tag
    let tag_start = template[start_pos..].find(IF_TAG_START)
        .map(|pos| start_pos + pos)?;

    // Find the end of the entire if block
    let tag_end = template[tag_start..].find(ENDIF_TAG)
        .map(|pos| tag_start + pos + ENDIF_TAG.len())?;

    // Extract the tag content
    let tag_content = &template[tag_start..tag_end];

    // Find where the condition ends
    let condition_end = tag_content.find(IF_TAG_END)?;
    
    // Extract and trim the condition
    let condition = tag_content[IF_TAG_START.len()..condition_end].trim().to_string();
    
    // Extract the content between the if and endif tags
    let content_start = tag_start + condition_end + IF_TAG_END.len();
    let content_end = tag_end - ENDIF_TAG.len();
    let content = template[content_start..content_end].to_string();

    Some(ConditionalTag {
        start: tag_start,
        end: tag_end,
        condition,
        content,
    })
}

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
    while let Some(tag) = find_conditional_tag(&result, current_pos) {
        let replacement = if conditions.contains(&tag.condition) {
            tag.content
        } else {
            String::new()
        };
        
        replacements.push((tag.start, tag.end, replacement));
        current_pos = tag.end;
    }

    // Apply replacements in reverse order to maintain correct positions
    for (start, end, replacement) in replacements.iter().rev() {
        result.replace_range(*start..*end, replacement);
    }

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