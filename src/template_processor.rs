use std::collections::HashMap;
use crate::error::Result;
use crate::liquid::_if::process_liquid_conditional_tags;
use crate::liquid::include::process_liquid_includes;
use crate::handlebars::{replace_template_variables, remove_handlebars_variables};
use crate::layout::insert_body_into_layout;
use crate::markdown::markdown_to_html;
use crate::write::write_html_to_file;

/// Processes all template tags in a given input string.
/// This includes both Liquid conditionals and Handlebars variables.
pub fn process_template_tags(input: &str, variables: &HashMap<String, String>) -> Result<String> {
    let mut result = input.to_string();
    // First process Liquid conditionals
    let keys: Vec<String> = variables.keys().cloned().collect();
    result = process_liquid_conditional_tags(&result, &keys);
    
    // Then process Handlebars variables
    result = replace_template_variables(&result, &variables)?;
    result = remove_handlebars_variables(&result)?;
    
    Ok(result)
}

/// Processes a page through the template pipeline:
/// 1. Converts markdown to HTML
/// 2. Processes liquid includes
/// 3. Inserts into layout
/// 4. Processes template tags
/// 5. Writes to file
pub fn render_page(
    body: &str,
    directory: &str,
    slug: &str,
    layout: &str,
    includes: &HashMap<String, String>,
    variables: &HashMap<String, String>,
) -> Result<()> {
    let mut html = markdown_to_html(&body);
    let file_name = directory.to_string() + &slug + ".html";
    html = process_liquid_includes(&html, &includes)?;
    html = insert_body_into_layout(&layout, &html)?;
    html = process_template_tags(&html, &variables)?;
    write_html_to_file(&file_name, &html)?;
    Ok(())
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
        let result = process_template_tags(input, &variables).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_render_page() {
        // This would need a more comprehensive test with temp files
        // For now, we'll rely on the integration tests
    }
} 