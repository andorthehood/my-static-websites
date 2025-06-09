use std::collections::HashMap;
use crate::error::Result;
use crate::template_processors::liquid::process_liquid_conditional_tags;
use crate::template_processors::handlebars::{replace_template_variables, remove_handlebars_variables};

/// Processes all template tags in a given input string.
/// This includes both Liquid conditionals and Handlebars variables.
pub fn process_template_tags(input: &str, variables: &HashMap<String, String>) -> Result<String> {
    let mut result = input.to_string();
    // First process Liquid conditionals
    let keys: Vec<String> = variables.keys().cloned().collect();
    result = process_liquid_conditional_tags(&result, &keys);
    
    // Then process Handlebars variables
    result = replace_template_variables(&result, &variables)?;
    result = remove_handlebars_variables(&result)?;
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_template_tags() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "World".to_string());
        variables.insert("show_greeting".to_string(), "true".to_string());
        
        let input = "{% if show_greeting %}Hello {{name}}!{% endif %}";
        let result = process_template_tags(input, &variables).unwrap();
        assert_eq!(result, "Hello World!");
    }
} 