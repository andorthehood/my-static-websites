use std::collections::HashMap;
use crate::error::{Error, Result};

pub fn replace_template_variable(template: &str, key: &str, value: &str) -> String {
    let mut result = template.to_string();

    let placeholder_with_space = format!("{{{{ {} }}}}", key);
    let placeholder_without_space = format!("{{{{{}}}}}", key);

    result = result.replace(&placeholder_with_space, value);
    result = result.replace(&placeholder_without_space, value);

    result
}

pub fn replace_template_variables(template: &str, variables: &HashMap<String, String>) -> String {
    let mut result = template.to_string();

    for (key, value) in variables {
        result = replace_template_variable(&result, key, value);
    }

    result
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
        let result = replace_template_variables(template, &variables);

        assert_eq!(result, "Lorem ipsum apple dolor banana sit amet.");
    }

    #[test]
    fn test_replace_template_variables_with_spaces() {
        let mut variables = HashMap::new();
        variables.insert("foo".to_string(), "apple".to_string());
        variables.insert("bar".to_string(), "banana".to_string());

        let template = "Lorem ipsum {{ foo }} dolor {{ bar }} sit amet.";
        let result = replace_template_variables(template, &variables);

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
}
