pub fn parse_liquid_include_tag(tag: &str) -> Option<(String, HashMap<String, String>)> {
    let parts: Vec<&str> = tag.trim().split_whitespace().collect();

    if parts.len() < 2 || !parts[0].starts_with("{%") || !parts.last().unwrap().ends_with("%}") {
        return None;
    }

    let template_file = parts[2].to_string();
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
    fn test_valid_liquid_include_tag() {
        let tag = "{% include 'template.liquid' prop1:\"foo\" prop2:\"bar\" %}";
        let expected_template = "template.liquid".to_string();
        let mut expected_props = HashMap::new();
        expected_props.insert("prop1".to_string(), "foo".to_string());
        expected_props.insert("prop2".to_string(), "bar".to_string());

        match parse_liquid_include_tag(tag) {
            Some((template_name, properties)) => {
                assert_eq!(template_name, expected_template);
                assert_eq!(properties, expected_props);
            },
            None => panic!("Parsing failed for a valid tag"),
        }
    }

    #[test]
    fn test_malformed_liquid_include_tag() {
        let malformed_tag = "{% this is not a valid tag %}";
        assert!(parse_liquid_include_tag(malformed_tag).is_none());
    }
}