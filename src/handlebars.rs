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

pub fn remove_handlebars_variables(input: &str) -> String {
    let mut result = String::new();
    let mut in_variable = false;
    let mut skip_next = false;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if skip_next {
            skip_next = false;
            i += 1;
            continue;
        }

        if i < chars.len() - 1 && chars[i] == '{' && chars[i + 1] == '{' {
            in_variable = true;
            i += 2; // Skip the '{{'
            while i < chars.len() && chars[i] == ' ' {
                i += 1;
            }
            continue;
        }

        if in_variable && i < chars.len() - 1 && chars[i] == '}' && chars[i + 1] == '}' {
            in_variable = false;
            i += 2; // Skip the '}}'
            continue;
        }

        if !in_variable {
            result.push(chars[i]);
        } else if chars[i] == '}' && i < chars.len() - 1 && chars[i + 1] == '}' {
            skip_next = true;
        }

        i += 1;
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

    #[test]
    fn test_remove_handlebars_variables() {
        let template = "Lorem ipsum {{foo}} dolor {{ bar }} sit amet.";
        let result = remove_handlebars_variables(template);
        assert_eq!(result, "Lorem ipsum  dolor  sit amet.");
    }
}
