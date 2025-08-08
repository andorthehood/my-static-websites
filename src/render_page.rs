use crate::config::{LAYOUTS_SUBDIR, SITES_BASE_DIR};
use crate::error::Result;
use crate::layout::{insert_body_into_layout, load_layout};

use crate::template_processors::markdown::markdown_to_html;
use crate::template_processors::process_template_tags;
use crate::types::{TemplateIncludes, Variables};
use crate::write::write_html_to_file;

/// Processes a page through the template pipeline:
/// 1. Converts markdown to HTML (if content is markdown)
/// 2. Inserts into secondary layout (if specified)
/// 3. Inserts into main layout (can be overridden via `main_layout` in front matter)
/// 4. Processes all template tags (liquid includes + conditionals + variables)
/// 5. Writes to file
pub fn render_page(
    body: &str,
    directory: &str,
    slug: &str,
    layout: &str,
    includes: &TemplateIncludes,
    variables: &Variables,
) -> Result<()> {
    let file_name = format!("{directory}{slug}.html");

    // Check if the content is markdown or HTML or liquid template
    let is_markdown = variables.get("file_type").is_none_or(|ft| ft == "md");
    let is_liquid = variables.get("file_type").is_some_and(|ft| ft == "liquid");

    // Process the body content first
    let processed_body = if is_markdown {
        markdown_to_html(body)
    } else {
        // For liquid files, process the template variables first
        if is_liquid {
            process_template_tags(body, variables, None, None)?
        } else {
            body.to_string()
        }
    };

    // Apply secondary layout if specified in front matter
    let content_with_layout = if let Some(secondary_layout_name) = variables.get("layout") {
        let layout_path = format!(
            "{SITES_BASE_DIR}/{}/{LAYOUTS_SUBDIR}/{secondary_layout_name}.html",
            variables.get("site_name").unwrap_or(&"".to_string())
        );

        if let Ok(secondary_layout) = load_layout(&layout_path) {
            // Insert the content into the secondary layout
            let layout_with_content = insert_body_into_layout(&secondary_layout, &processed_body)?;
            // Process any template variables in the combined result
            process_template_tags(&layout_with_content, variables, None, None)?
        } else {
            eprintln!(
                "⚠️  Warning: Layout '{}' specified in '{}' was not found at '{}'",
                secondary_layout_name, file_name, layout_path
            );
            processed_body
        }
    } else {
        processed_body
    };

    // Determine main layout: allow overriding via front matter `main_layout: <name>`
    let main_layout_content = if let Some(main_layout_name) = variables.get("main_layout") {
        let main_layout_path = format!(
            "{SITES_BASE_DIR}/{}/{LAYOUTS_SUBDIR}/{main_layout_name}.html",
            variables.get("site_name").unwrap_or(&"".to_string())
        );
        match load_layout(&main_layout_path) {
            Ok(custom_main_layout) => custom_main_layout,
            Err(err) => {
                eprintln!(
                    "⚠️  Warning: Main layout '{}' specified in '{}' was not found at '{}' ({}). Falling back to default main layout.",
                    main_layout_name, file_name, main_layout_path, err
                );
                layout.to_string()
            }
        }
    } else {
        layout.to_string()
    };

    // Insert content into main layout (custom if provided, otherwise default)
    let combined_content = insert_body_into_layout(&main_layout_content, &content_with_layout)?;

    // Process all template tags (liquid includes + conditionals + variables) in one go
    let html = process_template_tags(&combined_content, variables, Some(includes), None)?;

    write_html_to_file(&file_name, &html)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // ... existing code ...
}
