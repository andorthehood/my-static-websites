use crate::config::SiteConfig;
use crate::error::Result;
use std::fs;
use std::path::Path;

use crate::template_processors::liquid::replace_template_variables;
use crate::template_processors::process_template_tags;
use crate::types::{TemplateIncludes, Variables};
use std::collections::HashMap;

/// Helper to build a layout path that defaults to .html when no extension is provided
pub fn build_layout_path(site_name: &str, layout_name: &str, config: &SiteConfig) -> String {
    let has_extension = std::path::Path::new(layout_name).extension().is_some();
    if has_extension {
        format!(
            "{}/{site_name}/{}/{layout_name}",
            config.sites_base_dir, config.layouts_subdir
        )
    } else {
        format!(
            "{}/{site_name}/{}/{layout_name}.html",
            config.sites_base_dir, config.layouts_subdir
        )
    }
}

pub fn load_layout(file: &str) -> Result<String> {
    let file_path = Path::new(file);
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn insert_body_into_layout(layout: &str, body: &str) -> Result<String> {
    let mut variables = HashMap::new();
    variables.insert("body".to_string(), body.to_string());
    replace_template_variables(layout, &variables)
}

/// Loads and renders a pagination layout with the provided context variables.
/// Returns None if the layout is not configured or cannot be loaded.
/// This allows pagination generators to fall back to hardcoded HTML when needed.
pub fn load_and_render_pagination_layout(
    site_name: &str,
    layout_name: Option<&String>,
    context_variables: &Variables,
    includes: &TemplateIncludes,
    config: &SiteConfig,
) -> Option<String> {
    let layout_name = layout_name?;
    
    let layout_path = build_layout_path(site_name, layout_name, config);
    
    match load_layout(&layout_path) {
        Ok(layout_content) => {
            // Process the layout content with all template tags and variables
            match process_template_tags(&layout_content, context_variables, Some(includes), None) {
                Ok(rendered_content) => Some(rendered_content),
                Err(err) => {
                    eprintln!(
                        "⚠️  Warning: Failed to render pagination layout '{}': {}. Falling back to default markup.",
                        layout_name, err
                    );
                    None
                }
            }
        }
        Err(err) => {
            eprintln!(
                "⚠️  Warning: Pagination layout '{}' was not found at '{}' ({}). Falling back to default markup.",
                layout_name, layout_path, err
            );
            None
        }
    }
}
