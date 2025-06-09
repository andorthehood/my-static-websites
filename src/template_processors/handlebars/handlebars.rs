use std::collections::HashMap;
use crate::error::{Error, Result};

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
                return Err(Error::Handlebars("Unclosed variable in template".to_string()));
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
pub fn replace_template_variables(template: &str, variables: &HashMap<String, String>) -> Result<String> {
    let mut result = template.to_string();
    
    for (key, value) in variables {
        result = replace_template_variable(&result, key, value)?;
    }
    
    Ok(result)
}

/// Validates if a variable name follows Handlebars naming conventions.
fn is_valid_variable_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    
    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }
    
    name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.')
}

/// Removes Handlebars variables from the input string.
/// This function will remove any content between {{ and }} including the braces.
/// 
/// # Arguments
/// * `input` - The input string containing Handlebars variables
/// 
/// # Returns
/// * `Result<String>` - The string with variables removed or an error if malformed
pub fn remove_handlebars_variables(input: &str) -> Result<String> {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_variable = false;
    
    while let Some(current) = chars.next() {
        if current == '{' && chars.peek() == Some(&'{') {
            if in_variable {
                return Err(Error::Handlebars("Nested opening braces '{{' found inside a variable".to_string()));
            }
            in_variable = true;
            // Skip the second '{'
            chars.next();
            
            // Skip whitespace after '{{'
            while let Some(&c) = chars.peek() {
                if !c.is_whitespace() {
                    break;
                }
                chars.next();
            }
            continue;
        }
        
        if in_variable {
            if current == '}' && chars.peek() == Some(&'}') {
                in_variable = false;
                chars.next(); // Skip the second '}'
                continue;
            }
        } else {
            result.push(current);
        }
    }
    
    if in_variable {
        return Err(Error::Handlebars("Unclosed Handlebars variable".to_string()));
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

    #[test]
    fn test_remove_handlebars_variables() {
        let template = "Lorem ipsum {{foo}} dolor {{ bar }} sit amet.";
        let result = remove_handlebars_variables(template).unwrap();
        assert_eq!(result, "Lorem ipsum  dolor  sit amet.");
    }

    #[test]
    fn test_remove_handlebars_variables_with_error() {
        let template = "Lorem ipsum {{foo dolor {{ bar }} sit amet.";
        let err = remove_handlebars_variables(template).unwrap_err();
        assert!(matches!(err, Error::Handlebars(_)));
        if let Error::Handlebars(msg) = err {
            assert!(msg.contains("Nested opening braces"), "Error message should mention nested braces");
        }
    }

    #[test]
    fn test_nested_variables() {
        let template = "Hello {{user.name}} and {{deeply.nested.value}}!";
        let result = remove_handlebars_variables(template).unwrap();
        assert_eq!(result, "Hello  and !");
    }

    #[test]
    fn test_empty_template() {
        let template = "";
        let result = remove_handlebars_variables(template).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_whitespace_handling() {
        let template = "Hello {{  spaced  }} world";
        let result = remove_handlebars_variables(template).unwrap();
        assert_eq!(result, "Hello  world");
    }

    #[test]
    fn test_unclosed_variable() {
        let template = "Hello {{name";
        let err = remove_handlebars_variables(template).unwrap_err();
        assert!(matches!(err, Error::Handlebars(_)));
        if let Error::Handlebars(msg) = err {
            assert!(msg.contains("Unclosed"), "Error message should mention unclosed variable");
        }
    }

    #[test]
    fn test_invalid_variable_name() {
        let template = "Hello {{123invalid}}";
        let result = replace_template_variable(template, "123invalid", "value");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_variable_name() {
        assert!(is_valid_variable_name("validName"));
        assert!(is_valid_variable_name("valid_name"));
        assert!(is_valid_variable_name("valid.nested.name"));
        assert!(!is_valid_variable_name(""));
        assert!(!is_valid_variable_name("123invalid"));
        assert!(!is_valid_variable_name("invalid!name"));
    }
}
