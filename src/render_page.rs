use crate::error::Result;
use crate::layout::{insert_body_into_layout, load_layout};
use crate::template_processors::liquid::process_liquid_tags;
use crate::template_processors::markdown::markdown_to_html;
use crate::template_processors::process_template_tags;
use crate::types::{TemplateIncludes, Variables};
use crate::write::write_html_to_file;

/// Processes a page through the template pipeline:
/// 1. Converts markdown to HTML (if content is markdown)
/// 2. Processes liquid includes
/// 3. Inserts into secondary layout (if specified)
/// 4. Inserts into main layout
/// 5. Processes template tags
/// 6. Writes to file
pub fn render_page(
    body: &str,
    directory: &str,
    slug: &str,
    layout: &str,
    includes: &TemplateIncludes,
    variables: &Variables,
) -> Result<()> {
    let file_name = directory.to_string() + slug + ".html";
    let keys: Vec<String> = variables.keys().cloned().collect();

    // Check if the content is markdown or HTML or handlebars
    let is_markdown = variables.get("file_type").map_or(true, |ft| ft == "md");
    let is_handlebars = variables.get("file_type").map_or(false, |ft| ft == "hbs");

    // Process the body content first
    let processed_body = if is_markdown {
        markdown_to_html(body)
    } else {
        // For handlebars files, process the template variables first
        if is_handlebars {
            process_template_tags(body, variables)?
        } else {
            body.to_string()
        }
    };

    // Process liquid includes in the content
    let processed_content = process_liquid_tags(&processed_body, &keys, includes)?;

    // Apply secondary layout if specified in front matter
    let content_with_layout = if let Some(secondary_layout_name) = variables.get("layout") {
        let layout_path = format!(
            "./sites/{}/layouts/{}.html",
            variables.get("site_name").unwrap_or(&"".to_string()),
            secondary_layout_name
        );

        if let Ok(secondary_layout) = load_layout(&layout_path) {
            // First insert the content into the layout
            let layout_with_content =
                insert_body_into_layout(&secondary_layout, &processed_content)?;
            // Then process any template variables in the combined result
            process_template_tags(&layout_with_content, variables)?
        } else {
            processed_content
        }
    } else {
        processed_content
    };

    // Apply main layout and process template tags
    let html = insert_body_into_layout(layout, &content_with_layout)
        .and_then(|html| process_template_tags(&html, variables))?;

    write_html_to_file(&file_name, &html)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // ... existing code ...
}
