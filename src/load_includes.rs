use crate::types::TemplateIncludes;
use std::fs;
use std::path::Path;

pub fn load_liquid_includes(dir_path: &str) -> TemplateIncludes {
    let path = Path::new(dir_path);
    let mut templates = TemplateIncludes::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("liquid") {
                if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        templates.insert(filename.to_string(), contents);
                    }
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
}
