/// Removes surrounding quotes from a string if present
/// Handles both single and double quotes
pub fn trim_quotes(s: &str) -> &str {
    s.trim_matches('"').trim_matches('\'')
}

/// Checks if a string is a quoted literal
pub fn is_quoted_literal(s: &str) -> bool {
    (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\''))
}

/// Extracts the literal value from a quoted string, or returns None if not quoted
pub fn extract_literal_value(expression: &str) -> Option<String> {
    if is_quoted_literal(expression) {
        Some(expression[1..expression.len() - 1].to_string())
    } else {
        None
    }
}

/// Parses a key-value pair from a string in the format "key:value" or "key:\"value\""
pub fn parse_key_value_pair(pair: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = pair.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let key = parts[0].trim().to_string();
    let value = trim_quotes(parts[1].trim()).to_string();

    Some((key, value))
}

/// Splits a string on commas while respecting quotes
pub fn split_respecting_quotes(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '"';

    for ch in input.chars() {
        match ch {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
                current.push(ch);
            }
            ch if in_quotes && ch == quote_char => {
                in_quotes = false;
                current.push(ch);
            }
            ',' if !in_quotes => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_quotes() {
        assert_eq!(trim_quotes("\"hello\""), "hello");
        assert_eq!(trim_quotes("'hello'"), "hello");
        assert_eq!(trim_quotes("hello"), "hello");
        assert_eq!(trim_quotes("\"hello'"), "hello");
    }

    #[test]
    fn test_is_quoted_literal() {
        assert!(is_quoted_literal("\"hello\""));
        assert!(is_quoted_literal("'hello'"));
        assert!(!is_quoted_literal("hello"));
        assert!(!is_quoted_literal("\"hello'"));
    }

    #[test]
    fn test_extract_literal_value() {
        assert_eq!(
            extract_literal_value("\"hello\""),
            Some("hello".to_string())
        );
        assert_eq!(extract_literal_value("'world'"), Some("world".to_string()));
        assert_eq!(extract_literal_value("hello"), None);
    }

    #[test]
    fn test_parse_key_value_pair() {
        let result = parse_key_value_pair("name:\"Alice\"");
        assert_eq!(result, Some(("name".to_string(), "Alice".to_string())));

        let result = parse_key_value_pair("age:25");
        assert_eq!(result, Some(("age".to_string(), "25".to_string())));

        let result = parse_key_value_pair("invalid");
        assert_eq!(result, None);
    }

    #[test]
    fn test_split_respecting_quotes() {
        let result = split_respecting_quotes("\"active\", \"true\"");
        assert_eq!(result, vec!["\"active\"", "\"true\""]);

        let result = split_respecting_quotes("active, true, \"quoted, value\"");
        assert_eq!(result, vec!["active", "true", "\"quoted, value\""]);

        let result = split_respecting_quotes("simple, values");
        assert_eq!(result, vec!["simple", "values"]);
    }
}
