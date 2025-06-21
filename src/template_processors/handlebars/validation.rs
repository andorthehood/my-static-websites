/// Validates if a variable name follows Handlebars naming conventions.
pub fn is_valid_variable_name(name: &str) -> bool {
    let mut chars = name.chars();
    if let Some(first) = chars.next() {
        (first.is_alphabetic() || first == '_')
            && chars.all(|c| c.is_alphanumeric() || c == '_' || c == '.')
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_variable_name() {
        assert!(!is_valid_variable_name(""));
        assert!(!is_valid_variable_name("1name"));
        assert!(!is_valid_variable_name("name@"));
        assert!(!is_valid_variable_name("name space"));
    }

    #[test]
    fn test_is_valid_variable_name() {
        assert!(is_valid_variable_name("name"));
        assert!(is_valid_variable_name("_name"));
        assert!(is_valid_variable_name("name123"));
        assert!(is_valid_variable_name("user.name"));
        assert!(is_valid_variable_name("deeply.nested.value"));
    }
}
