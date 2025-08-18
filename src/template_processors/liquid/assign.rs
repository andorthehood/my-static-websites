use super::utils::{
    clear_variables_with_prefix, extract_tag_parameter, get_array_items, parse_assignment,
    read_until_closing_tag, resolve_variable_value, split_respecting_quotes, trim_quotes,
};
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

            // Use utility functions for tag processing
            let tag_content = read_until_closing_tag(&mut chars)?;
            let trimmed_content = tag_content.trim();

            if let Some(assign_content) = extract_tag_parameter(trimmed_content, "assign") {
                // Parse the assign statement
                process_assign_statement(&assign_content, variables)?;
                // Assign tags are removed from output (they don't render anything)
            } else {
                // Not an assign tag, keep the original tag
                result.push_str("{% ");
                result.push_str(trimmed_content);
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
    // Parse: variable_name = source | filter: args using utility function
    let (variable_name, expression) = parse_assignment(statement)
        .ok_or_else(|| Error::Liquid("Invalid assign syntax".to_string()))?;

    // Check if there's a filter
    if let Some(pipe_pos) = expression.find('|') {
        let source = expression[..pipe_pos].trim();
        let filter_part = expression[pipe_pos + 1..].trim();

        // Process the filter
        let filtered_result = apply_filter(source, filter_part, variables)?;

        // Clear any existing variables with the same prefix before storing new results
        clear_variables_with_prefix(variables, &variable_name);

        // Store filtered results as indexed variables
        for (index, item) in filtered_result.iter().enumerate() {
            for (key, value) in item {
                let full_key = format!("{variable_name}.{index}.{key}");
                variables.insert(full_key, value.clone());
            }
        }
    } else {
        // No filter, direct assignment
        if let Some(value) = resolve_variable_value(&expression, variables) {
            variables.insert(variable_name.clone(), value);
        }
    }

    Ok(())
}

fn apply_filter(
    source: &str,
    filter_expression: &str,
    variables: &HashMap<String, String>,
) -> Result<Vec<HashMap<String, String>>> {
    // Parse filter: "name: args"
    let (filter_name, filter_args) = super::utils::parse_filter_invocation(filter_expression)
        .ok_or_else(|| Error::Liquid("Invalid filter syntax".to_string()))?;

    match filter_name.as_str() {
        "where" => apply_where_filter(source, &filter_args, variables),
        _ => Err(Error::Liquid(format!("Unknown filter: {filter_name}"))),
    }
}

fn apply_where_filter(
    source: &str,
    args: &str,
    variables: &HashMap<String, String>,
) -> Result<Vec<HashMap<String, String>>> {
    // Parse args: "property", value or 'property', value
    let parts = split_respecting_quotes(args);
    if parts.len() != 2 {
        return Err(Error::Liquid(
            "where filter requires exactly 2 arguments".to_string(),
        ));
    }

    let property = trim_quotes(&parts[0]);
    let target_value = trim_quotes(&parts[1]);

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

    #[test]
    fn test_multiple_assign_tags_override() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "Alice".to_string());
        variables.insert("age".to_string(), "25".to_string());

        let template = "{% assign user = name %}{% assign user = age %}Hello {{ user }}!";
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "Hello {{ user }}!");
        // The second assign should override the first one
        assert_eq!(variables.get("user"), Some(&"25".to_string()));
    }

    #[test]
    fn test_assign_with_dot_in_variable_name() {
        let mut variables = HashMap::new();
        variables.insert("test".to_string(), "value".to_string());

        let template = "{% assign forloop.last = \"true\" %}Result: {{ forloop.last }}";
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "Result: {{ forloop.last }}");
        // Check that the variable with dot was stored correctly
        assert_eq!(variables.get("forloop.last"), Some(&"true".to_string()));

        println!("Variables after assign:");
        for (key, value) in &variables {
            if key.starts_with("forloop") {
                println!("  {} = {}", key, value);
            }
        }
    }

    #[test]
    fn test_multiple_filtered_assign_tags_override() {
        let mut variables = HashMap::new();

        // First set of users (3 active users)
        variables.insert("users.0.name".to_string(), "Alice".to_string());
        variables.insert("users.0.active".to_string(), "true".to_string());
        variables.insert("users.1.name".to_string(), "Bob".to_string());
        variables.insert("users.1.active".to_string(), "false".to_string());
        variables.insert("users.2.name".to_string(), "Carol".to_string());
        variables.insert("users.2.active".to_string(), "true".to_string());
        variables.insert("users.3.name".to_string(), "Dan".to_string());
        variables.insert("users.3.active".to_string(), "true".to_string());

        // Second set (only 1 active admin)
        variables.insert("admins.0.name".to_string(), "Charlie".to_string());
        variables.insert("admins.0.active".to_string(), "true".to_string());
        variables.insert("admins.1.name".to_string(), "David".to_string());
        variables.insert("admins.1.active".to_string(), "false".to_string());

        let template = r#"{% assign active_people = users | where: "active", "true" %}{% assign active_people = admins | where: "active", "true" %}"#;
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();

        assert_eq!(result, "");

        // The second assign should override the first one - should only have 1 admin, not 3 users
        assert_eq!(
            variables.get("active_people.0.name"),
            Some(&"Charlie".to_string())
        );

        // Should NOT have any variables from the first assignment (Alice, Carol, Dan)
        // This is the key test - if override doesn't work, we'd still have active_people.1 and active_people.2
        assert_eq!(variables.get("active_people.1.name"), None);
        assert_eq!(variables.get("active_people.2.name"), None);
    }

    #[test]
    fn test_assign_literal_string_without_filter() {
        let mut variables = HashMap::new();
        let template = r#"{% assign greeting = "Hello" %}X"#;
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();
        assert_eq!(result, "X");
        assert_eq!(variables.get("greeting"), Some(&"Hello".to_string()));
    }

    #[test]
    fn test_invalid_filter_syntax_missing_colon() {
        let mut variables = HashMap::new();
        variables.insert("items.0.name".to_string(), "A".to_string());
        let template = "{% assign res = items | where %}"; // no ':' -> invalid filter syntax
        let result = process_liquid_assign_tags(template, &mut variables);
        assert!(result.is_err());
        if let Err(Error::Liquid(msg)) = result {
            assert!(msg.contains("Invalid filter syntax"));
        } else {
            panic!("expected liquid error");
        }
    }

    #[test]
    fn test_where_filter_wrong_argument_count() {
        let mut variables = HashMap::new();
        variables.insert("items.0.status".to_string(), "active".to_string());
        // only one argument after where -> should error
        let template = r#"{% assign filtered = items | where: "status" %}"#;
        let result = process_liquid_assign_tags(template, &mut variables);
        assert!(result.is_err());
        if let Err(Error::Liquid(msg)) = result {
            assert!(msg.contains("where filter requires exactly 2 arguments"));
        } else {
            panic!("expected liquid error");
        }
    }

    #[test]
    fn test_assign_rhs_variable_missing_results_in_no_insertion() {
        let mut variables = HashMap::new();
        // 'missing' is not present; assignment should not create 'x'
        let template = "{% assign x = missing %}OK";
        let result = process_liquid_assign_tags(template, &mut variables).unwrap();
        assert_eq!(result, "OK");
        assert!(variables.get("x").is_none());
    }
}
