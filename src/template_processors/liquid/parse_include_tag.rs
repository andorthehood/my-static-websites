use std::collections::HashMap;

/// Parses a liquid include tag and extracts the template name and parameters.
///
/// # Arguments
/// * `tag` - The liquid include tag to parse
///
/// # Returns
/// * `Option<(String, HashMap<String, String>)>` - Template name and parameters if parsing succeeds
pub fn parse_liquid_include_tag(tag: &str) -> Option<(String, HashMap<String, String>)> {
    let parts: Vec<&str> = tag.split_whitespace().collect();

    if parts.len() < 4
        || !parts.first().is_some_and(|p| p.starts_with("{%"))
        || !parts.last().is_some_and(|p| p.ends_with("%}"))
    {
        return None;
    }

    let template_name = parts[2].to_string();
    let mut properties = HashMap::new();

    for &part in &parts[3..parts.len() - 1] {
        let kv: Vec<&str> = part.split(':').collect();
        if kv.len() == 2 {
            let key = kv[0].to_string();
            let value = kv[1].trim_matches('"').to_string();
            properties.insert(key, value);
        }
    }

    Some((template_name, properties))
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
