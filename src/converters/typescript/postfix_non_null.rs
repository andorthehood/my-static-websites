use crate::converters::typescript::utils::{is_identifier_char, push_char_from};

pub fn remove_postfix_non_null(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut index = 0;
    let bytes = input.as_bytes();
    let length = bytes.len();

    while index < length {
        let current_char = bytes[index] as char;
        if current_char == '!' {
            // Check previous non-space char
            let mut prev_index = index;
            while prev_index > 0 && (bytes[prev_index - 1] as char).is_ascii_whitespace() {
                prev_index -= 1;
            }
            let prev_char = if prev_index > 0 { bytes[prev_index - 1] as char } else { '\0' };
            
            // Check next non-space char
            let mut next_index = index + 1;
            while next_index < length && (bytes[next_index] as char).is_ascii_whitespace() {
                next_index += 1;
            }
            let next_char = if next_index < length { bytes[next_index] as char } else { '\0' };

            let prev_allows_postfix = prev_char == ')' || prev_char == ']' || is_identifier_char(prev_char);
            let next_is_terminator = next_char == '.'
                || next_char == ';'
                || next_char == ','
                || next_char == ')'
                || next_char == ']'
                || next_char == '\n';
            if prev_allows_postfix && next_is_terminator {
                // Drop this '!'
                index += 1;
                continue;
            }
        }
        push_char_from(input, &mut index, &mut output);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::remove_postfix_non_null;

    #[test]
    fn removes_postfix_non_null_in_assignment() {
        let ts = "const data = cache.get('k')!;";
        let js = remove_postfix_non_null(ts);
        assert!(js.contains("cache.get('k');"));
        assert!(!js.contains(")!;"));
    }

    #[test]
    fn removes_postfix_non_null_in_property_access() {
        let ts = "x!.y";
        let js = remove_postfix_non_null(ts);
        assert_eq!(js, "x.y");
    }
}
