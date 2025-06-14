use crate::error::Result;
use crate::template_processors::handlebars::replace_template_variables;
use std::collections::HashMap;

pub fn parse_liquid_include_tag(tag: &str) -> Option<(String, HashMap<String, String>)> {
    let parts: Vec<&str> = tag.trim().split_whitespace().collect();

    if parts.len() < 4
        || !parts.first().map(|p| p.starts_with("{%")).unwrap_or(false)
        || !parts.last().map(|p| p.ends_with("%}")).unwrap_or(false)
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

pub fn process_liquid_includes(input: &str, templates: &HashMap<String, String>) -> Result<String> {
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
}
