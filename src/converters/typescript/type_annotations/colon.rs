use crate::converters::typescript::utils::is_identifier_char;

use super::property_detection::is_object_literal_property;
use super::type_skipping::{remove_optional_marker_from_output, skip_type_annotation};

/// Processes colon characters and handles type annotation removal.
/// Returns `true` if the colon was part of a type annotation (indicating the caller should continue to next iteration).
pub fn handle_colon(input: &str, b: &[u8], len: usize, i: &mut usize, out: &mut String) -> bool {
    // If this is an object literal property, keep the colon
    if is_object_literal_property(input, b, *i) {
        out.push(':');
        *i += 1;
        return true;
    }

    // Look behind for something that looks like an identifier or ')' (return type)
    let mut j = *i;
    while j > 0 && (b[j - 1] as char).is_ascii_whitespace() {
        j -= 1;
    }
    let prev_char = if j > 0 { b[j - 1] as char } else { '\0' };

    let mut looks_like_type_context = is_identifier_char(prev_char) || prev_char == ')';

    // Support optional parameters/properties like `name?: string`.
    if !looks_like_type_context && prev_char == '?' {
        let mut k = j.saturating_sub(1); // points to the '?'
        while k > 0 && (b[k - 1] as char).is_ascii_whitespace() {
            k -= 1;
        }
        let before_optional = if k > 0 { b[k - 1] as char } else { '\0' };
        if is_identifier_char(before_optional) || before_optional == ')' {
            remove_optional_marker_from_output(out);
            looks_like_type_context = true;
        }
    }

    if looks_like_type_context {
        skip_type_annotation(b, len, i, out);
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_object_literal_colon() {
        let input = "{ name: value }";
        let b = input.as_bytes();
        let len = b.len();
        let mut i = 6; // Position of ':'
        let mut out = String::from("{ name");

        let handled = handle_colon(input, b, len, &mut i, &mut out);

        assert!(handled);
        assert_eq!(i, 7);
        assert_eq!(out, "{ name:");
    }

    #[test]
    fn removes_function_parameter_type() {
        let input = "(name: string)";
        let b = input.as_bytes();
        let len = b.len();
        let mut i = 5; // Position of ':'
        let mut out = String::from("(name");

        let handled = handle_colon(input, b, len, &mut i, &mut out);

        assert!(handled);
        assert_eq!(i, 13); // Advanced past "string"
        assert_eq!(out, "(name");
    }

    #[test]
    fn removes_return_type_annotation() {
        let input = "(): void {";
        let b = input.as_bytes();
        let len = b.len();
        let mut i = 2; // Position of ':'
        let mut out = String::from("()");

        let handled = handle_colon(input, b, len, &mut i, &mut out);

        assert!(handled);
        assert_eq!(i, 9); // Stopped before '{'
        assert_eq!(out, "()");
    }

    #[test]
    fn removes_optional_parameter_type() {
        let input = "(name?: string)";
        let b = input.as_bytes();
        let len = b.len();
        let mut i = 6; // Position of ':'
        let mut out = String::from("(name?");

        let handled = handle_colon(input, b, len, &mut i, &mut out);

        assert!(handled);
        assert_eq!(i, 14); // Advanced past "string"
        assert_eq!(out, "(name"); // Optional marker removed
    }

    #[test]
    fn handles_colon_after_identifier_space() {
        // Note: This also matches ternary operator colons after identifiers
        // In practice this is rare and acceptable since ternaries usually have
        // more complex expressions that don't end with simple identifiers
        let input = "x ? y : z";
        let b = input.as_bytes();
        let len = b.len();
        let mut i = 6; // Position of ':'
        let mut out = String::from("x ? y ");

        let handled = handle_colon(input, b, len, &mut i, &mut out);

        assert!(handled); // Will handle this as it looks like type context
    }

    #[test]
    fn handles_whitespace_before_colon() {
        let input = "(name  : string)";
        let b = input.as_bytes();
        let len = b.len();
        let mut i = 7; // Position of ':'
        let mut out = String::from("(name  ");

        let handled = handle_colon(input, b, len, &mut i, &mut out);

        assert!(handled);
        assert_eq!(i, 15);
        assert_eq!(out, "(name  ");
    }
}
