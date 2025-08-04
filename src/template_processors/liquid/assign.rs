use crate::error::{Error, Result};
use std::collections::HashMap;

/// Processes Liquid assign tags with filter support
///
/// Converts:
/// ```liquid
/// {% assign active_users = data.users | where: "active", true %}
/// ```
///
/// Into new variables in the variables map that can be used later in the template.
/// Supports the 'where' filter for filtering arrays based on property values.
pub fn process_liquid_assign_tags(
    template: &str,
    variables: &mut HashMap<String, String>,
) -> Result<String> {
    let mut result = String::new();
    let mut chars = template.chars().peekable();

    while let Some(current) = chars.next() {
        if current == '{' && chars.peek() == Some(&'%') {
            chars.next(); // Skip '%'

            // Check if this is an assign tag
            let mut tag_content = String::new();
            let mut found_closing = false;

            // Skip whitespace
            while let Some(&c) = chars.peek() {
                if !c.is_whitespace() {
                    break;
                }
                chars.next();
            }

            // Collect tag content until we find %}
            while let Some(c) = chars.next() {
                if c == '%' && chars.peek() == Some(&'}') {
                    chars.next(); // Skip '}'
                    found_closing = true;
                    break;
                }
                tag_content.push(c);
            }

            if !found_closing {
                return Err(Error::Liquid("Unclosed liquid tag".to_string()));
            }

            let tag_content = tag_content.trim();

            if tag_content.starts_with("assign ") {
                // Parse the assign statement
                let assign_content = &tag_content[7..]; // Remove "assign "
                process_assign_statement(assign_content, variables)?;
                // Assign tags are removed from output (they don't render anything)
            } else {
                // Not an assign tag, keep the original tag
                result.push_str("{% ");
                result.push_str(tag_content);
                result.push_str(" %}");
            }
        } else {
            result.push(current);
        }
    }

    Ok(result)
}

fn process_assign_statement(
    statement: &str,
    variables: &mut HashMap<String, String>,
) -> Result<()> {
    // Parse: variable_name = source | filter: args
    let parts: Vec<&str> = statement.split('=').collect();
    if parts.len() != 2 {
        return Err(Error::Liquid("Invalid assign syntax".to_string()));
    }

    let variable_name = parts[0].trim();
    let expression = parts[1].trim();

    // Check if there's a filter
    if let Some(pipe_pos) = expression.find('|') {
        let source = expression[..pipe_pos].trim();
        let filter_part = expression[pipe_pos + 1..].trim();

        // Process the filter
        let filtered_result = apply_filter(source, filter_part, variables)?;

        // Store filtered results as indexed variables
        for (index, item) in filtered_result.iter().enumerate() {
            for (key, value) in item {
                let full_key = format!("{}.{}.{}", variable_name, index, key);
                variables.insert(full_key, value.clone());
            }
        }
    } else {
        // No filter, direct assignment
        if let Some(value) = resolve_variable_value(expression, variables) {
            variables.insert(variable_name.to_string(), value);
        }
    }

    Ok(())
}

fn apply_filter(
    source: &str,
    filter_expression: &str,
    variables: &HashMap<String, String>,
) -> Result<Vec<HashMap<String, String>>> {
    // Parse filter: "where: 'property', value"
    let filter_parts: Vec<&str> = filter_expression.split(':').collect();
    if filter_parts.len() != 2 {
        return Err(Error::Liquid("Invalid filter syntax".to_string()));
    }

    let filter_name = filter_parts[0].trim();
    let filter_args = filter_parts[1].trim();

    match filter_name {
        "where" => apply_where_filter(source, filter_args, variables),
        _ => Err(Error::Liquid(format!("Unknown filter: {}", filter_name))),
    }
}

