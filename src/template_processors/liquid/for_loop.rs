use super::utils::{
    find_collection_size, read_until_closing_tag, read_until_endunless, skip_to_endunless,
};
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

            // Read tag content until %}
            let tag_content = read_until_closing_tag(&mut chars)?;
            let tag_content = tag_content.trim().to_string();

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
                let loop_body = super::utils::read_nested_block(&mut chars, "for ", "endfor")?;

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
    // Find how many items are in the collection
    let max_index = find_collection_size(collection_var, variables);

    // If no indexed items found, return empty string
    if max_index == 0 {
        return Ok(String::new());
    }

    // Expand the loop body for each item
    let mut result = String::new();
    for i in 0..max_index {
        // Replace forloop context directly with actual values (no assign tags)
        let is_last = i == max_index - 1;
        let is_first = i == 0;

        let mut expanded_body = loop_body.to_string();

        // Only replace forloop references that are NOT inside nested for loops
        // We do this by only replacing forloop references that are at the same nesting level
        expanded_body = replace_forloop_context_at_current_level(
            &expanded_body,
            is_last,
            is_first,
            i + 1, // 1-based index
            i,     // 0-based index
            max_index,
        );

        // Handle different spacing patterns for item variable references
        let patterns = super::utils::variable_placeholders(item_var);
        let replacements = [
            format!("{{{{{collection_var}.{i}."),
            format!("{{ {collection_var}.{i}."),
            format!("{{{{{collection_var}.{i}}}}}"),
            format!("{{ {collection_var}.{i} }}"),
        ];

        for (pattern, replacement) in patterns.iter().zip(replacements.iter()) {
            expanded_body = expanded_body.replace(pattern, replacement);
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

fn replace_forloop_context_at_current_level(
    template: &str,
    is_last: bool,
    is_first: bool,
    index: usize,
    index0: usize,
    length: usize,
) -> String {
    // Only replace forloop references that are at the top level (not inside nested for loops)
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    let mut nesting_level = 0;

    while let Some(current) = chars.next() {
        if current == '{' && chars.peek() == Some(&'%') {
            // Found a potential liquid tag
            let tag_start = result.len();
            result.push(current);
            result.push(chars.next().unwrap()); // push '%'

            // Read the tag content
            let mut tag_content = String::new();
            let mut found_closing = false;

            while let Some(c) = chars.next() {
                if c == '%' && chars.peek() == Some(&'}') {
                    chars.next(); // consume '}'
                    result.push(c);
                    result.push('}');
                    found_closing = true;
                    break;
                } else {
                    tag_content.push(c);
                    result.push(c);
                }
            }

            if found_closing {
                let tag_content = tag_content.trim();

                // Track nesting level
                if tag_content.starts_with("for ") {
                    nesting_level += 1;
                } else if tag_content == "endfor" {
                    nesting_level -= 1;
                } else if nesting_level == 0 && tag_content.starts_with("unless forloop.last") {
                    // This is an unless tag at the current level - replace it
                    let _tag_end = result.len();
                    if is_last {
                        // Remove the entire unless block for last iterations
                        result.truncate(tag_start);
                        // Skip to endunless and remove that too
                        skip_to_endunless(&mut chars);
                    } else {
                        // Keep content but remove unless tags for non-last iterations
                        result.truncate(tag_start);
                        // Read content until endunless and add it directly
                        let content = read_until_endunless(&mut chars);
                        result.push_str(&content);
                    }
                }
            }
        } else if current == '{' && chars.peek() == Some(&'{') {
            // Found handlebars expression - only replace if at current level
            let mut expr = String::new();
            expr.push(current);
            expr.push(chars.next().unwrap()); // push second '{'

            // Read until }}
            while let Some(c) = chars.next() {
                expr.push(c);
                if c == '}' && chars.peek() == Some(&'}') {
                    expr.push(chars.next().unwrap());
                    break;
                }
            }

            // Replace forloop variables only at current nesting level
            if nesting_level == 0 {
                if expr == "{{ forloop.last }}" {
                    result.push_str(if is_last { "true" } else { "false" });
                } else if expr == "{{ forloop.first }}" {
                    result.push_str(if is_first { "true" } else { "false" });
                } else if expr == "{{ forloop.index }}" {
                    result.push_str(&index.to_string());
                } else if expr == "{{ forloop.index0 }}" {
                    result.push_str(&index0.to_string());
                } else if expr == "{{ forloop.length }}" {
                    result.push_str(&length.to_string());
                } else {
                    result.push_str(&expr);
                }
            } else {
                result.push_str(&expr);
            }
        } else {
            result.push(current);
        }
    }

    result
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

        // The for loop processor now handles forloop context directly
        let result = process_liquid_for_loops(template, &variables).unwrap();

        // Should have processed unless tags with forloop context and remove comma on last item
        let expected = "{{ items.0 }},{{ items.1 }}";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_forloop_last_context() {
        let mut variables = HashMap::new();

        // Outer loop: skills
        variables.insert("skills.0.name".to_string(), "Programming".to_string());
        variables.insert("skills.0.keywords.0".to_string(), "Rust".to_string());
        variables.insert("skills.0.keywords.1".to_string(), "Python".to_string());
        variables.insert("skills.0.keywords.2".to_string(), "JavaScript".to_string());

        variables.insert("skills.1.name".to_string(), "Design".to_string());
        variables.insert("skills.1.keywords.0".to_string(), "Figma".to_string());
        variables.insert("skills.1.keywords.1".to_string(), "Photoshop".to_string());

        let template = r#"{% for skill in skills %}
{{ skill.name }}:
{% for keyword in skill.keywords %}{{ keyword }}{% unless forloop.last %}, {% endunless %}{% endfor %}
{% endfor %}"#;

        // The for loop processor now handles forloop context directly
        let result = process_liquid_for_loops(template, &variables).unwrap();

        // The inner forloop.last should refer to the keywords loop, not the skills loop
        // So we should see commas between keywords within each skill, but not after the last keyword
        let expected = r#"
{{ skills.0.name }}:
{{ skills.0.keywords.0 }}, {{ skills.0.keywords.1 }}, {{ skills.0.keywords.2 }}

{{ skills.1.name }}:
{{ skills.1.keywords.0 }}, {{ skills.1.keywords.1 }}
"#;

        println!("Result:\n{}", result);
        println!("Expected:\n{}", expected);

        assert_eq!(result, expected);
    }
}
