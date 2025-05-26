use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::error::MyError;

pub fn load_liquid_includes(dir_path: &str) -> Result<HashMap<String, String>, MyError> {
    let path = Path::new(dir_path);
    let mut templates = HashMap::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("liquid") {
            if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
                let contents = fs::read_to_string(&path)?;
                templates.insert(filename.to_string(), contents);
            }
        }
    }

    Ok(templates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use serde_json;
    use std::collections::BTreeMap;

    #[test]
    fn test_load_liquid_includes() {
        // Update test to handle Result
        let templates = load_liquid_includes("./_includes").unwrap_or_default();
        let sorted_templates: BTreeMap<_, _> = templates.into_iter().collect();
        let templates_json = serde_json::to_string_pretty(&sorted_templates).unwrap();
        // Use assert_snapshot! to compare the output against the stored snapshot
        assert_snapshot!(templates_json);
    }
}
