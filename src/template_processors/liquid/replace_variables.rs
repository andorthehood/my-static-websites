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
            let var_name = content.trim().to_string();

            if !is_valid_variable_name(&var_name) {
                return Err(Error::Liquid(format!("Invalid variable name: {var_name}")));
            }

            // Try to resolve the variable using nested access
            if let Some(value) = resolve_nested_path(&var_name, variables) {
                result.push_str(&value);
            } else {
                // Variable not found, keep the original placeholder
                write!(result, "{{{{ {var_name} }}}}").unwrap();
            }
        } else {
            result.push(current);
        }
    }

    Ok(result)
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
}
