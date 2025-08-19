use crate::converters::typescript::utils::{is_identifier_char, push_char_from};

/// Parser state for tracking strings and comments
#[derive(Default)]
struct CallGenericsParserState {
    in_single_quote: bool,
    in_double_quote: bool,
    in_backtick: bool,
    in_line_comment: bool,
    in_block_comment: bool,
}

impl CallGenericsParserState {
    fn in_any_string(&self) -> bool {
        self.in_single_quote || self.in_double_quote || self.in_backtick
    }
}

/// Handle comment parsing and state updates
fn handle_comments_parsing(
    state: &mut CallGenericsParserState,
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

    // Enter comments when not in strings
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
fn handle_string_parsing(
    state: &mut CallGenericsParserState,
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

/// Parse a generic type expression and return its end position
fn parse_generic_expression(bytes: &[u8], start_index: usize) -> Option<usize> {
    let mut index = start_index;
    let mut depth = 0;
    let length = bytes.len();

    while index < length {
        let current_char = bytes[index] as char;
        match current_char {
            '<' => depth += 1,
            '>' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            _ => {}
        }
        index += 1;
    }

    None
}

/// Skip whitespace characters and return the new position
fn skip_whitespace(bytes: &[u8], start_index: usize) -> usize {
    let mut index = start_index;
    let length = bytes.len();

    while index < length && (bytes[index] as char).is_ascii_whitespace() {
        index += 1;
    }

    index
}

/// Process identifier and potential generic removal
fn process_identifier_with_generics(
    bytes: &[u8],
    index: &mut usize,
    output: &mut String,
) -> bool {
    let length = bytes.len();
    let current_char = bytes[*index] as char;

    // Detect start of identifier (and ensure previous is not identifier char)
    if (current_char.is_ascii_alphabetic() || current_char == '_' || current_char == '$')
        && (*index == 0 || !is_identifier_char(bytes[*index - 1] as char))
    {
        // Read identifier
        let identifier_start = *index;
        *index += 1;
        while *index < length && is_identifier_char(bytes[*index] as char) {
            *index += 1;
        }

        // Copy identifier to output
        if let Ok(identifier_str) = std::str::from_utf8(&bytes[identifier_start..*index]) {
            output.push_str(identifier_str);
        }

        // Skip whitespace after identifier
        let after_whitespace = skip_whitespace(bytes, *index);

        // If next is '<', try to parse generic and remove it only if followed by '('
        if after_whitespace < length && bytes[after_whitespace] as char == '<' {
            if let Some(after_generic) = parse_generic_expression(bytes, after_whitespace) {
                // Check if there's a '(' after the generic (indicating a function call)
                let after_generic_whitespace = skip_whitespace(bytes, after_generic);
                if after_generic_whitespace < length && bytes[after_generic_whitespace] as char == '(' {
                    // This is a function call with generics - remove the generics
                    *index = after_generic;
                    return true;
                }
                // Not a call context, keep original including whitespace
                if let Ok(original_text) = std::str::from_utf8(&bytes[*index..after_generic]) {
                    output.push_str(original_text);
                }
                *index = after_generic;
                return true;
            }
        }
        return true;
    }

    false
}

pub fn remove_generics_before_calls(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut index = 0;
    let bytes = input.as_bytes();
    let length = bytes.len();
    let mut state = CallGenericsParserState::default();

    while index < length {
        let current_char = bytes[index] as char;

        // Handle comments first
        if handle_comments_parsing(&mut state, input, bytes, current_char, &mut index, &mut output) {
            continue;
        }

        // Handle strings
        if handle_string_parsing(&mut state, input, current_char, &mut index, &mut output) {
            continue;
        }

        // If inside strings, just copy
        if state.in_any_string() {
            push_char_from(input, &mut index, &mut output);
            continue;
        }

        // Process identifiers with potential generics
        if process_identifier_with_generics(bytes, &mut index, &mut output) {
            continue;
        }

        push_char_from(input, &mut index, &mut output);
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
