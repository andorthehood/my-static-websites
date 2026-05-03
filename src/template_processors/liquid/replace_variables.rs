use super::nested_access::resolve_nested_path;
use super::validation::is_valid_variable_name;
use crate::error::{Error, Result};
use std::collections::HashMap;
use std::fmt::Write;

/// Replaces all Liquid variables in a template with their corresponding values.
/// Now supports nested object access with dot notation and array indexing.
///
/// # Arguments
/// * `template` - The template string containing Liquid variables
/// * `variables` - A `HashMap` containing variable names and their values
///
/// # Returns
/// * `Result<String>` - The template with all variables replaced or an error if malformed
pub fn replace_template_variables(
    template: &str,
    variables: &HashMap<String, String>,
) -> Result<String> {
    let mut result = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();

    while let Some(current) = chars.next() {
        if current == '{' && chars.peek() == Some(&'{') {
            // consume second '{'
            chars.next();

            // Read entire variable content up to '}}'
            let content = super::utils::read_liquid_variable_content(&mut chars)?;
            let expression = content.trim().to_string();

            if let Some(value) = evaluate_variable_expression(&expression, variables)? {
                result.push_str(&value);
            } else {
                // Variable not found, keep the original placeholder
                write!(result, "{{{{ {expression} }}}}").unwrap();
            }
        } else {
            result.push(current);
        }
    }

    Ok(result)
}

fn evaluate_variable_expression(
    expression: &str,
    variables: &HashMap<String, String>,
) -> Result<Option<String>> {
    let mut parts = expression.split('|');
    let source = parts.next().unwrap_or("").trim();
    let mut value = match resolve_expression_value(source, variables)? {
        Some(value) => value,
        None => return Ok(None),
    };

    for filter in parts {
        value = apply_output_filter(&value, filter.trim())?;
    }

    Ok(Some(value))
}

fn resolve_expression_value(
    expression: &str,
    variables: &HashMap<String, String>,
) -> Result<Option<String>> {
    if expression.parse::<i64>().is_ok() {
        return Ok(Some(expression.to_string()));
    }

    if !is_valid_variable_name(expression) {
        return Err(Error::Liquid(format!(
            "Invalid variable name: {expression}"
        )));
    }

    Ok(resolve_nested_path(expression, variables))
}

fn apply_output_filter(value: &str, filter_expression: &str) -> Result<String> {
    let (filter_name, filter_args) = super::utils::parse_filter_invocation(filter_expression)
        .ok_or_else(|| Error::Liquid("Invalid filter syntax".to_string()))?;

    match filter_name.as_str() {
        "plus" => apply_plus_filter(value, &filter_args),
        _ => Err(Error::Liquid(format!("Unknown filter: {filter_name}"))),
    }
}

fn apply_plus_filter(value: &str, argument: &str) -> Result<String> {
    let left = value.trim().parse::<i64>().map_err(|_| {
        Error::Liquid(format!(
            "plus filter requires a numeric value, got: {}",
            value.trim()
        ))
    })?;
    let right = argument.trim().parse::<i64>().map_err(|_| {
        Error::Liquid(format!(
            "plus filter requires a numeric argument, got: {}",
            argument.trim()
        ))
    })?;

    Ok((left + right).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_multiple_variables() {
        let mut variables = HashMap::new();
        variables.insert("foo".to_string(), "apple".to_string());
        variables.insert("bar".to_string(), "banana".to_string());

        let template = "Lorem ipsum {{foo}} dolor {{bar}} sit amet.";
        let result = replace_template_variables(template, &variables).unwrap();

        assert_eq!(result, "Lorem ipsum apple dolor banana sit amet.");
    }

    #[test]
    fn test_replace_multiple_variables_with_spaces() {
        let mut variables = HashMap::new();
        variables.insert("foo".to_string(), "apple".to_string());
        variables.insert("bar".to_string(), "banana".to_string());

        let template = "Lorem ipsum {{ foo }} dolor {{ bar }} sit amet.";
        let result = replace_template_variables(template, &variables).unwrap();

        assert_eq!(result, "Lorem ipsum apple dolor banana sit amet.");
    }

    #[test]
    fn test_replace_multiple_variables_with_nested_access() {
        let mut variables = HashMap::new();
        variables.insert("user.name".to_string(), "Alice".to_string());
        variables.insert("items.0.title".to_string(), "First Item".to_string());
        variables.insert("items.1.title".to_string(), "Second Item".to_string());

        let template = "Hello {{user.name}}! Items: {{items.0.title}}, {{items.1.title}}";
        let result = replace_template_variables(template, &variables).unwrap();

        assert_eq!(result, "Hello Alice! Items: First Item, Second Item");
    }

    #[test]
    fn test_replace_variables_simple() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "Alice".to_string());

        let template = "Hello {{name}}!";
        let result = replace_template_variables(template, &variables).unwrap();
        assert_eq!(result, "Hello Alice!");
    }

    #[test]
    fn test_replace_variables_dot_notation() {
        let mut variables = HashMap::new();
        variables.insert("user.name".to_string(), "Bob".to_string());
        variables.insert("user.age".to_string(), "30".to_string());

        let template = "Hello {{user.name}}, you are {{user.age}} years old!";
        let result = replace_template_variables(template, &variables).unwrap();
        assert_eq!(result, "Hello Bob, you are 30 years old!");
    }

    #[test]
    fn test_replace_variables_array_indices() {
        let mut variables = HashMap::new();
        variables.insert("people.0.name".to_string(), "Alice".to_string());
        variables.insert("people.1.name".to_string(), "Bob".to_string());

        let template = "First: {{people.0.name}}, Second: {{people.1.name}}";
        let result = replace_template_variables(template, &variables).unwrap();
        assert_eq!(result, "First: Alice, Second: Bob");
    }

    #[test]
    fn test_replace_variables_not_found() {
        let variables = HashMap::new();

        let template = "Hello {{missing.variable}}!";
        let result = replace_template_variables(template, &variables).unwrap();
        assert_eq!(result, "Hello {{ missing.variable }}!");
    }

    #[test]
    fn test_replace_variables_complex() {
        let mut variables = HashMap::new();
        variables.insert("data.0.details.1.value".to_string(), "test".to_string());

        let template = "Value: {{data.0.details.1.value}}";
        let result = replace_template_variables(template, &variables).unwrap();
        assert_eq!(result, "Value: test");
    }

    #[test]
    fn test_replace_variables_invalid_name() {
        let variables = HashMap::new();

        let template = "Hello {{invalid-name}}!";
        let result = replace_template_variables(template, &variables);
        assert!(result.is_err());
    }

    #[test]
    fn test_replace_variables_plus_filter_with_variable() {
        let mut variables = HashMap::new();
        variables.insert("count".to_string(), "7".to_string());

        let template = "Line {{ count | plus: 20 }}";
        let result = replace_template_variables(template, &variables).unwrap();
        assert_eq!(result, "Line 27");
    }

    #[test]
    fn test_replace_variables_plus_filter_with_numeric_literal() {
        let variables = HashMap::new();

        let template = "Line {{ 1 | plus: 20 }}";
        let result = replace_template_variables(template, &variables).unwrap();
        assert_eq!(result, "Line 21");
    }

    #[test]
    fn test_replace_variables_chained_plus_filters() {
        let variables = HashMap::new();

        let template = "Line {{ 1 | plus: 20 | plus: 3 }}";
        let result = replace_template_variables(template, &variables).unwrap();
        assert_eq!(result, "Line 24");
    }
}
