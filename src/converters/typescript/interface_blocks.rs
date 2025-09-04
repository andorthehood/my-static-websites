use crate::converters::typescript::utils::{is_identifier_char, push_char_from};

/// Represents the state of string and comment parsing
#[allow(clippy::struct_excessive_bools)]
struct ParseState {
    in_single: bool,
    in_double: bool,
    in_backtick: bool,
    in_line_comment: bool,
    in_block_comment: bool,
}

impl ParseState {
    fn new() -> Self {
        Self {
            in_single: false,
            in_double: false,
            in_backtick: false,
            in_line_comment: false,
            in_block_comment: false,
        }
    }

    fn is_in_string(&self) -> bool {
        self.in_single || self.in_double || self.in_backtick
    }
}

/// Handles comment parsing and state updates
fn handle_comments(
    input: &str,
    bytes: &[u8],
    len: usize,
    i: &mut usize,
    c: char,
    state: &mut ParseState,
    out: &mut String,
) -> bool {
    // Handle exiting comments
    if state.in_line_comment {
        push_char_from(input, i, out);
        if c == '\n' {
            state.in_line_comment = false;
        }
        return true;
    }
    if state.in_block_comment {
        push_char_from(input, i, out);
        if c == '*' && *i < len && bytes[*i] as char == '/' {
            out.push('/');
            *i += 1;
            state.in_block_comment = false;
        }
        return true;
    }

    // Enter comments when not in strings
    if !state.is_in_string() && c == '/' && *i + 1 < len {
        let n = bytes[*i + 1] as char;
        if n == '/' {
            state.in_line_comment = true;
            out.push(c);
            out.push(n);
            *i += 2;
            return true;
        }
        if n == '*' {
            state.in_block_comment = true;
            out.push(c);
            out.push(n);
            *i += 2;
            return true;
        }
    }
    false
}

/// Handles string literal parsing and state updates
fn handle_strings(
    input: &str,
    i: &mut usize,
    c: char,
    state: &mut ParseState,
    out: &mut String,
) -> bool {
    if !state.in_double && !state.in_backtick && c == '\'' {
        state.in_single = !state.in_single;
        push_char_from(input, i, out);
        return true;
    }
    if !state.in_single && !state.in_backtick && c == '"' {
        state.in_double = !state.in_double;
        push_char_from(input, i, out);
        return true;
    }
    if !state.in_single && !state.in_double && c == '`' {
        state.in_backtick = !state.in_backtick;
        push_char_from(input, i, out);
        return true;
    }
    false
}

/// Checks if current position starts with interface keyword followed by whitespace
fn starts_with_interface(input: &str, i: usize) -> bool {
    input.get(i..).is_some_and(|s| s.starts_with("interface "))
        || input.get(i..).is_some_and(|s| s.starts_with("interface\t"))
        || input.get(i..).is_some_and(|s| s.starts_with("interface\n"))
}

/// Skips whitespace characters
fn skip_whitespace(bytes: &[u8], len: usize, i: &mut usize) {
    while *i < len && (bytes[*i] as char).is_ascii_whitespace() {
        *i += 1;
    }
}

/// Skips identifier characters
fn skip_identifier(bytes: &[u8], len: usize, i: &mut usize) {
    while *i < len && is_identifier_char(bytes[*i] as char) {
        *i += 1;
    }
}

/// Skips balanced brace blocks
fn skip_brace_block(bytes: &[u8], len: usize, i: &mut usize) {
    let mut depth = 0;
    while *i < len {
        let ch = bytes[*i] as char;
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
            *i += 1;
            if depth == 0 {
                break;
            }
            continue;
        }
        *i += 1;
    }
}

/// Processes interface declarations and removes them
fn handle_interface_block(
    input: &str,
    bytes: &[u8],
    len: usize,
    i: &mut usize,
    c: char,
    state: &ParseState,
) -> bool {
    if !state.is_in_string() && c == 'i' && starts_with_interface(input, *i) {
        // Skip keyword
        *i += "interface".len();

        // Skip whitespace and interface name
        skip_whitespace(bytes, len, i);
        skip_identifier(bytes, len, i);
        skip_whitespace(bytes, len, i);

        // Expect block starting with '{'
        if *i < len && bytes[*i] as char == '{' {
            skip_brace_block(bytes, len, i);
            skip_whitespace(bytes, len, i);
            return true; // Skip the interface block
        }
    }
    false
}

pub fn remove_interface_blocks(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut state = ParseState::new();

    while i < len {
        let c = bytes[i] as char;

        // Handle comments first
        if handle_comments(input, bytes, len, &mut i, c, &mut state, &mut out) {
            continue;
        }

        // Handle strings
        if handle_strings(input, &mut i, c, &mut state, &mut out) {
            continue;
        }

        // Handle interface blocks
        if handle_interface_block(input, bytes, len, &mut i, c, &state) {
            continue; // Interface block was skipped
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
