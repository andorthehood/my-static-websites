use super::parse_render_tag::parse_liquid_render_tag;
use super::replace_variables::replace_template_variables;
use crate::error::Result;
use std::collections::HashMap;

/// Processes all liquid render tags in the input string and replaces them with template content.
///
/// # Arguments
/// * `input` - The input string containing liquid render tags
/// * `templates` - A `HashMap` containing template names and their content
///
/// # Returns
/// * `Result<String>` - The processed string with renders replaced or an error if processing fails
pub fn process_liquid_renders(input: &str, templates: &HashMap<String, String>) -> Result<String> {
    let mut result = input.to_owned();
    let mut start = 0;

    while let Some(start_index) = result[start..].find("{% render") {
        let tag_start = start + start_index;
        let Some(end_index) = result[tag_start..].find("%}") else {
            break;
        };

        let tag_end = tag_start + end_index + 2;
        let tag = &result[tag_start..tag_end];

        if let Some((template_name, params)) = parse_liquid_render_tag(tag) {
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
    fn test_process_liquid_renders() {
        let mut templates = HashMap::new();
        templates.insert(
            "header".to_string(), // Now using normalized key without .liquid
            "Hello, {{ name }}!".to_string(),
        );

        let input = "{% render header.liquid name:\"World\" %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_process_liquid_renders_without_variables() {
        let mut templates = HashMap::new();
        templates.insert("simple".to_string(), "Simple template".to_string()); // Normalized key

        let input = "{% render simple.liquid %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "Simple template");
    }

    #[test]
    fn test_process_liquid_renders_with_multiple_variables() {
        let mut templates = HashMap::new();
        templates.insert(
            "greeting".to_string(), // Normalized key
            "{{ greeting }}, {{ name }}!".to_string(),
        );

        let input = "{% render greeting.liquid greeting:\"Hi\" name:\"Alice\" %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "Hi, Alice!");
    }

    #[test]
    fn test_process_liquid_renders_template_not_found() {
        let templates = HashMap::new();
        let input = "{% render not_found.liquid %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "{% render not_found.liquid %}");
    }

    #[test]
    fn test_process_liquid_renders_malformed_tag() {
        let templates = HashMap::new();
        let input = "{% render malformed %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "{% render malformed %}");
    }

    #[test]
    fn test_process_liquid_renders_unclosed_tag() {
        let templates = HashMap::new();
        let input = "{% render unclosed";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "{% render unclosed");
    }

    #[test]
    fn test_process_liquid_renders_with_error() {
        let mut templates = HashMap::new();
        templates.insert("header".to_string(), "Hello, {{ name }!".to_string()); // Normalized key

        let input = "{% render header.liquid name:\"World\" %}";
        let result = process_liquid_renders(input, &templates);
        assert!(result.is_err());
    }

    #[test]
    fn test_process_liquid_renders_with_spaces_in_parameter_value() {
        let mut templates = HashMap::new();
        templates.insert(
            "header".to_string(), // Normalized key
            "Hello, {{ name }}!".to_string(),
        );

        let input = "{% render header.liquid name:\"Hello Worlds\" %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "Hello, Hello Worlds!");
    }

    #[test]
    fn test_process_liquid_renders_resume_after_missing() {
        let mut templates = HashMap::new();
        templates.insert("ok".to_string(), "OK".to_string()); // Normalized key

        let input = "{% render missing.liquid %} and {% render ok.liquid %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "{% render missing.liquid %} and OK");
    }

    #[test]
    fn test_process_liquid_renders_comprehensive_syntax() {
        let mut templates = HashMap::new();
        templates.insert("header".to_string(), "HEADER".to_string());

        // Test that all syntaxes resolve to the same template
        let test_cases = vec![
            "{% render header.liquid %}",
            "{% render 'header.liquid' %}",
            "{% render \"header.liquid\" %}",
            "{% render 'header' %}",
            "{% render \"header\" %}",
        ];

        for input in test_cases {
            let result = process_liquid_renders(input, &templates).unwrap();
            assert_eq!(result, "HEADER", "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_process_liquid_renders_nested_paths() {
        let mut templates = HashMap::new();
        templates.insert(
            "components/buttons/cta".to_string(),
            "<button class=\"cta-button\">{{ text }}</button>".to_string(),
        );
        templates.insert(
            "layout/sidebar".to_string(),
            "<aside>{{ title }}</aside>".to_string(),
        );

        // Test nested path with parameters
        let input = "{% render 'components/buttons/cta' text:\"Buy Now\" %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "<button class=\"cta-button\">Buy Now</button>");

        // Test different levels of nesting
        let input = "{% render layout/sidebar title:\"My Sidebar\" %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "<aside>My Sidebar</aside>");

        // Test basic template without variables
        templates.insert(
            "components/card".to_string(),
            "<div class=\"card\">Card content</div>".to_string(),
        );
        let input = "{% render 'components/card' %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "<div class=\"card\">Card content</div>");
    }

    #[test]
    fn test_process_liquid_renders_nested_path_not_found() {
        let templates = HashMap::new();

        let input = "Before {% render 'components/not/found' %} After";
        let result = process_liquid_renders(input, &templates).unwrap();
        // Should leave the tag unchanged when template is not found
        assert_eq!(result, "Before {% render 'components/not/found' %} After");
    }

    #[test]
    fn test_process_liquid_renders_mixed_flat_and_nested() {
        let mut templates = HashMap::new();
        templates.insert("header".to_string(), "<h1>{{ title }}</h1>".to_string());
        templates.insert(
            "components/footer".to_string(),
            "<footer>{{ copyright }}</footer>".to_string(),
        );

        let input = "{% render header title:\"Welcome\" %} {% render 'components/footer' copyright:\"2024\" %}";
        let result = process_liquid_renders(input, &templates).unwrap();
        assert_eq!(result, "<h1>Welcome</h1> <footer>2024</footer>");
    }
}
