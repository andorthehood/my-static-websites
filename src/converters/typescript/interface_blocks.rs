use crate::converters::typescript::utils::{is_identifier_char, push_char_from};

pub fn remove_interface_blocks(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Track strings and comments to avoid false positives
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

        // Handle string states
        if !in_single && !in_double && !in_backtick {
            if c == '/' && i + 1 < len {
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
        }
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

        if !in_single && !in_double && !in_backtick {
            // Check for "interface" keyword
            if c == 'i'
                && (input
                    .get(i..)
                    .map_or(false, |s| s.starts_with("interface "))
                    || input
                        .get(i..)
                        .map_or(false, |s| s.starts_with("interface\t"))
                    || input
                        .get(i..)
                        .map_or(false, |s| s.starts_with("interface\n")))
            {
                // Skip keyword
                i += "interface".len();
                // Skip whitespace and name
                while i < len && (bytes[i] as char).is_ascii_whitespace() {
                    i += 1;
                }
                while i < len && is_identifier_char(bytes[i] as char) {
                    i += 1;
                }
                while i < len && (bytes[i] as char).is_ascii_whitespace() {
                    i += 1;
                }
                // Expect block starting with '{'
                if i < len && bytes[i] as char == '{' {
                    // Skip balanced braces
                    let mut depth = 0;
                    while i < len {
                        let ch = bytes[i] as char;
                        if ch == '{' {
                            depth += 1;
                        }
                        if ch == '}' {
                            depth -= 1;
                            i += 1;
                            if depth == 0 {
                                break;
                            }
                            continue;
                        }
                        i += 1;
                    }
                    // Skip trailing whitespace
                    while i < len && (bytes[i] as char).is_ascii_whitespace() {
                        i += 1;
                    }
                    continue; // do not copy interface block
                }
            }
        }

        push_char_from(input, &mut i, &mut out);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::remove_interface_blocks;

    #[test]
    fn removes_interface_block() {
        let ts = "interface X { a: string; }\nconst a = 1;";
        let js = remove_interface_blocks(ts);
        assert!(!js.contains("interface X"));
        assert!(js.contains("const a = 1;"));
    }

    #[test]
    fn keeps_interface_word_in_strings() {
        let ts = "console.log('interface X { a: string }');";
        let js = remove_interface_blocks(ts);
        assert!(js.contains("'interface X { a: string }'"));
    }
}
