use crate::types::TemplateIncludes;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Normalizes a path key by converting separators to `/` and removing `.liquid` extension
fn normalize_template_key(relative_path: &str) -> String {
    let normalized = relative_path.replace('\\', "/");
    normalized.strip_suffix(".liquid").unwrap_or(&normalized).to_string()
}

pub fn load_liquid_includes(dir_path: &str) -> TemplateIncludes {
    let base_path = Path::new(dir_path);
    let mut templates = TemplateIncludes::new();

    if !base_path.exists() {
        return templates;
    }

    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        
        // Only process .liquid files
        if path.extension().and_then(|ext| ext.to_str()) != Some("liquid") {
            continue;
        }

        // Calculate relative path from base directory
        if let Ok(relative_path) = path.strip_prefix(base_path) {
            if let Some(relative_str) = relative_path.to_str() {
                if let Ok(contents) = fs::read_to_string(path) {
                    // Normalize the key by converting path separators and removing .liquid extension
                    let normalized_key = normalize_template_key(relative_str);
                    
                    // Check for duplicates and warn if found
                    if templates.contains_key(&normalized_key) {
                        eprintln!("Warning: Duplicate template key '{}' found. File '{}' overwrites previous entry.", normalized_key, relative_str);
                    }
                    
                    templates.insert(normalized_key, contents);
                }
            }
        }
    }

    templates
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use std::collections::BTreeMap;

    #[test]
    fn test_load_liquid_includes() {
        let templates = load_liquid_includes("./sites/test/includes");
        let sorted_templates: BTreeMap<_, _> = templates.into_iter().collect();
        let templates_debug = format!("{:#?}", sorted_templates);
        assert_snapshot!(templates_debug);
    }

    #[test]
    fn test_nested_includes_integration() {
        let templates = load_liquid_includes("./sites/test/includes");
        
        // Verify that nested templates are loaded with correct keys
        assert!(templates.contains_key("components/buttons/cta"));
        assert!(templates.contains_key("components/card"));
        assert!(templates.contains_key("layout/sidebar"));
        
        // Verify that flat templates still work
        assert!(templates.contains_key("test-include"));
        assert!(templates.contains_key("post"));
        
        // Test the content of a nested template
        let cta_template = templates.get("components/buttons/cta");
        assert!(cta_template.is_some());
        assert!(cta_template.unwrap().contains("cta-button"));
    }
}
