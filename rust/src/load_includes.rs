use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn load_liquid_includes() -> Result<HashMap<String, String>> {
    let path = Path::new("../_includes");
    let mut templates = HashMap::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                if extension == "liquid" {
                    if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
                        let path_clone = path.clone();
                        let contents = fs::read_to_string(path_clone)?;
                        templates.insert(filename.to_string(), contents);
                    }
                }
            }
        }
    }

    Ok(templates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use std::collections::BTreeMap;
    use serde_json;

    #[test]
    fn test_load_liquid_includes() {
        let templates = load_liquid_includes().unwrap();
        let sorted_templates: BTreeMap<_, _> = templates.into_iter().collect();
        let templates_json = serde_json::to_string_pretty(&sorted_templates).unwrap();
        // Use assert_snapshot! to compare the output against the stored snapshot
        assert_snapshot!(templates_json);
    }
}
