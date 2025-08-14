use crate::converters::typescript::utils::{is_identifier_char, push_char_from};

pub fn remove_postfix_non_null(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();

    while i < len {
        let c = b[i] as char;
        if c == '!' {
            // Check previous non-space char
            let mut j = i;
            while j > 0 && (b[j - 1] as char).is_ascii_whitespace() {
                j -= 1;
            }
            let prev = if j > 0 { b[j - 1] as char } else { '\0' };
            // Check next non-space char
            let mut k = i + 1;
            while k < len && (b[k] as char).is_ascii_whitespace() {
                k += 1;
            }
            let next = if k < len { b[k] as char } else { '\0' };

            let prev_allows_postfix = prev == ')' || prev == ']' || is_identifier_char(prev);
            let next_is_terminator = next == '.'
                || next == ';'
                || next == ','
                || next == ')'
                || next == ']'
                || next == '\n';
            if prev_allows_postfix && next_is_terminator {
                // Drop this '!'
                i += 1;
                continue;
            }
        }
        push_char_from(input, &mut i, &mut out);
    }

    out
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
