use crate::converters::typescript::utils::{is_identifier_char, push_char_from};

/// Parser state for tracking strings and comments
#[derive(Default)]
struct InterfaceParserState {
    in_single_quote: bool,
    in_double_quote: bool,
    in_backtick: bool,
    in_line_comment: bool,
    in_block_comment: bool,
}

impl InterfaceParserState {
    fn in_any_string(&self) -> bool {
        self.in_single_quote || self.in_double_quote || self.in_backtick
    }
}

/// Handle comment parsing and state updates
fn handle_interface_comments(
    state: &mut InterfaceParserState,
    input: &str,
    bytes: &[u8],
    current_char: char,
    index: &mut usize,
    output: &mut String,
) -> bool {
    let length = bytes.len();

    // Handle exiting comments
    if state.in_line_comment {
        push_char_from(input, index, output);
        if current_char == '\n' {
            state.in_line_comment = false;
        }
        return true;
    }
    if state.in_block_comment {
        push_char_from(input, index, output);
        if current_char == '*' && *index < length && bytes[*index] as char == '/' {
            output.push('/');
            *index += 1;
            state.in_block_comment = false;
        }
        return true;
    }

    // Handle string states
    if !state.in_any_string() && current_char == '/' && *index + 1 < length {
        let next_char = bytes[*index + 1] as char;
        if next_char == '/' {
            state.in_line_comment = true;
            output.push(current_char);
            output.push(next_char);
            *index += 2;
            return true;
        }
        if next_char == '*' {
            state.in_block_comment = true;
            output.push(current_char);
            output.push(next_char);
            *index += 2;
            return true;
        }
    }

    false
}

/// Handle string parsing and state updates
fn handle_interface_strings(
    state: &mut InterfaceParserState,
    input: &str,
    current_char: char,
    index: &mut usize,
    output: &mut String,
) -> bool {
    match current_char {
        '\'' if !state.in_double_quote && !state.in_backtick => {
            state.in_single_quote = !state.in_single_quote;
            push_char_from(input, index, output);
            true
        }
        '"' if !state.in_single_quote && !state.in_backtick => {
            state.in_double_quote = !state.in_double_quote;
            push_char_from(input, index, output);
            true
        }
        '`' if !state.in_single_quote && !state.in_double_quote => {
            state.in_backtick = !state.in_backtick;
            push_char_from(input, index, output);
            true
        }
        _ => false,
    }
}

/// Check if the current position starts with "interface" keyword
fn is_interface_keyword_start(input: &str, start_index: usize) -> bool {
    input
        .get(start_index..)
        .is_some_and(|s| s.starts_with("interface "))
        || input
            .get(start_index..)
            .is_some_and(|s| s.starts_with("interface\t"))
        || input
            .get(start_index..)
            .is_some_and(|s| s.starts_with("interface\n"))
}

/// Skip whitespace characters and return the new position
fn skip_interface_whitespace(bytes: &[u8], start_index: usize) -> usize {
    let mut index = start_index;
    let length = bytes.len();

    while index < length && (bytes[index] as char).is_ascii_whitespace() {
        index += 1;
    }

    index
}

/// Skip identifier characters and return the new position
fn skip_identifier_chars(bytes: &[u8], start_index: usize) -> usize {
    let mut index = start_index;
    let length = bytes.len();

    while index < length && is_identifier_char(bytes[index] as char) {
        index += 1;
    }

    index
}

/// Skip balanced braces and return the new position
fn skip_balanced_braces(bytes: &[u8], start_index: usize) -> usize {
    let mut index = start_index;
    let mut depth = 0;
    let length = bytes.len();

    while index < length {
        let current_char = bytes[index] as char;
        match current_char {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                index += 1;
                if depth == 0 {
                    break;
                }
                continue;
            }
            _ => {}
        }
        index += 1;
    }

    index
}

/// Process interface blocks and remove them
fn process_interface_block(
    input: &str,
    bytes: &[u8],
    index: &mut usize,
) -> bool {
    let length = bytes.len();

    if is_interface_keyword_start(input, *index) {
        // Skip "interface" keyword
        *index += "interface".len();

        // Skip whitespace and interface name
        *index = skip_interface_whitespace(bytes, *index);
        *index = skip_identifier_chars(bytes, *index);
        *index = skip_interface_whitespace(bytes, *index);

        // Expect block starting with '{'
        if *index < length && bytes[*index] as char == '{' {
            // Skip balanced braces
            *index = skip_balanced_braces(bytes, *index);

            // Skip trailing whitespace
            *index = skip_interface_whitespace(bytes, *index);

            return true; // Interface block was processed and should be skipped
        }
    }

    false
}

pub fn remove_interface_blocks(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut index = 0;
    let bytes = input.as_bytes();
    let length = bytes.len();
    let mut state = InterfaceParserState::default();

    while index < length {
        let current_char = bytes[index] as char;

        // Handle comments first
        if handle_interface_comments(&mut state, input, bytes, current_char, &mut index, &mut output) {
            continue;
        }

        // Handle strings
        if handle_interface_strings(&mut state, input, current_char, &mut index, &mut output) {
            continue;
        }

        // Process interface blocks only when not in strings or comments
        if !state.in_any_string() {
            // Check for "interface" keyword
            if current_char == 'i' && process_interface_block(input, bytes, &mut index) {
                continue; // Interface block was removed, don't copy it
            }
        }

        push_char_from(input, &mut index, &mut output);
    }

    output
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
