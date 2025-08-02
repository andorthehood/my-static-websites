use crate::error::{Error, Result};

/// Removes Liquid variables from the input string.
/// This function will remove any content between {{ and }} including the braces.
///
/// # Arguments
/// * `input` - The input string containing Liquid variables
///
/// # Returns
/// * `Result<String>` - The string with variables removed or an error if malformed
pub fn remove_liquid_variables(input: &str) -> Result<String> {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_variable = false;

    while let Some(current) = chars.next() {
        if current == '{' && chars.peek() == Some(&'{') {
            if in_variable {
                return Err(Error::Liquid(
                    "Nested opening braces '{{' found inside a variable".to_string(),
                ));
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
        } else if in_variable {
            if current == '}' && chars.peek() == Some(&'}') {
                in_variable = false;
                chars.next(); // Skip the second '}'
            }
        } else {
            result.push(current);
        }
    }

    if in_variable {
        return Err(Error::Liquid("Unclosed Liquid variable".to_string()));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_liquid_variables() {
        let template = "Lorem ipsum {{foo}} dolor {{ bar }} sit amet.";
        let result = remove_liquid_variables(template).expect("Failed to remove liquid variables");
        assert_eq!(result, "Lorem ipsum  dolor  sit amet.");
    }

    #[test]
    fn test_remove_liquid_variables_with_error() {
        let template = "Lorem ipsum {{foo dolor {{ bar }} sit amet.";
        let err = remove_liquid_variables(template).unwrap_err();
        assert!(matches!(err, Error::Liquid(_)));
        if let Error::Liquid(msg) = err {
            assert!(
                msg.contains("Nested opening braces"),
                "Error message should mention nested braces"
            );
        }
    }

    #[test]
    fn test_nested_variables() {
        let template = "Hello {{user.name}} and {{deeply.nested.value}}!";
        let result = remove_liquid_variables(template).expect("Failed to remove nested variables");
        assert_eq!(result, "Hello  and !");
    }

    #[test]
    fn test_empty_template() {
        let template = "";
        let result = remove_liquid_variables(template).expect("Failed to process empty template");
        assert_eq!(result, "");
    }

    #[test]
    fn test_whitespace_handling() {
        let template = "Hello {{  user.name  }} World";
        let result =
            remove_liquid_variables(template).expect("Failed to handle whitespace in variables");
        assert_eq!(result, "Hello  World");
    }

    #[test]
    fn test_unclosed_variable() {
        let template = "Hello {{user.name World";
        let err = remove_liquid_variables(template).unwrap_err();
        assert!(matches!(err, Error::Liquid(_)));
        if let Error::Liquid(msg) = err {
            assert!(
                msg.contains("Unclosed"),
                "Error message should mention unclosed variable"
            );
        }
    }
}
