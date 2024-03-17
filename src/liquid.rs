use crate::handlebars::replace_template_variables;
use std::collections::HashMap;

pub fn parse_liquid_include_tag(tag: &str) -> Option<(String, HashMap<String, String>)> {
    let parts: Vec<&str> = tag.trim().split_whitespace().collect();

    if parts.len() < 2 || !parts[0].starts_with("{%") || !parts.last().unwrap().ends_with("%}") {
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

pub fn process_liquid_includes(input: &str, templates: &HashMap<String, String>) -> String {
    let mut result = input.to_owned();
    let mut start = 0;

    while let Some(start_index) = result[start..].find("{% include") {
        let tag_start = start + start_index;
        let end_index = match result[tag_start..].find("%}") {
            Some(index) => index,
            None => break,
        };

        let tag_end = tag_start + end_index + 2;
        let tag = &result[tag_start..tag_end];

        if let Some((template_name, params)) = parse_liquid_include_tag(tag) {
            if let Some(template_content) = templates.get(&template_name) {
                let processed_content = replace_template_variables(template_content, &params);
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

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_liquid_include_tag() {
        let tag = "{% include template.liquid prop1:\"foo\" prop2:\"bar\" %}";
        let expected_template = "template.liquid".to_string();
        let mut expected_props = HashMap::new();
        expected_props.insert("prop1".to_string(), "foo".to_string());
        expected_props.insert("prop2".to_string(), "bar".to_string());

        match parse_liquid_include_tag(tag) {
            Some((template_name, properties)) => {
                assert_eq!(template_name, expected_template);
                assert_eq!(properties, expected_props);
            }
            None => panic!("Parsing failed for a valid tag"),
        }
    }

    #[test]
    fn test_malformed_liquid_include_tag() {
        let malformed_tag = "{ this is not a valid tag }";
        assert!(parse_liquid_include_tag(malformed_tag).is_none());
    }

    #[test]
    fn test_process_liquid_includes() {
        // Sample input with Liquid include tags
        let input = "Before tag {% include template1.liquid key1:\"value1\" key2:\"value2\" %} and {% include template2.liquid keyA:\"valueA\" %} after tag.";

        // Templates HashMap
        let mut templates = HashMap::new();
        templates.insert(
            "template1.liquid".to_string(),
            "Template 1: {{key1}}, {{key2}}".to_string(),
        );
        templates.insert(
            "template2.liquid".to_string(),
            "Template 2: {{keyA}}".to_string(),
        );

        let expected_output =
            "Before tag Template 1: value1, value2 and Template 2: valueA after tag.";

        let result = process_liquid_includes(input, &templates);

        assert_eq!(result, expected_output);
    }
}
