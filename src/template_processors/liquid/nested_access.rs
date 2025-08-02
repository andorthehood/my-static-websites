use std::collections::HashMap;

/// Resolves a nested variable path from a flat HashMap structure.
/// Supports dot notation for nested access (e.g., "user.name", "people.0.name").
///
/// The function expects JSON-like data to be flattened into the HashMap with keys like:
/// - "people.0.name" for people[0].name in JSON
/// - "people.0.details.age" for people[0].details.age in JSON
///
/// # Arguments
/// * `path` - The nested path using dot notation (e.g., "people.0.details.name")
/// * `variables` - The flat HashMap containing the data
///
/// # Returns
/// * `Option<String>` - The resolved value or None if not found
pub fn resolve_nested_path(path: &str, variables: &HashMap<String, String>) -> Option<String> {
    variables.get(path).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_nested_path_simple() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "Alice".to_string());

        assert_eq!(
            resolve_nested_path("name", &variables),
            Some("Alice".to_string())
        );
        assert_eq!(resolve_nested_path("missing", &variables), None);
    }

    #[test]
    fn test_resolve_nested_path_dot_notation() {
        let mut variables = HashMap::new();
        variables.insert("user.name".to_string(), "Bob".to_string());
        variables.insert("user.age".to_string(), "30".to_string());

        assert_eq!(
            resolve_nested_path("user.name", &variables),
            Some("Bob".to_string())
        );
        assert_eq!(
            resolve_nested_path("user.age", &variables),
            Some("30".to_string())
        );
    }

    #[test]
    fn test_resolve_nested_path_array_indices() {
        let mut variables = HashMap::new();
        variables.insert("people.0.name".to_string(), "Alice".to_string());
        variables.insert("people.0.age".to_string(), "25".to_string());
        variables.insert("people.1.name".to_string(), "Bob".to_string());

        assert_eq!(
            resolve_nested_path("people.0.name", &variables),
            Some("Alice".to_string())
        );
        assert_eq!(
            resolve_nested_path("people.0.age", &variables),
            Some("25".to_string())
        );
        assert_eq!(
            resolve_nested_path("people.1.name", &variables),
            Some("Bob".to_string())
        );
        assert_eq!(resolve_nested_path("people.2.name", &variables), None);
    }

    #[test]
    fn test_resolve_nested_path_complex() {
        let mut variables = HashMap::new();
        variables.insert("data.0.details.1.value".to_string(), "test".to_string());

        assert_eq!(
            resolve_nested_path("data.0.details.1.value", &variables),
            Some("test".to_string())
        );
    }
}