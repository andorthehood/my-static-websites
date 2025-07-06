use crate::error::Result;
use crate::template_processors::handlebars::{
    remove_handlebars_variables, replace_template_variables,
};
use crate::template_processors::liquid::{process_liquid_conditional_tags, process_liquid_tags};
use crate::template_processors::markdown::markdown_to_html;
use crate::types::{ContentItem, TemplateIncludes, Variables};
use std::collections::HashMap;

/// Processes all template tags in a given input string.
/// This includes both Liquid conditionals and Handlebars variables.
pub fn process_template_tags(input: &str, variables: &HashMap<String, String>) -> Result<String> {
    let mut result = input.to_string();
    // First process Liquid conditionals
    let keys: Vec<String> = variables.keys().cloned().collect();
    result = process_liquid_conditional_tags(&result, &keys);

    // Then process Handlebars variables
    result = replace_template_variables(&result, variables)?;
    result = remove_handlebars_variables(&result)?;

    Ok(result)
}

/// Processes all template tags including liquid includes in a given input string.
/// This includes Liquid conditionals, Liquid includes, and Handlebars variables.
pub fn process_template_tags_with_includes(
    input: &str,
    variables: &HashMap<String, String>,
    includes: &TemplateIncludes,
) -> Result<String> {
    let keys: Vec<String> = variables.keys().cloned().collect();
    // First process Liquid tags (both conditionals and includes)
    let result = process_liquid_tags(input, &keys, includes)?;

    // Then process Handlebars variables
    let result = replace_template_variables(&result, variables)?;
    let result = remove_handlebars_variables(&result)?;

    Ok(result)
}

/// Centralized content processing function that handles the complete pipeline:
/// 1. Processes liquid includes and conditionals
/// 2. Converts markdown to HTML (if needed)
/// 3. Processes handlebars template variables
///
/// This ensures consistent processing across all content generation functions.
pub fn process_content(
    content: &str,
    content_item: &ContentItem,
    includes: &TemplateIncludes,
    variables: &Variables,
) -> Result<String> {
    // Create a combined variable set for processing
    let mut combined_variables = variables.clone();
    combined_variables.extend(content_item.clone());

    // Step 1: Process liquid includes and conditionals
    let content_with_liquid =
        process_template_tags_with_includes(content, &combined_variables, includes)?;

    // Step 2: Convert markdown to HTML if needed
    let is_markdown = content_item.get("file_type").map_or(true, |ft| ft == "md");
    let html_content = if is_markdown {
        markdown_to_html(&content_with_liquid)
    } else {
        content_with_liquid
    };

    Ok(html_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_template_tags() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "World".to_string());
        variables.insert("show_greeting".to_string(), "true".to_string());

        let input = "{% if show_greeting %}Hello {{name}}!{% endif %}";
        let result =
            process_template_tags(input, &variables).expect("Processing template tags failed");
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_process_template_tags_with_includes() {
        let mut includes = HashMap::new();
        includes.insert("test.liquid".to_string(), "Hello {{ name }}!".to_string());

        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "World".to_string());

        let input = "{% include test.liquid name:\"World\" %}";
        let result = process_template_tags_with_includes(input, &variables, &includes).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_process_content_with_liquid_includes() {
        let mut includes = HashMap::new();
        includes.insert("test.liquid".to_string(), "Hello {{ name }}!".to_string());

        let mut content_item = HashMap::new();
        content_item.insert("name".to_string(), "World".to_string());
        content_item.insert("file_type".to_string(), "md".to_string());

        let variables = HashMap::new();

        let content = "{% include test.liquid name:\"World\" %}";
        let result = process_content(content, &content_item, &includes, &variables).unwrap();
        // The markdown processor strips line breaks between non-list lines
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_process_content_markdown_only() {
        let includes = HashMap::new();
        let mut content_item = HashMap::new();
        content_item.insert("file_type".to_string(), "md".to_string());
        let variables = HashMap::new();

        let content = "# Test Heading\n\nThis is a paragraph.";
        let result = process_content(content, &content_item, &includes, &variables).unwrap();
        // The markdown processor strips line breaks between non-list lines
        assert_eq!(result, "# Test HeadingThis is a paragraph.");
    }

    #[test]
    fn test_process_content_html_only() {
        let includes = HashMap::new();
        let mut content_item = HashMap::new();
        content_item.insert("file_type".to_string(), "html".to_string());
        let variables = HashMap::new();

        let content = "<p>Already HTML</p>";
        let result = process_content(content, &content_item, &includes, &variables).unwrap();
        assert_eq!(result, "<p>Already HTML</p>");
    }

    #[test]
    fn test_process_content_with_template_variables() {
        let includes = HashMap::new();
        let mut content_item = HashMap::new();
        content_item.insert("file_type".to_string(), "md".to_string());
        content_item.insert("title".to_string(), "Test Title".to_string());

        let variables = HashMap::new();

        let content = "# {{title}}\n\nContent here.";
        let result = process_content(content, &content_item, &includes, &variables).unwrap();
        // The markdown processor strips line breaks between non-list lines
        assert_eq!(result, "# Test TitleContent here.");
    }
}
