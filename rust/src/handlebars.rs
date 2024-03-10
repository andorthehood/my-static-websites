use std::collections::HashMap;

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
}
