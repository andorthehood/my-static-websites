use std::collections::HashMap;

/// Gets all items from an array-like structure in the variables `HashMap`
///
/// For a source like "users", this function looks for keys like:
/// - "users.0.name", "users.0.age" (first user)
/// - "users.1.name", "users.1.age" (second user)
///
/// And returns a Vec of `HashMaps`, where each `HashMap` contains the properties
/// of one item (e.g., {"name": "Alice", "age": "25"})
pub fn get_array_items(
    source: &str,
    variables: &HashMap<String, String>,
) -> Vec<HashMap<String, String>> {
    let mut items = Vec::new();
    let mut current_index = 0;

    loop {
        let mut item = HashMap::new();
        let mut found_any = false;

        // Look for all properties of the current item
        for (key, value) in variables {
            if let Some(stripped) = key.strip_prefix(&format!("{source}.")) {
                if let Some(remainder) = stripped.strip_prefix(&format!("{current_index}.")) {
                    item.insert(remainder.to_string(), value.clone());
                    found_any = true;
                }
            }
        }

        if !found_any {
            break;
        }

        items.push(item);
        current_index += 1;
    }

    items
}

/// Resolves a variable value from either a literal string or variable lookup
///
/// If the expression is a quoted string, returns the literal value.
/// Otherwise, tries to look up the expression as a variable name.
pub fn resolve_variable_value(
    expression: &str,
    variables: &HashMap<String, String>,
) -> Option<String> {
    // Check if the expression is a quoted string literal
    if (expression.starts_with('"') && expression.ends_with('"'))
        || (expression.starts_with('\'') && expression.ends_with('\''))
    {
        // Remove quotes and return the literal value
        Some(expression[1..expression.len() - 1].to_string())
    } else {
        // Try to look up as a variable name
        variables.get(expression).cloned()
    }
}

/// Finds the maximum index for a collection by scanning variable keys
///
/// For a collection "items", this scans for keys like "items.0", "items.1", etc.
/// and returns the count of items found.
pub fn find_collection_size(collection_name: &str, variables: &HashMap<String, String>) -> usize {
    let mut max_index = 0;
    let collection_prefix = format!("{collection_name}.");

    for key in variables.keys() {
        if key.starts_with(&collection_prefix) {
            // Extract the index from keys like "people.0.name", "people.1.age", etc.
            // or from keys like "colors.0", "colors.1" for string arrays
            let suffix = &key[collection_prefix.len()..];

            let index_str = if let Some(dot_pos) = suffix.find('.') {
                // Object properties: "colors.0.name" -> "0"
                &suffix[..dot_pos]
            } else {
                // String arrays: "colors.0" -> "0"
                suffix
            };

            if let Ok(index) = index_str.parse::<usize>() {
                max_index = max_index.max(index + 1);
            }
        }
    }

    max_index
}

/// Clears variables with a given prefix from the `HashMap`
/// Used when overriding filtered results in assign statements
pub fn clear_variables_with_prefix(variables: &mut HashMap<String, String>, prefix: &str) {
    let full_prefix = format!("{prefix}.");
    variables.retain(|key, _| !key.starts_with(&full_prefix));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_array_items() {
        let mut variables = HashMap::new();
        variables.insert("users.0.name".to_string(), "Alice".to_string());
        variables.insert("users.0.age".to_string(), "25".to_string());
        variables.insert("users.1.name".to_string(), "Bob".to_string());
        variables.insert("users.1.age".to_string(), "30".to_string());

        let items = get_array_items("users", &variables);
        assert_eq!(items.len(), 2);

        assert_eq!(items[0].get("name"), Some(&"Alice".to_string()));
        assert_eq!(items[0].get("age"), Some(&"25".to_string()));
        assert_eq!(items[1].get("name"), Some(&"Bob".to_string()));
        assert_eq!(items[1].get("age"), Some(&"30".to_string()));
    }

    #[test]
    fn test_get_array_items_empty() {
        let variables = HashMap::new();
        let items = get_array_items("empty", &variables);
        assert!(items.is_empty());
    }

    #[test]
    fn test_resolve_variable_value_literal() {
        let variables = HashMap::new();

        let result = resolve_variable_value("\"hello\"", &variables);
        assert_eq!(result, Some("hello".to_string()));

        let result = resolve_variable_value("'world'", &variables);
        assert_eq!(result, Some("world".to_string()));
    }

    #[test]
    fn test_resolve_variable_value_lookup() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "Alice".to_string());

        let result = resolve_variable_value("name", &variables);
        assert_eq!(result, Some("Alice".to_string()));

        let result = resolve_variable_value("missing", &variables);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_collection_size() {
        let mut variables = HashMap::new();
        variables.insert("items.0".to_string(), "first".to_string());
        variables.insert("items.1.title".to_string(), "second".to_string());
        variables.insert("items.2.title".to_string(), "third".to_string());

        let size = find_collection_size("items", &variables);
        assert_eq!(size, 3);

        let empty_size = find_collection_size("empty", &variables);
        assert_eq!(empty_size, 0);
    }

    #[test]
    fn test_clear_variables_with_prefix() {
        let mut variables = HashMap::new();
        variables.insert("active_users.0.name".to_string(), "Alice".to_string());
        variables.insert("active_users.1.name".to_string(), "Bob".to_string());
        variables.insert("other.value".to_string(), "keep".to_string());

        clear_variables_with_prefix(&mut variables, "active_users");

        assert_eq!(variables.get("active_users.0.name"), None);
        assert_eq!(variables.get("active_users.1.name"), None);
        assert_eq!(variables.get("other.value"), Some(&"keep".to_string()));
    }
}