fn apply_where_filter(
    source: &str,
    args: &str,
    variables: &HashMap<String, String>,
) -> Result<Vec<HashMap<String, String>>> {
    // Parse args: "property", value or 'property', value
    let args = args.trim();
    let parts: Vec<&str> = args.split(',').collect();
    if parts.len() != 2 {
        return Err(Error::Liquid(
            "where filter requires exactly 2 arguments".to_string(),
        ));
    }

    let property = parts[0].trim().trim_matches('"').trim_matches('\'');
    let target_value = parts[1].trim().trim_matches('"').trim_matches('\'');

    // Get all items from the source array
    let source_items = get_array_items(source, variables);
    let mut filtered_items = Vec::new();

    for item in source_items {
        let matches = if target_value == "nil" {
            // For nil, match items that either don't have the property or have it set to nil/empty
            match item.get(property) {
                None => true,                                      // Property doesn't exist
                Some(value) => value.is_empty() || value == "nil", // Property is empty or explicitly nil
            }
        } else {
            // Regular matching - property must exist and match exactly
            match item.get(property) {
                Some(item_value) => item_value == target_value,
                None => false,
            }
        };

        if matches {
            filtered_items.push(item);
        }
    }

    Ok(filtered_items)
}

fn get_array_items(
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
            if let Some(stripped) = key.strip_prefix(&format!("{}.", source)) {
                if let Some(remainder) = stripped.strip_prefix(&format!("{}.", current_index)) {
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

fn resolve_variable_value(expression: &str, variables: &HashMap<String, String>) -> Option<String> {
    variables.get(expression).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_assign() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "Alice".to_string());

        let template = "{% assign user_name = name %}Hello {{ user_name }}!";
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "Hello {{ user_name }}!");
        assert_eq!(variables.get("user_name"), Some(&"Alice".to_string()));
    }

    #[test]
    fn test_where_filter() {
        let mut variables = HashMap::new();
        variables.insert("users.0.name".to_string(), "Alice".to_string());
        variables.insert("users.0.active".to_string(), "true".to_string());
        variables.insert("users.1.name".to_string(), "Bob".to_string());
        variables.insert("users.1.active".to_string(), "false".to_string());
        variables.insert("users.2.name".to_string(), "Charlie".to_string());
        variables.insert("users.2.active".to_string(), "true".to_string());

        let template = "{% assign active_users = users | where: \"active\", \"true\" %}";
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "");

        // Check that filtered results are stored
        assert_eq!(
            variables.get("active_users.0.name"),
            Some(&"Alice".to_string())
        );
        assert_eq!(
            variables.get("active_users.0.active"),
            Some(&"true".to_string())
        );
        assert_eq!(
            variables.get("active_users.1.name"),
            Some(&"Charlie".to_string())
        );
        assert_eq!(
            variables.get("active_users.1.active"),
            Some(&"true".to_string())
        );

        // Bob should not be in the filtered results
        assert_eq!(variables.get("active_users.2.name"), None);
    }

    #[test]
    fn test_where_filter_single_quotes() {
        let mut variables = HashMap::new();
        variables.insert("items.0.type".to_string(), "featured".to_string());
        variables.insert("items.0.title".to_string(), "Featured Article".to_string());
        variables.insert("items.1.type".to_string(), "regular".to_string());
        variables.insert("items.1.title".to_string(), "Regular Article".to_string());

        let template = "{% assign featured_items = items | where: 'type', 'featured' %}";
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "");
        assert_eq!(
            variables.get("featured_items.0.title"),
            Some(&"Featured Article".to_string())
        );
        assert_eq!(variables.get("featured_items.1.title"), None);
    }

    #[test]
    fn test_assign_with_surrounding_text() {
        let mut variables = HashMap::new();
        variables.insert("data.users.0.status".to_string(), "active".to_string());
        variables.insert("data.users.0.name".to_string(), "Alice".to_string());
        variables.insert("data.users.1.status".to_string(), "inactive".to_string());
        variables.insert("data.users.1.name".to_string(), "Bob".to_string());

        let template =
            "Before\n{% assign active_users = data.users | where: \"status\", \"active\" %}\nAfter";
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "Before\n\nAfter");
        assert_eq!(
            variables.get("active_users.0.name"),
            Some(&"Alice".to_string())
        );
    }

    #[test]
    fn test_invalid_assign_syntax() {
        let mut variables = HashMap::new();

        let template = "{% assign invalid syntax %}";
        let result = process_liquid_assign_tags(template, &mut variables);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_filter() {
        let mut variables = HashMap::new();
        variables.insert("items.0.name".to_string(), "test".to_string());

        let template = "{% assign result = items | unknown_filter: \"arg\" %}";
        let result = process_liquid_assign_tags(template, &mut variables);

        assert!(result.is_err());
    }

    #[test]
    fn test_non_assign_tags_unchanged() {
        let mut variables = HashMap::new();

        let template = "{% if condition %}Hello{% endif %}";
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "{% if condition %}Hello{% endif %}");
    }

    #[test]
    fn test_where_filter_with_nil_keyword() {
        let mut variables = HashMap::new();
        // Post with my_prop explicitly set to nil
        variables.insert("site.posts.0.title".to_string(), "Post 1".to_string());
        variables.insert("site.posts.0.my_prop".to_string(), "nil".to_string());

        // Post with my_prop set to empty string
        variables.insert("site.posts.1.title".to_string(), "Post 2".to_string());
        variables.insert("site.posts.1.my_prop".to_string(), "".to_string());

        // Post without my_prop defined at all
        variables.insert("site.posts.2.title".to_string(), "Post 3".to_string());

        // Post with my_prop set to a value
        variables.insert("site.posts.3.title".to_string(), "Post 4".to_string());
        variables.insert("site.posts.3.my_prop".to_string(), "some_value".to_string());

        let template = "{% assign filtered_posts = site.posts | where: 'my_prop', nil %}";
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "");

        // Should match posts 1, 2, and 3 (nil, empty, and undefined)
        assert_eq!(
            variables.get("filtered_posts.0.title"),
            Some(&"Post 1".to_string())
        );
        assert_eq!(
            variables.get("filtered_posts.1.title"),
            Some(&"Post 2".to_string())
        );
        assert_eq!(
            variables.get("filtered_posts.2.title"),
            Some(&"Post 3".to_string())
        );

        // Should not match post 4
        assert_eq!(variables.get("filtered_posts.3.title"), None);
    }

    #[test]
    fn test_where_filter_nil_with_quotes() {
        let mut variables = HashMap::new();
        variables.insert("posts.0.title".to_string(), "Post 1".to_string());
        // Post without the property

        variables.insert("posts.1.title".to_string(), "Post 2".to_string());
        variables.insert("posts.1.featured".to_string(), "true".to_string());

        let template = r#"{% assign non_featured = posts | where: "featured", nil %}"#;
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "");

        // Should match post 1 (no featured property)
        assert_eq!(
            variables.get("non_featured.0.title"),
            Some(&"Post 1".to_string())
        );

        // Should not match post 2 (has featured property)
        assert_eq!(variables.get("non_featured.1.title"), None);
    }

    #[test]
    fn test_where_filter_nil_vs_string_nil() {
        let mut variables = HashMap::new();

        // Post with property explicitly set to string "nil"
        variables.insert("items.0.name".to_string(), "Item 1".to_string());
        variables.insert("items.0.status".to_string(), "nil".to_string());

        // Post with property set to actual nil/empty
        variables.insert("items.1.name".to_string(), "Item 2".to_string());
        variables.insert("items.1.status".to_string(), "".to_string());

        // Post with property set to "active"
        variables.insert("items.2.name".to_string(), "Item 3".to_string());
        variables.insert("items.2.status".to_string(), "active".to_string());

        // Filter for nil should match both explicit "nil" and empty string
        let template = "{% assign nil_items = items | where: 'status', nil %}";
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "");

        // Should match both items with nil/empty status
        assert_eq!(
            variables.get("nil_items.0.name"),
            Some(&"Item 1".to_string())
        );
        assert_eq!(
            variables.get("nil_items.1.name"),
            Some(&"Item 2".to_string())
        );

        // Should not match item with "active" status
        assert_eq!(variables.get("nil_items.2.name"), None);
    }
}
