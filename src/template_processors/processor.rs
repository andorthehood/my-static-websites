use crate::error::Result;
use crate::template_processors::liquid::{
    process_liquid_assign_tags, process_liquid_conditional_tags, process_liquid_for_loops,
    process_liquid_tags_with_assigns, process_liquid_unless_tags, remove_liquid_variables,
    replace_template_variables,
};
use crate::template_processors::markdown::markdown_to_html;
use crate::types::{ContentItem, TemplateIncludes};
use std::collections::HashMap;

/// Processes template tags in a given input string with optional advanced features.
///
/// This unified function handles:
/// - Liquid conditionals (always)
/// - Liquid includes (when includes are provided)
/// - Markdown to HTML conversion (when content_item with markdown file_type is provided)
/// - Liquid variables (always)
///
/// # Arguments
/// * `input` - The input string containing template tags
/// * `variables` - Variables for template processing
/// * `includes` - Optional liquid includes for {% include %} tags
/// * `content_item` - Optional content metadata for markdown processing and additional variables
///
/// # Returns
/// * `Result<String>` - The processed template or an error if processing fails
pub fn process_template_tags(
    input: &str,
    variables: &HashMap<String, String>,
    includes: Option<&TemplateIncludes>,
    content_item: Option<&ContentItem>,
) -> Result<String> {
    // Create combined variables if content_item is provided
    let mut combined_variables = if let Some(item) = content_item {
        let mut combined = variables.clone();
        combined.extend(item.clone());
        combined
    } else {
        variables.clone()
    };

    let keys: Vec<String> = combined_variables.keys().cloned().collect();

    // Step 1: Process liquid tags (conditionals, assign tags, for loops, and includes if provided)
    let mut result = if let Some(includes) = includes {
        // Process all liquid tags including assigns
        process_liquid_tags_with_assigns(input, &keys, includes, &mut combined_variables)?
    } else {
        // New order: assigns -> for loops -> unless -> if
        let processed_assigns = process_liquid_assign_tags(input, &mut combined_variables)?;
        let processed_for_loops =
            process_liquid_for_loops(&processed_assigns, &combined_variables)?;
        let processed_unless =
            process_liquid_unless_tags(&processed_for_loops, &combined_variables)?;
        
        process_liquid_conditional_tags(&processed_unless, &combined_variables)?
    };

    // Step 2: Convert markdown to HTML if content_item indicates markdown
    if let Some(item) = content_item {
        let is_markdown = item.get("file_type").is_none_or(|ft| ft == "md");
        if is_markdown {
            result = markdown_to_html(&result);
        }
    }

    // Step 3: Process liquid variables
    result = replace_template_variables(&result, &combined_variables)?;
    result = remove_liquid_variables(&result)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_template_tags_simple() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "World".to_string());
        variables.insert("show_greeting".to_string(), "true".to_string());

        let input = "{% if show_greeting %}Hello {{name}}!{% endif %}";
        let result = process_template_tags(input, &variables, None, None)
            .expect("Processing template tags failed");
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_process_template_tags_with_includes() {
        let mut includes = HashMap::new();
        includes.insert("test.liquid".to_string(), "Hello {{ name }}!".to_string());

        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "World".to_string());

        let input = "{% include test.liquid name:\"World\" %}";
        let result = process_template_tags(input, &variables, Some(&includes), None).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_process_template_tags_with_markdown() {
        let includes = HashMap::new();
        let mut content_item = HashMap::new();
        content_item.insert("file_type".to_string(), "md".to_string());
        let variables = HashMap::new();

        let content = "# Test Heading\n\nThis is a paragraph.";
        let result =
            process_template_tags(content, &variables, Some(&includes), Some(&content_item))
                .unwrap();
        // The markdown processor strips line breaks between non-list lines
        assert_eq!(result, "# Test HeadingThis is a paragraph.");
    }

    #[test]
    fn test_process_template_tags_with_content_variables() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "World".to_string());

        let mut content_item = HashMap::new();
        content_item.insert("file_type".to_string(), "html".to_string());
        content_item.insert("greeting".to_string(), "Hello".to_string());

        let input = "{{greeting}} {{name}}!";
        let result = process_template_tags(input, &variables, None, Some(&content_item))
            .expect("Processing template tags failed");
        assert_eq!(result, "Hello World!");
    }
}
