use super::utils::{
    find_collection_size, parse_assignment, parse_filter_invocation, read_until_closing_tag,
    read_until_endunless, resolve_variable_value, skip_to_endunless,
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

            if let Some(for_content) = tag_content.strip_prefix("for ") {
                // Parse the for loop
                let parts: Vec<&str> = for_content.split(" in ").collect();

                if parts.len() != 2 {
                    return Err(Error::Liquid("Invalid for loop syntax".to_string()));
                }

                let item_var = parts[0].trim();

                // Split the RHS into collection identifier and optional parameters
                let rhs = parts[1].trim();
                let mut rhs_iter = rhs.split_whitespace();
                let collection_var = rhs_iter
                    .next()
                    .ok_or_else(|| Error::Liquid("Invalid for loop syntax".to_string()))?
                    .trim();
                let params_str = rhs_iter.collect::<Vec<_>>().join(" ");

                // Parse optional parameters (e.g., limit:10)
                let mut limit: Option<usize> = None;
                if !params_str.is_empty() {
                    let params = super::utils::parse_space_separated_key_value_params(&params_str);
                    if let Some(limit_str) = params.get("limit") {
                        if let Ok(lim) = limit_str.parse::<usize>() {
                            limit = Some(lim);
                        }
                    }
                }

                // Find the loop body until {% endfor %}
                let loop_body = super::utils::read_nested_block(&mut chars, "for ", "endfor")?;

                // Expand the loop
                let expanded =
                    expand_for_loop(item_var, collection_var, &loop_body, variables, limit);
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
    limit: Option<usize>,
) -> String {
    // Find how many items are in the collection
    let total_size = find_collection_size(collection_var, variables);

    // If no indexed items found, return empty string
    if total_size == 0 {
        return String::new();
    }

    // Determine how many iterations to perform based on optional limit
    let loop_len = match limit {
        Some(lim) => std::cmp::min(total_size, lim),
        None => total_size,
    };

    // Expand the loop body for each item
    let mut result = String::new();
    for i in 0..loop_len {
        // Replace forloop context directly with actual values (no assign tags)
        let is_last = i == loop_len - 1;
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
            loop_len,
        );
        expanded_body = replace_forloop_references_in_current_level_tags(
            &expanded_body,
            is_last,
            is_first,
            i + 1,
            i,
            loop_len,
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

        // Replace item variable references inside Liquid if tag conditions
        // e.g., turn `{% if item.active %}` into `{% if collection.0.active %}`
        expanded_body = expanded_body.replace(
            &format!("{{% if {item_var}."),
            &format!("{{% if {collection_var}.{i}."),
        );

        expanded_body = inline_loop_assigns(&expanded_body, variables);

        result.push_str(&expanded_body);
    }

    result
}

fn replace_forloop_references_in_text(
    template: &str,
    is_last: bool,
    is_first: bool,
    index: usize,
    index0: usize,
    length: usize,
) -> String {
    template
        .replace("forloop.index0", &index0.to_string())
        .replace("forloop.index", &index.to_string())
        .replace("forloop.length", &length.to_string())
        .replace("forloop.last", if is_last { "true" } else { "false" })
        .replace("forloop.first", if is_first { "true" } else { "false" })
}

fn replace_forloop_references_in_current_level_tags(
    template: &str,
    is_last: bool,
    is_first: bool,
    index: usize,
    index0: usize,
    length: usize,
) -> String {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    let mut nesting_level = 0;

    while let Some(current) = chars.next() {
        if current == '{' && chars.peek() == Some(&'%') {
            chars.next();
            let mut tag_content = String::new();

            while let Some(c) = chars.next() {
                if c == '%' && chars.peek() == Some(&'}') {
                    chars.next();
                    break;
                }
                tag_content.push(c);
            }

            let trimmed = tag_content.trim();
            let rendered_tag = if nesting_level == 0 {
                replace_forloop_references_in_text(
                    trimmed, is_last, is_first, index, index0, length,
                )
            } else {
                trimmed.to_string()
            };

            result.push_str("{% ");
            result.push_str(&rendered_tag);
            result.push_str(" %}");

            if trimmed.starts_with("for ") {
                nesting_level += 1;
            } else if trimmed == "endfor" {
                nesting_level -= 1;
            }
        } else {
            result.push(current);
        }
    }

    result
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
                }
                tag_content.push(c);
                result.push(c);
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
                if let Some(replacement) =
                    replace_forloop_expression(&expr, is_last, is_first, index, index0, length)
                {
                    result.push_str(&replacement);
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

fn replace_forloop_expression(
    expression: &str,
    is_last: bool,
    is_first: bool,
    index: usize,
    index0: usize,
    length: usize,
) -> Option<String> {
    let inner = expression.strip_prefix("{{")?.strip_suffix("}}")?.trim();

    let replacements = [
        ("forloop.index0", index0.to_string()),
        ("forloop.index", index.to_string()),
        ("forloop.length", length.to_string()),
        (
            "forloop.last",
            if is_last { "true" } else { "false" }.to_string(),
        ),
        (
            "forloop.first",
            if is_first { "true" } else { "false" }.to_string(),
        ),
    ];

    for (name, value) in replacements {
        if inner == name {
            return Some(value);
        }

        if let Some(remainder) = inner.strip_prefix(name) {
            if remainder.trim_start().starts_with('|') {
                return Some(format!("{{{{ {value}{remainder} }}}}"));
            }
        }
    }

    None
}

fn inline_loop_assigns(template: &str, variables: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    let mut assigned_values = HashMap::new();

    while let Some(current) = chars.next() {
        if current == '{' && chars.peek() == Some(&'%') {
            chars.next();
            let Ok(tag_content) = read_until_closing_tag(&mut chars) else {
                result.push_str("{%");
                result.push_str(&chars.collect::<String>());
                break;
            };
            let trimmed = tag_content.trim();

            if let Some(assign_content) = super::utils::extract_tag_parameter(trimmed, "assign") {
                if let Some((name, expression)) = parse_assignment(&assign_content) {
                    if let Ok(value) =
                        evaluate_loop_assign_expression(&expression, variables, &assigned_values)
                    {
                        assigned_values.insert(name, value);
                    }
                }
            } else {
                result.push_str("{% ");
                result.push_str(&replace_local_assigns_in_tag(trimmed, &assigned_values));
                result.push_str(" %}");
            }
        } else if current == '{' && chars.peek() == Some(&'{') {
            chars.next();
            let Ok(expression) = super::utils::read_liquid_variable_content(&mut chars) else {
                result.push_str("{{");
                result.push_str(&chars.collect::<String>());
                break;
            };
            let trimmed = expression.trim();

            if let Some(value) = assigned_values.get(trimmed) {
                result.push_str(value);
            } else {
                result.push_str("{{ ");
                result.push_str(trimmed);
                result.push_str(" }}");
            }
        } else {
            result.push(current);
        }
    }

    result
}

fn replace_local_assigns_in_tag(tag: &str, assigned_values: &HashMap<String, String>) -> String {
    tag.split_whitespace()
        .map(|token| {
            assigned_values
                .get(token)
                .map_or_else(|| token.to_string(), Clone::clone)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn evaluate_loop_assign_expression(
    expression: &str,
    variables: &HashMap<String, String>,
    assigned_values: &HashMap<String, String>,
) -> Result<String> {
    let mut parts = expression.split('|');
    let source = parts.next().unwrap_or("").trim();
    let mut value = resolve_loop_assign_value(source, variables, assigned_values)?;

    for filter in parts {
        value = apply_loop_assign_filter(&value, filter.trim(), variables, assigned_values)?;
    }

    Ok(value)
}

fn resolve_loop_assign_value(
    expression: &str,
    variables: &HashMap<String, String>,
    assigned_values: &HashMap<String, String>,
) -> Result<String> {
    let expression = expression.trim();

    if let Some(value) = assigned_values.get(expression) {
        return Ok(value.clone());
    }

    if let Some(value) = resolve_variable_value(expression, variables) {
        return Ok(value);
    }

    if expression.parse::<i64>().is_ok() {
        return Ok(expression.to_string());
    }

    Err(Error::Liquid(format!(
        "Unable to resolve loop assign expression: {expression}"
    )))
}

fn apply_loop_assign_filter(
    value: &str,
    filter_expression: &str,
    variables: &HashMap<String, String>,
    assigned_values: &HashMap<String, String>,
) -> Result<String> {
    let (filter_name, filter_args) = parse_filter_invocation(filter_expression)
        .ok_or_else(|| Error::Liquid("Invalid filter syntax".to_string()))?;
    let left = value
        .trim()
        .parse::<i64>()
        .map_err(|_| Error::Liquid(format!("numeric filter requires a number, got: {value}")))?;
    let right = resolve_loop_assign_value(&filter_args, variables, assigned_values)?
        .trim()
        .parse::<i64>()
        .map_err(|_| {
            Error::Liquid(format!(
                "numeric filter requires a number, got: {}",
                filter_args.trim()
            ))
        })?;

    match filter_name.as_str() {
        "plus" => Ok((left + right).to_string()),
        "modulo" => {
            if right == 0 {
                return Err(Error::Liquid(
                    "modulo filter argument cannot be zero".to_string(),
                ));
            }
            Ok((left % right).to_string())
        }
        _ => Err(Error::Liquid(format!("Unknown filter: {filter_name}"))),
    }
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
        assert!(result.contains("{{ groups.0.members.0.name }}"));
        assert!(result.contains("{{ groups.0.members.1.name }}"));
        assert!(result.contains("{{ groups.1.members.0.name }}"));
    }

    #[test]
    fn test_for_loop_with_limit() {
        let mut variables = HashMap::new();
        variables.insert("items.0.name".to_string(), "A".to_string());
        variables.insert("items.1.name".to_string(), "B".to_string());
        variables.insert("items.2.name".to_string(), "C".to_string());

        let template = "{% for item in items limit:2 %}{{ forloop.index }}/{{ forloop.length }}: {{ item.name }}\n{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        let expected = "1/2: {{ items.0.name }}\n2/2: {{ items.1.name }}\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_loop_forloop_first_and_index0() {
        let mut variables = HashMap::new();
        variables.insert("items.0.name".to_string(), "A".to_string());
        variables.insert("items.1.name".to_string(), "B".to_string());

        let template = "{% for item in items %}{{ forloop.first }} {{ forloop.index0 }} {{ item.name }}\n{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();
        let expected = "true 0 {{ items.0.name }}\nfalse 1 {{ items.1.name }}\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_loop_unless_forloop_last_blocks() {
        let mut variables = HashMap::new();
        variables.insert("items.0.name".to_string(), "A".to_string());
        variables.insert("items.1.name".to_string(), "B".to_string());

        // unless forloop.last should include content for all but last iteration; removed entirely for last
        let template = "{% for item in items %}{{ item.name }}{% unless forloop.last %}, {% endunless %}{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();
        // Comma and space only between first and second
        let expected = "{{ items.0.name }}, {{ items.1.name }}";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_forloop_vars_not_replaced_inside_nested_loops() {
        let mut variables = HashMap::new();
        variables.insert("outer.0.inner.0".to_string(), "a".to_string());
        variables.insert("outer.0.inner.1".to_string(), "b".to_string());
        variables.insert("outer.1.inner.0".to_string(), "c".to_string());

        // forloop.index in inner loop should not be replaced by outer's forloop replacements
        let template = "{% for o in outer %}{% for i in o.inner %}({{ forloop.index }}){% endfor %}{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        // The inner forloop.index should be replaced by the inner loop indices (1,2 for first outer; 1 for second)
        let expected = "(1)(2)(1)";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_loop_preserves_filters_on_forloop_index() {
        let mut variables = HashMap::new();
        variables.insert("items.0.name".to_string(), "A".to_string());
        variables.insert("items.1.name".to_string(), "B".to_string());

        let template =
            "{% for item in items %}{{ forloop.index | plus: 20 }}: {{ item.name }}\n{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        let expected =
            "{{ 1 | plus: 20 }}: {{ items.0.name }}\n{{ 2 | plus: 20 }}: {{ items.1.name }}\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_loop_inlines_assigns_using_forloop_index() {
        let mut variables = HashMap::new();
        variables.insert("items.0.name".to_string(), "A".to_string());
        variables.insert("items.1.name".to_string(), "B".to_string());

        let template = "{% for item in items %}{% assign row_mod = forloop.index | modulo: 2 %}{% if row_mod == 0 %}reverse{% endif %} {{ item.name }}\n{% endfor %}";
        let result = process_liquid_for_loops(template, &variables).unwrap();

        let expected =
            "{% if 1 == 0 %}reverse{% endif %} {{ items.0.name }}\n{% if 0 == 0 %}reverse{% endif %} {{ items.1.name }}\n";
        assert_eq!(result, expected);
    }
}
