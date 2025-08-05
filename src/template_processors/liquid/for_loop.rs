use crate::error::{Error, Result};
use std::collections::HashMap;

/// Processes Liquid for loops by expanding them into individual variable references
///
/// Converts:
/// ```liquid
/// {% for person in people %}
///   Name: {{ person.name }}
/// {% endfor %}
/// ```
///
/// Into:
/// ```
/// Name: {{ people.0.name }}
/// Name: {{ people.1.name }}
/// Name: {{ people.2.name }}
/// ```
///
/// This allows the existing variable replacement system to handle the actual substitution.
/// Supports nested loops by recursively processing until no more for loops remain.
pub fn process_liquid_for_loops(
    template: &str,
    variables: &HashMap<String, String>,
) -> Result<String> {
    let mut current_template = template.to_string();

    // Keep processing until no more for loops are found
    loop {
        let processed = process_single_pass(&current_template, variables)?;

        // If no changes were made, we're done
        if processed == current_template {
            break;
        }

        current_template = processed;
    }

    Ok(current_template)
}

/// Processes a single pass of for loop expansion
fn process_single_pass(template: &str, variables: &HashMap<String, String>) -> Result<String> {
    let mut result = String::new();
    let mut chars = template.chars().peekable();

    while let Some(current) = chars.next() {
        if current == '{' && chars.peek() == Some(&'%') {
            chars.next(); // Skip '%'

            // Check if this is a for loop
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

            if tag_content.starts_with("for ") {
                // Parse the for loop
                let for_content = &tag_content[4..]; // Remove "for "
                let parts: Vec<&str> = for_content.split(" in ").collect();

                if parts.len() != 2 {
                    return Err(Error::Liquid("Invalid for loop syntax".to_string()));
                }

                let item_var = parts[0].trim();
                let collection_var = parts[1].trim();

                // Find the loop body until {% endfor %}
                let mut loop_body = String::new();
                let mut depth = 1;

                while depth > 0 {
                    if chars.peek().is_none() {
                        return Err(Error::Liquid(
                            "Unclosed for loop - missing {% endfor %}".to_string(),
                        ));
                    }

                    let c = chars.next().unwrap();

                    if c == '{' && chars.peek() == Some(&'%') {
                        chars.next(); // Skip '%'
                        let mut inner_tag = String::new();

                        // Skip whitespace
                        while let Some(&ch) = chars.peek() {
                            if !ch.is_whitespace() {
                                break;
                            }
                            chars.next();
                        }

                        // Collect tag content
                        while let Some(ch) = chars.next() {
                            if ch == '%' && chars.peek() == Some(&'}') {
                                chars.next(); // Skip '}'
                                break;
                            }
                            inner_tag.push(ch);
                        }

                        let inner_tag = inner_tag.trim();
                        if inner_tag.starts_with("for ") {
                            depth += 1;
                        } else if inner_tag == "endfor" {
                            depth -= 1;
                        }

                        if depth > 0 {
                            loop_body.push_str("{% ");
                            loop_body.push_str(&inner_tag);
                            loop_body.push_str(" %}");
                        }
                    } else if depth > 0 {
                        loop_body.push(c);
                    }
                }

                // Expand the loop
                let expanded = expand_for_loop(item_var, collection_var, &loop_body, variables)?;
                result.push_str(&expanded);
            } else {
                // Not a for loop, keep the original tag
                result.push_str("{% ");
                result.push_str(&tag_content);
                result.push_str(" %}");
            }
        } else {
            result.push(current);
        }
    }

    Ok(result)
}

