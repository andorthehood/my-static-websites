use super::replace_variable::replace_template_variable;
use crate::error::Result;
use std::collections::HashMap;

/// Replaces all Handlebars variables in a template with their corresponding values.
///
/// # Arguments
/// * `template` - The template string containing Handlebars variables
/// * `variables` - A `HashMap` containing variable names and their values
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
}
