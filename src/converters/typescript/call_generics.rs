use crate::converters::typescript::utils::{is_identifier_char, push_char_from};

pub fn remove_generics_before_calls(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();

    // Track strings and comments
    let mut in_single = false;
    let mut in_double = false;
    let mut in_backtick = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while i < len {
        let c = b[i] as char;

        // Handle exiting comments
        if in_line_comment {
            push_char_from(input, &mut i, &mut out);
            if c == '\n' {
                in_line_comment = false;
            }
            continue;
        }
        if in_block_comment {
            push_char_from(input, &mut i, &mut out);
            if c == '*' && i < len && b[i] as char == '/' {
                out.push('/');
                i += 1;
                in_block_comment = false;
            }
            continue;
        }

        // Enter comments when not in strings
        if !in_single && !in_double && !in_backtick
            && c == '/' && i + 1 < len {
                let n = b[i + 1] as char;
                if n == '/' {
                    in_line_comment = true;
                    out.push(c);
                    out.push(n);
                    i += 2;
                    continue;
                }
                if n == '*' {
                    in_block_comment = true;
                    out.push(c);
                    out.push(n);
                    i += 2;
                    continue;
                }
            }

        // String toggles
        if !in_double && !in_backtick && c == '\'' {
            in_single = !in_single;
            push_char_from(input, &mut i, &mut out);
            continue;
        }
        if !in_single && !in_backtick && c == '"' {
            in_double = !in_double;
            push_char_from(input, &mut i, &mut out);
            continue;
        }
        if !in_single && !in_double && c == '`' {
            in_backtick = !in_backtick;
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        // If inside strings, just copy
        if in_single || in_double || in_backtick {
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        // Detect start of identifier (and ensure previous is not identifier char)
        if (c.is_ascii_alphabetic() || c == '_' || c == '$')
            && (i == 0 || !is_identifier_char(b[i - 1] as char))
        {
            // Read identifier
            let start_ident = i;
            i += 1;
            while i < len && is_identifier_char(b[i] as char) {
                i += 1;
            }
            // Copy identifier to output
            if let Ok(ident_str) = std::str::from_utf8(&b[start_ident..i]) {
                out.push_str(ident_str);
            }

            // Skip whitespace
            let mut j = i;
            while j < len && (b[j] as char).is_ascii_whitespace() {
                j += 1;
            }
            // If next is '<', try to parse generic and remove it only if next non-space after generic is '('
            if j < len && b[j] as char == '<' {
                let mut k = j;
                let mut depth = 0;
                let mut valid = false;
                while k < len {
                    let ch2 = b[k] as char;
                    if ch2 == '<' {
                        depth += 1;
                    } else if ch2 == '>' {
                        depth -= 1;
                        if depth == 0 {
                            k += 1;
                            valid = true;
                            break;
                        }
                    }
                    k += 1;
                }
                if valid {
                    // Check next non-space
                    let mut m = k;
                    while m < len && (b[m] as char).is_ascii_whitespace() {
                        m += 1;
                    }
                    if m < len && b[m] as char == '(' {
                        // Drop the generic by advancing i to k (after '>')
                        i = k;
                        continue;
                    }
                    // Not a call context, keep original including whitespace
                    if let Ok(orig) = std::str::from_utf8(&b[i..k]) {
                        out.push_str(orig);
                    }
                    i = k;
                    continue;
                }
                // If not valid generic, fall through
            }
            continue;
        }
        push_char_from(input, &mut i, &mut out);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::remove_generics_before_calls;

    #[test]
    fn strips_generics_before_call() {
        let ts = "new Promise<void>((resolve) => resolve());";
        let js = remove_generics_before_calls(ts);
        assert!(js.contains("new Promise((resolve)"));
        assert!(!js.contains("<void>"));
    }

    #[test]
    fn keeps_generics_when_not_a_call() {
        let ts = "type X = Promise<void>;";
        let js = remove_generics_before_calls(ts);
        assert!(js.contains("Promise<void>"));
    }
}
