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
    // Determine output extension from source file name:
    // - If original source file is like name.<ext>.liquid -> use <ext> for output
    // - Otherwise, default to .html
    let output_extension = variables
        .get("source_file_name")
        .and_then(|name| {
            name.strip_suffix(".liquid")
                .or_else(|| name.strip_suffix(".html"))
        })
        .and_then(|name_without_liquid| name_without_liquid.rsplit_once('.'))
        .map(|(_, ext)| ext)
        .unwrap_or("html");

    let file_name = format!("{directory}{slug}.{}", output_extension);

    // Helper to build a layout path that defaults to .html when no extension is provided
    fn build_layout_path(site_name: &str, layout_name: &str) -> String {
        let has_extension = std::path::Path::new(layout_name).extension().is_some();
        if has_extension {
            format!(
                "{SITES_BASE_DIR}/{}/{LAYOUTS_SUBDIR}/{}",
                site_name, layout_name
            )
        } else {
            format!(
                "{SITES_BASE_DIR}/{}/{LAYOUTS_SUBDIR}/{}.html",
                site_name, layout_name
            )
        }
    }

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
        let site_name = variables.get("site_name").map(String::as_str).unwrap_or("");
        let layout_path = build_layout_path(site_name, secondary_layout_name);

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
        let site_name = variables.get("site_name").map(String::as_str).unwrap_or("");
        let main_layout_path = build_layout_path(site_name, main_layout_name);
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
    use super::*;
    use std::collections::HashMap;
    use std::fs;

    fn read_file(path: &str) -> String {
        fs::read_to_string(path).expect("failed to read output")
    }

    #[test]
    fn test_render_page_with_missing_secondary_layout_warns_and_falls_back() {
        let body = "Hello BODY";
        let directory = "out/render_tests/";
        let slug = "missing_secondary";
        let main_layout = "[MAIN] {{body}} [/MAIN]";

        let includes: TemplateIncludes = HashMap::new();
        let mut variables: Variables = HashMap::new();
        variables.insert("site_name".into(), "test".into());
        variables.insert("layout".into(), "nonexistent_secondary".into());
        variables.insert("file_type".into(), "html".into());

        render_page(body, directory, slug, main_layout, &includes, &variables)
            .expect("render_page failed");

        let out_path = format!("{}{}.html", directory, slug);
        let content = read_file(&out_path);
        assert_eq!(content, "[MAIN]Hello BODY[/MAIN]");
    }

    #[test]
    fn test_render_page_with_missing_main_layout_override_falls_back_to_param_layout() {
        let body = "X";
        let directory = "out/render_tests/";
        let slug = "missing_main_override";
        let main_layout = "<wrap>{{body}}</wrap>";

        let includes: TemplateIncludes = HashMap::new();
        let mut variables: Variables = HashMap::new();
        variables.insert("site_name".into(), "test".into());
        variables.insert("main_layout".into(), "no_such_layout".into());
        variables.insert("file_type".into(), "html".into());

        render_page(body, directory, slug, main_layout, &includes, &variables)
            .expect("render_page failed");

        let out_path = format!("{}{}.html", directory, slug);
        let content = read_file(&out_path);
        assert_eq!(content, "<wrap>X</wrap>");
    }

    #[test]
    fn test_render_page_with_malformed_include_propagates_error() {
        let body = "{% include bad.liquid name:\"World\" %}";
        let directory = "out/render_tests/";
        let slug = "malformed_include";
        let main_layout = "{{body}}";

        let mut includes: TemplateIncludes = HashMap::new();
        // malformed variable (missing second closing brace) inside include template
        includes.insert("bad.liquid".into(), "Hello, {{ name }!".into());

        let mut variables: Variables = HashMap::new();
        variables.insert("file_type".into(), "html".into());

        let result = render_page(body, directory, slug, main_layout, &includes, &variables);

        assert!(result.is_err());
    }
}
