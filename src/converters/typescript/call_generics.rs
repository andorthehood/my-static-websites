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
    b: &[u8],
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
        if c == '*' && *i < len && b[*i] as char == '/' {
            out.push('/');
            *i += 1;
            state.in_block_comment = false;
        }
        return true;
    }

    // Enter comments when not in strings
    if !state.is_in_string() && c == '/' && *i + 1 < len {
        let n = b[*i + 1] as char;
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

/// Skips whitespace and returns the final position
fn skip_whitespace(b: &[u8], len: usize, start: usize) -> usize {
    let mut pos = start;
    while pos < len && (b[pos] as char).is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

/// Tries to parse and skip a balanced generic block, returns end position if valid
fn try_parse_generic_block(b: &[u8], len: usize, start: usize) -> Option<usize> {
    let mut pos = start;
    let mut depth = 0;

    while pos < len {
        let ch = b[pos] as char;
        if ch == '<' {
            depth += 1;
        } else if ch == '>' {
            depth -= 1;
            pos += 1;
            if depth == 0 {
                return Some(pos);
            }
            continue;
        }
        pos += 1;
    }
    None
}

/// Processes identifier and handles generic removal for function calls
fn handle_identifier(
    _input: &str,
    bytes: &[u8],
    length: usize,
    position: &mut usize,
    current_char: char,
    output: &mut String,
) -> bool {
    // Detect start of identifier (and ensure previous is not identifier char)
    if (current_char.is_ascii_alphabetic() || current_char == '_' || current_char == '$')
        && (*position == 0 || !is_identifier_char(bytes[*position - 1] as char))
    {
        // Read identifier
        let start_ident = *position;
        *position += 1;
        while *position < length && is_identifier_char(bytes[*position] as char) {
            *position += 1;
        }

        // Copy identifier to output
        if let Ok(ident_str) = std::str::from_utf8(&bytes[start_ident..*position]) {
            output.push_str(ident_str);
        }

        // Skip whitespace after identifier
        let whitespace_end = skip_whitespace(bytes, length, *position);

        // If next is '<', try to parse generic and remove it only if next non-space after generic is '('
        if whitespace_end < length && bytes[whitespace_end] as char == '<' {
            if let Some(generic_end) = try_parse_generic_block(bytes, length, whitespace_end) {
                // Check if next non-space character is '(' (function call)
                let after_generic = skip_whitespace(bytes, length, generic_end);
                if after_generic < length && bytes[after_generic] as char == '(' {
                    // Drop the generic by advancing position to generic_end (after '>')
                    *position = generic_end;
                    return true;
                }
                // Not a call context, keep original including whitespace
                if let Ok(orig) = std::str::from_utf8(&bytes[*position..generic_end]) {
                    output.push_str(orig);
                }
                *position = generic_end;
                return true;
            }
        }
        return true;
    }
    false
}

pub fn remove_generics_before_calls(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut position = 0;
    let bytes = input.as_bytes();
    let length = bytes.len();
    let mut state = ParseState::new();

    while position < length {
        let current_char = bytes[position] as char;

        // Handle comments first
        if handle_comments(
            input,
            bytes,
            length,
            &mut position,
            current_char,
            &mut state,
            &mut output,
        ) {
            continue;
        }

        // Handle strings
        if handle_strings(input, &mut position, current_char, &mut state, &mut output) {
            continue;
        }

        // If inside strings, just copy
        if state.is_in_string() {
            push_char_from(input, &mut position, &mut output);
            continue;
        }

        // Handle identifiers with potential generics
        if handle_identifier(
            input,
            bytes,
            length,
            &mut position,
            current_char,
            &mut output,
        ) {
            continue;
        }

        push_char_from(input, &mut position, &mut output);
    }

    output
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
