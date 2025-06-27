use crate::error::Result;
use crate::layout::insert_body_into_layout;
use crate::template_processors::liquid::process_liquid_tags;
use crate::template_processors::markdown::markdown_to_html;
use crate::template_processors::process_template_tags;
use crate::types::{TemplateIncludes, Variables};
use crate::write::write_html_to_file;

/// Processes a page through the template pipeline:
/// 1. Converts markdown to HTML (if content is markdown)
/// 2. Processes liquid includes
/// 3. Inserts into layout
/// 4. Processes template tags
/// 5. Writes to file
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

    // Check if the content is markdown or HTML
    let is_markdown = variables.get("file_type").map_or(true, |ft| ft == "md");
    let processed_body = if is_markdown {
        markdown_to_html(body)
    } else {
        body.to_string()
    };

    let html = process_liquid_tags(&processed_body, &keys, includes)
        .and_then(|html| insert_body_into_layout(layout, &html))
        .and_then(|html| process_template_tags(&html, variables))?;

    write_html_to_file(&file_name, &html)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // ... existing code ...
}
