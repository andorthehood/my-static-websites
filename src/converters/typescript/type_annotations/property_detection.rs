use crate::converters::typescript::utils::is_identifier_char;

/// Finds the start of the property name by walking backwards
pub fn find_property_name_start(b: &[u8], mut j: usize) -> usize {
    while j > 0 {
        let ch = b[j - 1] as char;
        if is_identifier_char(ch) || ch.is_ascii_digit() {
            j -= 1;
        } else {
            break;
        }
    }
    j
}

/// Skips whitespace backwards, handling comment lines
pub fn skip_whitespace_and_comments(input: &str, b: &[u8], mut k: usize) -> usize {
    while k > 0 && (b[k - 1] as char).is_ascii_whitespace() {
        if b[k - 1] as char == '\n' {
            let mut line_start = k - 1;
            while line_start > 0 && b[line_start - 1] as char != '\n' {
                line_start -= 1;
            }
            let line_slice = &input[line_start..k - 1];
            if line_slice.contains("//") {
                k = line_start;
                continue;
            }
        }
        k -= 1;
    }
    k
}

/// Checks if this colon is part of an object literal property
pub fn is_object_literal_property(input: &str, b: &[u8], i: usize) -> bool {
    let mut j = i;
    while j > 0 && (b[j - 1] as char).is_ascii_whitespace() {
        j -= 1;
    }

    let name_start = find_property_name_start(b, j);
    let k = skip_whitespace_and_comments(input, b, name_start);
    let token_before_name = if k > 0 { b[k - 1] as char } else { '\0' };

    token_before_name == '{' || token_before_name == ','
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_property_name_start_finds_start() {
        let input = "{ name: value }";
        let b = input.as_bytes();
        let j = 6; // After 'name'
        let start = find_property_name_start(b, j);
        assert_eq!(start, 2); // Before 'n' in 'name'
    }

    #[test]
    fn find_property_name_start_handles_numbers() {
        let input = "{ prop123: value }";
        let b = input.as_bytes();
        let j = 9; // After 'prop123'
        let start = find_property_name_start(b, j);
        assert_eq!(start, 2); // Before 'p' in 'prop123'
    }

    #[test]
    fn find_property_name_start_stops_at_non_identifier() {
        let input = "{ 'prop': value }";
        let b = input.as_bytes();
        let j = 7; // After closing quote
        let start = find_property_name_start(b, j);
        assert_eq!(start, 3); // Stops at 'p' before opening quote
    }

    #[test]
    fn skip_whitespace_and_comments_skips_spaces() {
        let input = "   abc";
        let b = input.as_bytes();
        let k = 3; // After spaces
        let result = skip_whitespace_and_comments(input, b, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn skip_whitespace_and_comments_skips_comment_line() {
        let input = "code\n// comment\n";
        let b = input.as_bytes();
        let k = 16; // Position after "// comment\n"
        let result = skip_whitespace_and_comments(input, b, k);
        assert_eq!(result, 4); // Should skip back past first newline
    }

    #[test]
    fn is_object_literal_property_detects_after_brace() {
        let input = "{ name: value }";
        let b = input.as_bytes();
        let i = 6; // Position of ':'
        assert!(is_object_literal_property(input, b, i));
    }

    #[test]
    fn is_object_literal_property_detects_after_comma() {
        let input = "{ a: 1, name: value }";
        let b = input.as_bytes();
        let i = 12; // Position of second ':'
        assert!(is_object_literal_property(input, b, i));
    }

    #[test]
    fn is_object_literal_property_rejects_function_param() {
        let input = "function f(name: string)";
        let b = input.as_bytes();
        let i = 15; // Position of ':'
        assert!(!is_object_literal_property(input, b, i));
    }

    #[test]
    fn is_object_literal_property_handles_whitespace() {
        let input = "{  name  : value }";
        let b = input.as_bytes();
        let i = 9; // Position of ':'
        assert!(is_object_literal_property(input, b, i));
    }
}
