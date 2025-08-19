use crate::converters::typescript::utils::push_char_from;

pub fn remove_as_casts(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Track strings and comments to avoid modifying inside them
    let mut in_single = false;
    let mut in_double = false;
    let mut in_backtick = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while i < len {
        let c = bytes[i] as char;

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
            if c == '*' && i < len && bytes[i] as char == '/' {
                out.push('/');
                i += 1;
                in_block_comment = false;
            }
            continue;
        }

        // Handle entering comments when not in strings
        if !in_single && !in_double && !in_backtick
            && c == '/' && i + 1 < len {
                let n = bytes[i + 1] as char;
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

        // Handle string state toggles
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

        // If inside any string, just copy
        if in_single || in_double || in_backtick {
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        if i + 2 < len
            && bytes[i].is_ascii_whitespace()
            && input.get(i + 1..).is_some_and(|s| s.starts_with("as "))
        {
            // Found " as ": remove until a terminator character
            i += 1 + 3; // skip space + "as "
            while i < len {
                let ch = bytes[i] as char;
                if ch == ')' || ch == ';' || ch == ',' || ch == '\n' || ch == '.' || ch == ']' {
                    break;
                }
                i += 1;
            }
            continue; // do not copy the removed type
        }
        // Handle "(ident as Type)" where there might not be leading space before 'as'
        if i + 4 < len && input.get(i..).is_some_and(|s| s.starts_with(" as ")) {
            i += 4;
            while i < len {
                let ch = bytes[i] as char;
                if ch == ')' || ch == ';' || ch == ',' || ch == '\n' || ch == '.' || ch == ']' {
                    break;
                }
                i += 1;
            }
            continue;
        }
        push_char_from(input, &mut i, &mut out);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::remove_as_casts;

    #[test]
    fn removes_inline_as_cast() {
        let ts = "const x = y as number;";
        let js = remove_as_casts(ts);
        assert_eq!(js.trim(), "const x = y;");
    }

    #[test]
    fn removes_parenthesized_as_cast() {
        let ts = "(style as HTMLLinkElement).onload = () => {};";
        let js = remove_as_casts(ts);
        assert!(js.contains("(style).onload"));
        assert!(!js.contains("as HTMLLinkElement"));
    }
}
