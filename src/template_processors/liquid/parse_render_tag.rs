use super::utils::{extract_tag_inner, parse_space_separated_key_value_params, skip_whitespace};
use std::collections::HashMap;

/// Validates the basic structure of a liquid render tag.
///
/// # Arguments
/// * `tag` - The tag to validate
///
/// # Returns
/// * `Option<&str>` - The content inside the tag if valid, None otherwise
fn validate_render_tag(tag: &str) -> Option<&str> {
    extract_tag_inner(tag, "render")
}

/// Extracts the template name from the render tag content.
/// Now supports both quoted and unquoted template names, and normalizes by removing .liquid extension.
///
/// # Arguments
/// * `content` - The content inside the render tag
///
/// # Returns
/// * `Option<(String, String)>` - Template name and remaining content if parsing succeeds
fn extract_template_name(content: &str) -> Option<(String, String)> {
    let mut chars = content.chars().peekable();
    let mut template_name = String::new();

    // Skip leading whitespace using utility function
    skip_whitespace(&mut chars);

    // Check if the template name is quoted
    let is_quoted = chars.peek() == Some(&'\'') || chars.peek() == Some(&'"');
    let quote_char = if is_quoted {
        chars.next() // consume the opening quote
    } else {
        None
    };

    // Read template name
    while let Some(ch) = chars.peek() {
        if is_quoted {
            // If quoted, read until the closing quote
            if Some(*ch) == quote_char {
                chars.next(); // consume the closing quote
                break;
            }
        } else {
            // If not quoted, read until whitespace (backward compatibility)
            if ch.is_whitespace() {
                break;
            }
        }
        template_name.push(chars.next().unwrap());
    }

    if template_name.is_empty() {
        return None;
    }

    // Normalize the template name by removing .liquid extension if present
    let normalized_name = template_name.strip_suffix(".liquid").unwrap_or(&template_name);

    let remaining: String = chars.collect();
    Some((normalized_name.to_string(), remaining))
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

/// Parses a liquid render tag and extracts the template name and parameters.
///
/// # Arguments
/// * `tag` - The liquid render tag to parse
///
/// # Returns
/// * `Option<(String, HashMap<String, String>)>` - Template name and parameters if parsing succeeds
pub fn parse_liquid_render_tag(tag: &str) -> Option<(String, HashMap<String, String>)> {
    let content = validate_render_tag(tag)?;
    let (template_name, remaining) = extract_template_name(content)?;
    let properties = parse_parameters(&remaining);

    Some((template_name, properties))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{extract_template_name, parse_parameters, validate_render_tag};

    #[test]
    fn test_validate_render_tag() {
        assert!(validate_render_tag("{% render test.liquid %}").is_some());
        assert!(validate_render_tag("invalid tag").is_none());
        assert!(validate_render_tag("{% render test.liquid").is_none());
    }

    #[test]
    fn test_extract_template_name() {
        let result = extract_template_name(" test.liquid param:value");
        assert!(result.is_some());
        let (name, remaining) = result.unwrap();
        assert_eq!(name, "test"); // Now expects normalized name without .liquid
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
    fn test_parse_simple_render_tag() {
        let tag = "{% render header.liquid %}";
        let result = parse_liquid_render_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "header"); // Now expects normalized name without .liquid
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_render_tag_with_parameters() {
        let tag = "{% render greeting.liquid name:\"Alice\" greeting:\"Hello\" %}";
        let result = parse_liquid_render_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "greeting"); // Now expects normalized name without .liquid
        assert_eq!(params.get("name"), Some(&"Alice".to_string()));
        assert_eq!(params.get("greeting"), Some(&"Hello".to_string()));
    }

    #[test]
    fn test_parse_invalid_render_tag() {
        let tag = "invalid tag";
        let result = parse_liquid_render_tag(tag);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_render_tag_with_malformed_parameter() {
        let tag = "{% render t.liquid malformed greeting:\"Hello\" %}";
        let result = parse_liquid_render_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "t"); // Now expects normalized name without .liquid
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("greeting"), Some(&"Hello".to_string()));
    }

    #[test]
    fn test_parse_render_tag_with_spaces_in_quoted_value() {
        let tag = "{% render header.liquid name:\"Hello Worlds\" %}";
        let result = parse_liquid_render_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "header"); // Now expects normalized name without .liquid
        assert_eq!(params.get("name"), Some(&"Hello Worlds".to_string()));
    }

    #[test]
    fn test_parse_render_tag_with_single_quotes() {
        let tag = "{% render 'header.liquid' name:\"World\" %}";
        let result = parse_liquid_render_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "header"); // Extension stripped
        assert_eq!(params.get("name"), Some(&"World".to_string()));
    }

    #[test]
    fn test_parse_render_tag_with_double_quotes() {
        let tag = "{% render \"header.liquid\" name:\"World\" %}";
        let result = parse_liquid_render_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "header"); // Extension stripped
        assert_eq!(params.get("name"), Some(&"World".to_string()));
    }

    #[test]
    fn test_parse_render_tag_quoted_without_extension() {
        let tag = "{% render 'header' name:\"World\" %}";
        let result = parse_liquid_render_tag(tag);

        assert!(result.is_some());
        let (template_name, params) = result.unwrap();
        assert_eq!(template_name, "header"); // No extension to strip
        assert_eq!(params.get("name"), Some(&"World".to_string()));
    }

    #[test]
    fn test_parse_render_tag_comprehensive_syntax_support() {
        // Test that all these syntaxes produce the same normalized template name
        let test_cases = vec![
            "{% render header.liquid %}",
            "{% render 'header.liquid' %}",
            "{% render \"header.liquid\" %}",
            "{% render 'header' %}",
            "{% render \"header\" %}",
        ];

        for tag in test_cases {
            let result = parse_liquid_render_tag(tag);
            assert!(result.is_some(), "Failed to parse: {}", tag);
            let (template_name, _) = result.unwrap();
            assert_eq!(template_name, "header", "Unexpected template name for: {}", tag);
        }
    }
}