fn expand_for_loop(
    item_var: &str,
    collection_var: &str,
    loop_body: &str,
    variables: &HashMap<String, String>,
) -> Result<String> {
    // Find how many items are in the collection by checking variables
    let mut max_index = 0;
    let collection_prefix = format!("{collection_var}.");

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

    // If no indexed items found, return empty string
    if max_index == 0 {
        return Ok(String::new());
    }

    // Expand the loop body for each item
    let mut result = String::new();
    for i in 0..max_index {
        // Replace variable references with all possible spacing formats
        let mut expanded_body = loop_body.to_string();

        // Process unless tags with current forloop context
        let mut forloop_vars = HashMap::new();
        forloop_vars.insert(
            "forloop.last".to_string(),
            if i == max_index - 1 { "true" } else { "false" }.to_string(),
        );

        // Process unless tags in this iteration with the correct forloop context
        expanded_body = super::unless::process_liquid_unless_tags(&expanded_body, &forloop_vars)?;

        // Handle different spacing patterns for variable references
        let patterns_to_replace = vec![
            // No spaces: {{item.
            (
                format!("{{{{{item_var}."),
                format!("{{{{{collection_var}.{i}."),
            ),
            // Spaces: {{ item.
            (
                format!("{{ {item_var}."),
                format!("{{ {collection_var}.{i}."),
            ),
            // No spaces: {{item}}
            (
                format!("{{{{{item_var}}}}}"),
                format!("{{{{{collection_var}.{i}}}}}"),
            ),
            // Spaces: {{ item }}
            (
                format!("{{ {item_var} }}"),
                format!("{{ {collection_var}.{i} }}"),
            ),
        ];

        for (pattern, replacement) in patterns_to_replace {
            expanded_body = expanded_body.replace(&pattern, &replacement);
        }
        // Also replace for loop references like "for member in group.members"
        expanded_body = expanded_body.replace(
            &format!(" in {item_var}."),
            &format!(" in {collection_var}.{i}."),
        );

        result.push_str(&expanded_body);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_for_loop() {
        let mut variables = HashMap::new();
        variables.insert("people.0.name".to_string(), "Alice".to_string());
        variables.insert("people.1.name".to_string(), "Bob".to_string());
        variables.insert("people.2.name".to_string(), "Charlie".to_string());

        let template = "{% for person in people %}Name: {{ person.name }}\n{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        let expected =
            "Name: {{ people.0.name }}\nName: {{ people.1.name }}\nName: {{ people.2.name }}\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_loop_with_multiple_properties() {
        let mut variables = HashMap::new();
        variables.insert("users.0.name".to_string(), "Alice".to_string());
        variables.insert("users.0.age".to_string(), "25".to_string());
        variables.insert("users.1.name".to_string(), "Bob".to_string());
        variables.insert("users.1.age".to_string(), "30".to_string());

        let template =
            "{% for user in users %}{{ user.name }} is {{ user.age }} years old.\n{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        let expected = "{{ users.0.name }} is {{ users.0.age }} years old.\n{{ users.1.name }} is {{ users.1.age }} years old.\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_loop_empty_collection() {
        let variables = HashMap::new();

        let template = "{% for item in empty %}{{ item.value }}{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        assert_eq!(result, "");
    }

    #[test]
    fn test_for_loop_with_surrounding_text() {
        let mut variables = HashMap::new();
        variables.insert("items.0.title".to_string(), "First".to_string());
        variables.insert("items.1.title".to_string(), "Second".to_string());

        let template = "Before\n{% for item in items %}Item: {{ item.title }}\n{% endfor %}After";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        let expected = "Before\nItem: {{ items.0.title }}\nItem: {{ items.1.title }}\nAfter";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_non_for_loop_tags_unchanged() {
        let variables = HashMap::new();

        let template = "{% if condition %}Hello{% endif %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        assert_eq!(result, "{% if condition %}Hello{% endif %}");
    }

    #[test]
    fn test_nested_for_loops() {
        let mut variables = HashMap::new();
        variables.insert("groups.0.members.0.name".to_string(), "Alice".to_string());
        variables.insert("groups.0.members.1.name".to_string(), "Bob".to_string());
        variables.insert("groups.1.members.0.name".to_string(), "Charlie".to_string());

        let template = "{% for group in groups %}{% for member in group.members %}{{ member.name }} {% endfor %}{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        // Should be fully expanded with both loop levels processed
        let expected = "{{ groups.0.members.0.name }} {{ groups.0.members.1.name }} {{ groups.1.members.0.name }} ";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_for_loops_with_text() {
        let mut variables = HashMap::new();
        variables.insert("departments.0.name".to_string(), "Engineering".to_string());
        variables.insert(
            "departments.0.employees.0.name".to_string(),
            "Alice".to_string(),
        );
        variables.insert(
            "departments.0.employees.1.name".to_string(),
            "Bob".to_string(),
        );
        variables.insert("departments.1.name".to_string(), "Sales".to_string());
        variables.insert(
            "departments.1.employees.0.name".to_string(),
            "Charlie".to_string(),
        );

        let template = "{% for dept in departments %}Department: {{ dept.name }}\n{% for emp in dept.employees %}  - {{ emp.name }}\n{% endfor %}{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        let expected = "Department: {{ departments.0.name }}\n  - {{ departments.0.employees.0.name }}\n  - {{ departments.0.employees.1.name }}\nDepartment: {{ departments.1.name }}\n  - {{ departments.1.employees.0.name }}\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_loop_with_string_array() {
        let mut variables = HashMap::new();
        variables.insert("colors.0".to_string(), "red".to_string());
        variables.insert("colors.1".to_string(), "green".to_string());
        variables.insert("colors.2".to_string(), "blue".to_string());

        let template = "{% for color in colors %}{{ color }} {% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        let expected = "{{ colors.0 }} {{ colors.1 }} {{ colors.2 }} ";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_invalid_for_loop_syntax() {
        let variables = HashMap::new();

        let template = "{% for item %}{{ item }}{% endfor %}";
        let result = process_liquid_for_loops(template, &variables);

        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_for_loop() {
        let variables = HashMap::new();

        let template = "{% for item in items %}{{ item }}";
        let result = process_liquid_for_loops(template, &variables);

        assert!(result.is_err());
    }

    #[test]
    fn test_forloop_context_processing() {
        let mut variables = HashMap::new();
        variables.insert("items.0".to_string(), "apple".to_string());
        variables.insert("items.1".to_string(), "banana".to_string());

        let template = "{% for item in items %}{{ item }}{% unless forloop.last %},{% endunless %}{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        // Should process unless tags with forloop context and remove comma on last item
        let expected = "{{ items.0 }},{{ items.1 }}";
        assert_eq!(result, expected);
    }
}
