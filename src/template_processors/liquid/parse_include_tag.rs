use super::utils::{extract_tag_inner, parse_space_separated_key_value_params, skip_whitespace};
use std::collections::HashMap;

/// Validates the basic structure of a liquid include tag.
///
/// # Arguments
/// * `tag` - The tag to validate
///
/// # Returns
/// * `Option<&str>` - The content inside the tag if valid, None otherwise
fn validate_include_tag(tag: &str) -> Option<&str> {
    extract_tag_inner(tag, "include")
}

/// Extracts the template name from the include tag content.
///
/// # Arguments
/// * `content` - The content inside the include tag
///
/// # Returns
/// * `Option<(String, String)>` - Template name and remaining content if parsing succeeds
fn extract_template_name(content: &str) -> Option<(String, String)> {
    let mut chars = content.chars().peekable();
    let mut template_name = String::new();

    // Skip leading whitespace using utility function
    skip_whitespace(&mut chars);

    // Read template name
    while let Some(ch) = chars.peek() {
        if ch.is_whitespace() {
            break;
        }
        template_name.push(chars.next().unwrap());
    }

    if template_name.is_empty() {
        return None;
    }

    let remaining: String = chars.collect();
    Some((template_name, remaining))
}

/// Parses parameters from the remaining content after the template name.
///
/// # Arguments
/// * `remaining` - The remaining content containing parameters
///
/// # Returns
/// * `HashMap<String, String>` - Parsed parameters
fn parse_parameters(remaining: &str) -> HashMap<String, String> {
    parse_space_separated_key_value_params(remaining)
}

/// Parses a liquid include tag and extracts the template name and parameters.
///
/// # Arguments
/// * `tag` - The liquid include tag to parse
///
/// # Returns
/// * `Option<(String, HashMap<String, String>)>` - Template name and parameters if parsing succeeds
pub fn parse_liquid_include_tag(tag: &str) -> Option<(String, HashMap<String, String>)> {
    let content = validate_include_tag(tag)?;
    let (template_name, remaining) = extract_template_name(content)?;
    let properties = parse_parameters(&remaining);

    Some((template_name, properties))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{extract_template_name, parse_parameters, validate_include_tag};

    #[test]
    fn test_validate_include_tag() {
        assert!(validate_include_tag("{% include test.liquid %}").is_some());
        assert!(validate_include_tag("invalid tag").is_none());
        assert!(validate_include_tag("{% include test.liquid").is_none());
    }

    #[test]
    fn test_extract_template_name() {
        let result = extract_template_name(" test.liquid param:value");
        assert!(result.is_some());
        let (name, remaining) = result.unwrap();
        assert_eq!(name, "test.liquid");
        assert_eq!(remaining.trim(), "param:value");

        assert!(extract_template_name("").is_none());
    }

    #[test]
    fn test_parse_parameters() {
        let result = parse_parameters("name:\"Alice\" greeting:\"Hello\"");
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("name"), Some(&"Alice".to_string()));
        assert_eq!(result.get("greeting"), Some(&"Hello".to_string()));

        let empty_result = parse_parameters("");
        assert!(empty_result.is_empty());
    }

    #[test]
    fn test_parse_simple_include_tag() {
        let tag = "{% include header.liquid %}";
        let result = parse_liquid_include_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "header.liquid");
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_include_tag_with_parameters() {
        let tag = "{% include greeting.liquid name:\"Alice\" greeting:\"Hello\" %}";
        let result = parse_liquid_include_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "greeting.liquid");
        assert_eq!(params.get("name"), Some(&"Alice".to_string()));
        assert_eq!(params.get("greeting"), Some(&"Hello".to_string()));
    }

    #[test]
    fn test_parse_invalid_include_tag() {
        let tag = "invalid tag";
        let result = parse_liquid_include_tag(tag);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_include_tag_with_malformed_parameter() {
        let tag = "{% include t.liquid malformed greeting:\"Hello\" %}";
        let result = parse_liquid_include_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "t.liquid");
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("greeting"), Some(&"Hello".to_string()));
    }

    #[test]
    fn test_parse_include_tag_with_spaces_in_quoted_value() {
        let tag = "{% include header.liquid name:\"Hello Worlds\" %}";
        let result = parse_liquid_include_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "header.liquid");
        assert_eq!(params.get("name"), Some(&"Hello Worlds".to_string()));
    }
}
