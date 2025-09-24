use crate::error::{Error, Result};
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

/// Represents a nested-aware conditional block
#[derive(Debug, PartialEq)]
struct NestedIfBlock {
    start: usize,
    end: usize,
    condition: String,
    inner_content: String,
}

/// Finds the next IF block with proper nested depth awareness
fn find_nested_if_block(template: &str, start_pos: usize) -> Result<Option<NestedIfBlock>> {
    let template_slice = &template[start_pos..];

    // Find the start of the next {% if tag
    let Some(if_start_rel) = template_slice.find("{% if") else {
        return Ok(None);
    };
    let if_start = start_pos + if_start_rel;

    // Find the end of the opening tag
    let Some(opening_end_rel) = template_slice[if_start_rel..].find("%}") else {
        return Err(Error::Liquid("Unclosed {% if tag".to_string()));
    };
    let opening_end = if_start + opening_end_rel + 2;

    // Extract the condition from the opening tag
    let condition_start = if_start + 5; // Skip "{% if"
    let condition_end = opening_end - 2; // Before "%}"
    let condition = template[condition_start..condition_end].trim().to_string();

    // Now use character iteration to find the matching {% endif %} with proper nesting
    let mut chars = template[opening_end..].chars().peekable();
    let inner_content = read_nested_if_content(&mut chars)?;

    // Calculate the end position
    let content_len = inner_content.len();
    let inner_end = opening_end + content_len;
    let endif_end = inner_end + "{% endif %}".len();

    Ok(Some(NestedIfBlock {
        start: if_start,
        end: endif_end,
        condition,
        inner_content,
    }))
}

/// Reads the content of an IF block until finding the matching endif, tracking nested depth
fn read_nested_if_content(chars: &mut Peekable<Chars>) -> Result<String> {
    let mut content = String::new();
    let mut depth = 1i32; // We start inside an {% if %} block

    while depth > 0 {
        let Some(c) = chars.next() else {
            return Err(Error::Liquid(
                "Unclosed block - missing {% endif %}".to_string(),
            ));
        };

        if c == '{' && chars.peek() == Some(&'%') {
            chars.next(); // consume '%'
            let mut tag_content = String::new();

            // Read the tag content until %}
            while let Some(tc) = chars.next() {
                if tc == '%' && chars.peek() == Some(&'}') {
                    chars.next(); // consume '}'
                    break;
                }
                tag_content.push(tc);
            }

            let trimmed = tag_content.trim();

            // Check if this affects our depth
            if trimmed.starts_with("if ") {
                depth += 1;
            } else if trimmed == "endif" {
                depth -= 1;
            }

            // Only add to content if we're still inside the block
            if depth > 0 {
                content.push_str("{% ");
                content.push_str(trimmed);
                content.push_str(" %}");
            }
        } else if depth > 0 {
            content.push(c);
        }
    }

    Ok(content)
}

/// Processes Liquid conditional tags in a template string with proper nested support.
///
/// This function handles {% if condition %}content{% endif %} tags by:
/// - Keeping the content if the condition is truthy based on variables
/// - Removing the content if the condition is falsy
/// - Properly handling nested {% if %} blocks with depth tracking
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
    if template.is_empty() {
        return Ok(String::new());
    }

    let mut result = template.to_string();
    let mut replacements = Vec::new();
    let mut current_pos = 0;

    // Find and process all conditional tags with proper nesting
    while let Some(if_block) = find_nested_if_block(&result, current_pos)? {
        let condition = if_block.condition.trim();
        let is_truthy = variables.get(condition).is_some_and(|v| {
            let t = v.trim();
            !t.is_empty() && t != "false"
        });

        // Recursively process the inner content if the condition is truthy
        let replacement = if is_truthy {
            // Process nested IF blocks within the content recursively
            process_liquid_conditional_tags(&if_block.inner_content, variables)?
        } else {
            String::new()
        };

        replacements.push((if_block.start, if_block.end, replacement));
        current_pos = if_block.end;
    }

    // Apply replacements in reverse order to maintain correct positions
    super::utils::apply_replacements_in_reverse(&mut result, &replacements);

    // Check if there are any unclosed if tags remaining
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

    #[test]
    fn test_nested_if_truthy_outer_truthy_inner() {
        let input =
            "{% if outer %}Outer start {% if inner %}Inner content{% endif %} Outer end{% endif %}";
        let mut variables = HashMap::new();
        variables.insert("outer".to_string(), "true".to_string());
        variables.insert("inner".to_string(), "true".to_string());
        let expected_output = "Outer start Inner content Outer end";
        let output = process_liquid_conditional_tags(input, &variables).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_nested_if_truthy_outer_falsy_inner() {
        let input =
            "{% if outer %}Outer start {% if inner %}Inner content{% endif %} Outer end{% endif %}";
        let mut variables = HashMap::new();
        variables.insert("outer".to_string(), "true".to_string());
        // inner is falsy (not defined)
        let expected_output = "Outer start  Outer end";
        let output = process_liquid_conditional_tags(input, &variables).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_nested_if_falsy_outer() {
        let input =
            "{% if outer %}Outer start {% if inner %}Inner content{% endif %} Outer end{% endif %}";
        let variables: HashMap<String, String> = HashMap::new();
        // Both outer and inner are falsy (not defined)
        let expected_output = "";
        let output = process_liquid_conditional_tags(input, &variables).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_deeply_nested_if_blocks() {
        let input = "{% if level1 %}L1 {% if level2 %}L2 {% if level3 %}L3 content{% endif %} L2 end{% endif %} L1 end{% endif %}";
        let mut variables = HashMap::new();
        variables.insert("level1".to_string(), "true".to_string());
        variables.insert("level2".to_string(), "true".to_string());
        variables.insert("level3".to_string(), "true".to_string());
        let expected_output = "L1 L2 L3 content L2 end L1 end";
        let output = process_liquid_conditional_tags(input, &variables).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_unterminated_nested_if_block() {
        let input = "{% if outer %}Outer start {% if inner %}Inner content";
        let mut variables = HashMap::new();
        variables.insert("outer".to_string(), "true".to_string());
        let result = process_liquid_conditional_tags(input, &variables);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("missing {% endif %}")
                || error_msg.contains("Missing {% endif %} tag")
        );
    }
}
