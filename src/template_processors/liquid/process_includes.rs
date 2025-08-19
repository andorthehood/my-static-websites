use super::parse_include_tag::parse_liquid_include_tag;
use super::replace_variables::replace_template_variables;
use crate::error::Result;
use std::collections::HashMap;

/// Processes all liquid include tags in the input string and replaces them with template content.
///
/// # Arguments
/// * `input` - The input string containing liquid include tags
/// * `templates` - A `HashMap` containing template names and their content
///
/// # Returns
/// * `Result<String>` - The processed string with includes replaced or an error if processing fails
pub fn process_liquid_includes(input: &str, templates: &HashMap<String, String>) -> Result<String> {
    let mut result = input.to_owned();
    let mut start = 0;

    while let Some(start_index) = result[start..].find("{% include") {
        let tag_start = start + start_index;
        let Some(end_index) = result[tag_start..].find("%}") else {
            break;
        };

        let tag_end = tag_start + end_index + 2;
        let tag = &result[tag_start..tag_end];

        if let Some((template_name, params)) = parse_liquid_include_tag(tag) {
            if let Some(template_content) = templates.get(&template_name) {
                let processed_content = replace_template_variables(template_content, &params)?;
                result.replace_range(tag_start..tag_end, &processed_content);

                start = tag_start + processed_content.len();
            } else {
                // Move start to just after the current tag if the template was not found
                start = tag_end;
            }
        } else {
            // Move start to just after the current tag if parsing failed
            start = tag_end;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_liquid_includes() {
        let mut templates = HashMap::new();
        templates.insert(
            "header.liquid".to_string(),
            "Hello, {{ name }}!".to_string(),
        );

        let input = "{% include header.liquid name:\"World\" %}";
        let result = process_liquid_includes(input, &templates).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_process_liquid_includes_without_variables() {
        let mut templates = HashMap::new();
        templates.insert("simple.liquid".to_string(), "Simple template".to_string());

        let input = "{% include simple.liquid %}";
        let result = process_liquid_includes(input, &templates).unwrap();
        assert_eq!(result, "Simple template");
    }

    #[test]
    fn test_process_liquid_includes_with_multiple_variables() {
        let mut templates = HashMap::new();
        templates.insert(
            "greeting.liquid".to_string(),
            "{{ greeting }}, {{ name }}!".to_string(),
        );

        let input = "{% include greeting.liquid greeting:\"Hi\" name:\"Alice\" %}";
        let result = process_liquid_includes(input, &templates).unwrap();
        assert_eq!(result, "Hi, Alice!");
    }

    #[test]
    fn test_process_liquid_includes_template_not_found() {
        let templates = HashMap::new();
        let input = "{% include not_found.liquid %}";
        let result = process_liquid_includes(input, &templates).unwrap();
        assert_eq!(result, "{% include not_found.liquid %}");
    }

    #[test]
    fn test_process_liquid_includes_malformed_tag() {
        let templates = HashMap::new();
        let input = "{% include malformed %}";
        let result = process_liquid_includes(input, &templates).unwrap();
        assert_eq!(result, "{% include malformed %}");
    }

    #[test]
    fn test_process_liquid_includes_unclosed_tag() {
        let templates = HashMap::new();
        let input = "{% include unclosed";
        let result = process_liquid_includes(input, &templates).unwrap();
        assert_eq!(result, "{% include unclosed");
    }

    #[test]
    fn test_process_liquid_includes_with_error() {
        let mut templates = HashMap::new();
        templates.insert("header.liquid".to_string(), "Hello, {{ name }!".to_string());

        let input = "{% include header.liquid name:\"World\" %}";
        let result = process_liquid_includes(input, &templates);
        assert!(result.is_err());
    }

    #[test]
    fn test_process_liquid_includes_with_spaces_in_parameter_value() {
        let mut templates = HashMap::new();
        templates.insert(
            "header.liquid".to_string(),
            "Hello, {{ name }}!".to_string(),
        );

        let input = "{% include header.liquid name:\"Hello Worlds\" %}";
        let result = process_liquid_includes(input, &templates).unwrap();
        assert_eq!(result, "Hello, Hello Worlds!");
    }

    #[test]
    fn test_process_liquid_includes_resume_after_missing() {
        let mut templates = HashMap::new();
        templates.insert("ok.liquid".to_string(), "OK".to_string());

        let input = "{% include missing.liquid %} and {% include ok.liquid %}";
        let result = process_liquid_includes(input, &templates).unwrap();
        assert_eq!(result, "{% include missing.liquid %} and OK");
    }
}
