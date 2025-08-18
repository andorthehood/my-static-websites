use crate::converters::typescript::utils::push_char_from;

pub fn remove_query_selector_generics(input: &str) -> String {
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
        let ch = b[i] as char;

        // Handle exiting comments
        if in_line_comment {
            push_char_from(input, &mut i, &mut out);
            if ch == '\n' {
                in_line_comment = false;
            }
            continue;
        }
        if in_block_comment {
            push_char_from(input, &mut i, &mut out);
            if ch == '*' && i < len && b[i] as char == '/' {
                out.push('/');
                i += 1;
                in_block_comment = false;
            }
            continue;
        }

        // Enter comments if not in string
        if !in_single && !in_double && !in_backtick
            && ch == '/' && i + 1 < len {
                let n = b[i + 1] as char;
                if n == '/' {
                    in_line_comment = true;
                    out.push(ch);
                    out.push(n);
                    i += 2;
                    continue;
                }
                if n == '*' {
                    in_block_comment = true;
                    out.push(ch);
                    out.push(n);
                    i += 2;
                    continue;
                }
            }

        // String toggles
        if !in_double && !in_backtick && ch == '\'' {
            in_single = !in_single;
            push_char_from(input, &mut i, &mut out);
            continue;
        }
        if !in_single && !in_backtick && ch == '"' {
            in_double = !in_double;
            push_char_from(input, &mut i, &mut out);
            continue;
        }
        if !in_single && !in_double && ch == '`' {
            in_backtick = !in_backtick;
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        // If inside any string, just copy
        if in_single || in_double || in_backtick {
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        if i + 12 <= len
            && input
                .get(i..)
                .is_some_and(|s| s.starts_with("querySelector"))
        {
            out.push_str("querySelector");
            i += "querySelector".len();
            // Optional "All"
            if i + 3 <= len && input.get(i..).is_some_and(|s| s.starts_with("All")) {
                out.push_str("All");
                i += 3;
            }
            // Skip spaces
            while i < len && (b[i] as char).is_ascii_whitespace() {
                i += 1;
            }
            // Remove generic if present
            if i < len && b[i] as char == '<' {
                let mut depth = 0;
                while i < len {
                    let ch2 = b[i] as char;
                    if ch2 == '<' {
                        depth += 1;
                    }
                    if ch2 == '>' {
                        depth -= 1;
                        i += 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    i += 1;
                }
                // Skip spaces
                while i < len && (b[i] as char).is_ascii_whitespace() {
                    i += 1;
                }
            }
            continue;
        }
        push_char_from(input, &mut i, &mut out);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::remove_query_selector_generics;

    #[test]
    fn strips_query_selector_generic() {
        let ts = "const el = document.querySelector<HTMLElement>('.x');";
        let js = remove_query_selector_generics(ts);
        assert!(js.contains("document.querySelector('.x')"));
        assert!(!js.contains("<HTMLElement>"));
    }

    #[test]
    fn strips_query_selector_all_generic() {
        let ts = "const els = document.querySelectorAll<HTMLAnchorElement>('a');";
        let js = remove_query_selector_generics(ts);
        assert!(js.contains("document.querySelectorAll('a')"));
        assert!(!js.contains("<HTMLAnchorElement>"));
    }
}
