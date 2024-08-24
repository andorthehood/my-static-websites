pub fn process_liquid_conditional_tags(input: &str, conditions: &[String]) -> String {
    const IF_TAG_START: &str = "{% if ";
    const IF_TAG_END: &str = "%}";
    const ENDIF_TAG: &str = "{% endif %}";
    const IF_TAG_START_LEN: usize = IF_TAG_START.len();
    const ENDIF_TAG_LEN: usize = ENDIF_TAG.len();

    let mut result = input.to_string();
    let mut start = 0;
    let mut replacements = Vec::new();

    while let Some(start_index) = result[start..].find(IF_TAG_START) {
        let tag_start = start + start_index;
        let end_index = match result[tag_start..].find(ENDIF_TAG) {
            Some(index) => tag_start + index + ENDIF_TAG_LEN,
            None => break,
        };
        let tag_content = &result[tag_start..end_index];
        if let Some(condition_end) = tag_content.find(IF_TAG_END) {
            let condition = &tag_content[IF_TAG_START_LEN..condition_end].trim();
            if conditions.contains(&condition.to_string()) {
                // Collect the range to replace with just the content inside
                let content_start = tag_start + condition_end + IF_TAG_END.len();
                let content_end = end_index - ENDIF_TAG_LEN;
                let content = &result[content_start..content_end];
                replacements.push((tag_start, end_index, content.to_string()));
            } else {
                // Collect the range to remove
                replacements.push((tag_start, end_index, "".to_string()));
            }
        }

        // Move start to the end of the current tag to avoid overlapping replacements
        start = end_index;
    }

    // Perform the replacements
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
}