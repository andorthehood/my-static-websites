use super::validation::is_valid_variable_name;
use crate::error::{Error, Result};
use std::collections::HashMap;

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
        return Err(Error::Handlebars(format!("Invalid variable name: {}", key)));
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
                result.push_str(&format!("{{{{ {} }}}}", var_name));
            }
        } else {
            result.push(current);
        }
    }

    Ok(result)
}

/// Replaces all Handlebars variables in a template with their corresponding values.
///
/// # Arguments
/// * `template` - The template string containing Handlebars variables
/// * `variables` - A HashMap containing variable names and their values
///
/// # Returns
/// * `Result<String>` - The template with all variables replaced or an error if malformed
pub fn replace_template_variables(
    template: &str,
    variables: &HashMap<String, String>,
) -> Result<String> {
    let mut result = template.to_string();

    for (key, value) in variables {
        result = replace_template_variable(&result, key, value)?;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_template_variables() {
        let mut variables = HashMap::new();
        variables.insert("foo".to_string(), "apple".to_string());
        variables.insert("bar".to_string(), "banana".to_string());

        let template = "Lorem ipsum {{foo}} dolor {{bar}} sit amet.";
        let result = replace_template_variables(template, &variables).unwrap();

        assert_eq!(result, "Lorem ipsum apple dolor banana sit amet.");
    }

    #[test]
    fn test_replace_template_variables_with_spaces() {
        let mut variables = HashMap::new();
        variables.insert("foo".to_string(), "apple".to_string());
        variables.insert("bar".to_string(), "banana".to_string());

        let template = "Lorem ipsum {{ foo }} dolor {{ bar }} sit amet.";
        let result = replace_template_variables(template, &variables).unwrap();

        assert_eq!(result, "Lorem ipsum apple dolor banana sit amet.");
    }
}
