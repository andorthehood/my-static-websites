use super::utils::skip_whitespace;
use std::collections::HashMap;

/// Validates the basic structure of a liquid include tag.
///
/// # Arguments
/// * `tag` - The tag to validate
///
/// # Returns
/// * `Option<&str>` - The content inside the tag if valid, None otherwise
fn validate_include_tag(tag: &str) -> Option<&str> {
    let trimmed = tag.trim();

    if !trimmed.starts_with("{% include") || !trimmed.ends_with("%}") {
        return None;
    }

    Some(&trimmed[10..trimmed.len() - 2].trim())
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
    let mut properties = HashMap::new();
    let remaining = remaining.trim();

    if remaining.is_empty() {
        return properties;
    }

    let mut i = 0;
    let remaining_chars: Vec<char> = remaining.chars().collect();

    while i < remaining_chars.len() {
        // Skip whitespace
        while i < remaining_chars.len() && remaining_chars[i].is_whitespace() {
            i += 1;
        }

        if i >= remaining_chars.len() {
            break;
        }

        // Read key - stop at whitespace or colon
        let mut key = String::new();
        while i < remaining_chars.len()
            && remaining_chars[i] != ':'
            && !remaining_chars[i].is_whitespace()
        {
            key.push(remaining_chars[i]);
            i += 1;
        }

        // Skip whitespace after key
        while i < remaining_chars.len() && remaining_chars[i].is_whitespace() {
            i += 1;
        }

        if i >= remaining_chars.len() || remaining_chars[i] != ':' {
            // Malformed parameter (no colon), just continue - we're already positioned correctly
            continue;
        }

        i += 1; // Skip the ':'

        // Read value
        let mut value = String::new();
        let mut in_quotes = false;

        if i < remaining_chars.len() && remaining_chars[i] == '"' {
            in_quotes = true;
            i += 1; // Skip opening quote
        }

        while i < remaining_chars.len() {
            let ch = remaining_chars[i];
            if in_quotes {
                if ch == '"' {
                    i += 1; // Skip closing quote
                    break;
                }
                value.push(ch);
            } else {
                if ch.is_whitespace() {
                    break;
                }
                value.push(ch);
            }
            i += 1;
        }

        if !key.is_empty() {
            properties.insert(key.trim().to_string(), value);
        }
    }

    properties
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
