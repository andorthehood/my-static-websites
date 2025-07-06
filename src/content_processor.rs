use crate::error::Result;
use crate::template_processors::liquid::process_liquid_tags;
use crate::template_processors::markdown::markdown_to_html;
use crate::template_processors::process_template_tags;
use crate::types::{ContentItem, TemplateIncludes, Variables};

/// Centralized content processing function that handles the complete pipeline:
/// 1. Processes liquid includes
/// 2. Converts markdown to HTML (if needed)
/// 3. Processes template variables
///
/// This ensures consistent processing across all content generation functions.
pub fn process_content(
    content: &str,
    content_item: &ContentItem,
    includes: &TemplateIncludes,
    variables: &Variables,
) -> Result<String> {
    // Create a combined variable set for liquid processing
    let mut combined_variables = variables.clone();
    combined_variables.extend(content_item.clone());

    // Get variable keys for liquid processing
    let keys: Vec<String> = combined_variables.keys().cloned().collect();

    // Step 1: Process liquid includes first
    let content_with_includes = process_liquid_tags(content, &keys, includes)?;

    // Step 2: Convert markdown to HTML if needed
    let is_markdown = content_item.get("file_type").map_or(true, |ft| ft == "md");
    let html_content = if is_markdown {
        markdown_to_html(&content_with_includes)
    } else {
        content_with_includes
    };

    // Step 3: Process template variables
    let processed_content = process_template_tags(&html_content, &combined_variables)?;

    Ok(processed_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
        // The markdown processor just converts line breaks to <br />, doesn't add paragraphs
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
