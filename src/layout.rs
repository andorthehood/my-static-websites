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
/// Returns an error if the layout is not configured or cannot be loaded.
pub fn load_and_render_pagination_layout(
    site_name: &str,
    layout_name: Option<&String>,
    context_variables: &Variables,
    includes: &TemplateIncludes,
    config: &SiteConfig,
) -> Result<String> {
    let layout_name = layout_name.ok_or_else(|| {
        crate::error::Error::Liquid("No pagination layout configured".to_string())
    })?;
    
    let layout_path = build_layout_path(site_name, layout_name, config);
    
    let layout_content = load_layout(&layout_path).map_err(|err| {
        crate::error::Error::Liquid(format!(
            "Pagination layout '{}' was not found at '{}': {}",
            layout_name, layout_path, err
        ))
    })?;

    // Process the layout content with all template tags and variables
    process_template_tags(&layout_content, context_variables, Some(includes), None)
        .map_err(|err| {
            crate::error::Error::Liquid(format!(
                "Failed to render pagination layout '{}': {}",
                layout_name, err
            ))
        })
}
