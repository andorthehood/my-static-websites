/// Validates if a variable name follows Handlebars naming conventions.
pub fn is_valid_variable_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    
    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }
    
    name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.')
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