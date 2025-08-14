pub use super::quote_utils::trim_quotes;

/// Checks if a string is a quoted literal
#[cfg(test)]
pub fn is_quoted_literal(s: &str) -> bool {
    (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\''))
}

/// Extracts the literal value from a quoted string, or returns None if not quoted
#[cfg(test)]
pub fn extract_literal_value(expression: &str) -> Option<String> {
    if is_quoted_literal(expression) {
        Some(expression[1..expression.len() - 1].to_string())
    } else {
        None
    }
}

/// Parses a key-value pair from a string in the format "key:value" or "key:\"value\""
#[cfg(test)]
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

use std::collections::HashMap;

/// Parses a space-separated list of key:value pairs with optional quoted values into a HashMap.
/// Example: `name:"Alice" greeting:"Hello"` -> {"name": "Alice", "greeting": "Hello"}
pub fn parse_space_separated_key_value_params(input: &str) -> HashMap<String, String> {
    let mut properties = HashMap::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        // skip whitespace
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }

        // read key
        let mut key = String::new();
        while i < chars.len() && chars[i] != ':' && !chars[i].is_whitespace() {
            key.push(chars[i]);
            i += 1;
        }

        // skip whitespace before ':'
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= chars.len() || chars[i] != ':' {
            // malformed; continue scanning forward
            continue;
        }
        i += 1; // skip ':'

        // read value (quoted or unquoted)
        let mut value = String::new();
        let mut in_quotes = false;
        let mut quote_char = '"';

        if i < chars.len() && (chars[i] == '"' || chars[i] == '\'') {
            in_quotes = true;
            quote_char = chars[i];
            i += 1; // skip opening quote
        }

        while i < chars.len() {
            let ch = chars[i];
            if in_quotes {
                if ch == quote_char {
                    i += 1; // skip closing quote
                    break;
                }
                value.push(ch);
                i += 1;
            } else {
                if ch.is_whitespace() {
                    break;
                }
                value.push(ch);
                i += 1;
            }
        }

        if !key.is_empty() {
            properties.insert(key.trim().to_string(), value);
        }
    }

    properties
}

/// Builds common liquid variable placeholder patterns for a variable name.
/// Returns four variants: `{{var.`, `{{ var.`, `{{var}}`, `{{ var }}`
pub fn variable_placeholders(var: &str) -> [String; 4] {
    [
        format!("{{{{{var}."),
        format!("{{ {var}."),
        format!("{{{{{var}}}}}"),
        format!("{{ {var} }}"),
    ]
}

/// Parses a filter invocation of the form `name: args` and returns (name, args)
pub fn parse_filter_invocation(s: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }
    Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
}

/// Applies a list of range replacements to a string in reverse order.
pub fn apply_replacements_in_reverse(target: &mut String, replacements: &[(usize, usize, String)]) {
    for (start, end, replacement) in replacements.iter().rev() {
        target.replace_range(*start..*end, replacement);
    }
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

    #[test]
    fn test_parse_space_separated_key_value_params() {
        let map = parse_space_separated_key_value_params(
            "name:\"Alice\" greeting:\"Hello World\" count:42",
        );
        assert_eq!(map.get("name"), Some(&"Alice".to_string()));
        assert_eq!(map.get("greeting"), Some(&"Hello World".to_string()));
        assert_eq!(map.get("count"), Some(&"42".to_string()));
    }

    #[test]
    fn test_variable_placeholders() {
        let p = variable_placeholders("item");
        assert_eq!(p[0], "{{item.");
        assert_eq!(p[1], "{ item.");
        assert_eq!(p[2], "{{item}}");
        assert_eq!(p[3], "{ item }");
    }

    #[test]
    fn test_parse_filter_invocation() {
        let parsed = parse_filter_invocation("where: 'a', 'b'").unwrap();
        assert_eq!(parsed.0, "where");
        assert_eq!(parsed.1, "'a', 'b'");
        assert!(parse_filter_invocation("invalid").is_none());
    }

    #[test]
    fn test_apply_replacements_in_reverse() {
        let mut s = "0123456789".to_string();
        let replacements = vec![(2, 4, "AB".to_string()), (6, 9, "XYZ".to_string())];
        apply_replacements_in_reverse(&mut s, &replacements);
        assert_eq!(s, "01AB45XYZ9");
    }
}
