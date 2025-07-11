use super::validation::is_valid_variable_name;
use crate::error::{Error, Result};
use std::fmt::Write;

/// Replaces a single Handlebars variable in a template with its value.
///
/// # Arguments
/// * `template` - The template string containing Handlebars variables
/// * `key` - The variable name to replace
/// * `value` - The value to replace the variable with
///
/// # Returns
/// * `Result<String>` - The template with the variable replaced or an error if malformed
pub fn replace_template_variable(template: &str, key: &str, value: &str) -> Result<String> {
    if !is_valid_variable_name(key) {
        return Err(Error::Handlebars(format!("Invalid variable name: {key}")));
    }

    let mut result = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();

    while let Some(current) = chars.next() {
        if current == '{' && chars.peek() == Some(&'{') {
            chars.next(); // Skip second '{'
            let mut var_name = String::new();

            // Skip whitespace
            while let Some(&c) = chars.peek() {
                if !c.is_whitespace() {
                    break;
                }
                chars.next();
            }

            // Collect variable name
            while let Some(&c) = chars.peek() {
                if c == '}' {
                    break;
                }
                var_name.push(chars.next().unwrap());
            }

            var_name = var_name.trim().to_string();

            // Check for closing braces
            if chars.next() != Some('}') || chars.next() != Some('}') {
                return Err(Error::Handlebars(
                    "Unclosed variable in template".to_string(),
                ));
            }

            if var_name == key {
                result.push_str(value);
            } else {
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
    fn test_replace_single_variable() {
        let template = "Hello {{name}}, welcome!";
        let result = replace_template_variable(template, "name", "Alice").unwrap();
        assert_eq!(result, "Hello Alice, welcome!");
    }

    #[test]
    fn test_replace_single_variable_with_spaces() {
        let template = "Hello {{ name }}, welcome!";
        let result = replace_template_variable(template, "name", "Alice").unwrap();
        assert_eq!(result, "Hello Alice, welcome!");
    }

    #[test]
    fn test_replace_single_variable_not_found() {
        let template = "Hello {{ name }}, welcome!";
        let result = replace_template_variable(template, "age", "25").unwrap();
        assert_eq!(result, "Hello {{ name }}, welcome!");
    }

    #[test]
    fn test_invalid_variable_name() {
        let template = "Hello, world!";
        let result = replace_template_variable(template, "invalid name", "value");
        assert!(result.is_err());
    }
}
