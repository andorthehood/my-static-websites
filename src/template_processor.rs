use std::collections::HashMap;
use crate::error::Result;
use crate::template_processors::liquid::process_liquid_tags;
use crate::template_processors::process_template_tags;
use crate::layout::insert_body_into_layout;
use crate::template_processors::markdown::markdown_to_html;
use crate::write::write_html_to_file;

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
    
    // Process both liquid includes and conditionals in one step
    let keys: Vec<String> = variables.keys().cloned().collect();
    html = process_liquid_tags(&html, &keys, &includes)?;
    
    html = insert_body_into_layout(&layout, &html)?;
    html = process_template_tags(&html, &variables)?;
    write_html_to_file(&file_name, &html)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_page() {
        // This would need a more comprehensive test with temp files
        // For now, we'll rely on the integration tests
    }
} 