use crate::converters::typescript::utils::{is_identifier_char, push_char_from};

/// Parser state for tracking strings and comments
#[derive(Default)]
struct ParserState {
    in_single: bool,
    in_double: bool,
    in_backtick: bool,
    in_line_comment: bool,
    in_block_comment: bool,
}

impl ParserState {
    fn in_any_string(&self) -> bool {
        self.in_single || self.in_double || self.in_backtick
    }
}

/// Handle comment parsing and state updates
fn handle_comments(
    state: &mut ParserState,
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

    // Handle entering comments when not in strings
    if !state.in_any_string()
        && current_char == '/'
        && *index + 1 < length
    {
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
fn handle_strings(
    state: &mut ParserState,
    input: &str,
    current_char: char,
    index: &mut usize,
    output: &mut String,
) -> bool {
    match current_char {
        '\'' if !state.in_double && !state.in_backtick => {
            state.in_single = !state.in_single;
            push_char_from(input, index, output);
            true
        }
        '"' if !state.in_single && !state.in_backtick => {
            state.in_double = !state.in_double;
            push_char_from(input, index, output);
            true
        }
        '`' if !state.in_single && !state.in_double => {
            state.in_backtick = !state.in_backtick;
            push_char_from(input, index, output);
            true
        }
        _ => false,
    }
}

/// Check if a colon represents an object literal property
fn is_object_literal_property(input: &str, bytes: &[u8], colon_index: usize) -> bool {
    // Look behind for the identifier end
    let mut position = colon_index;
    while position > 0 && (bytes[position - 1] as char).is_ascii_whitespace() {
        position -= 1;
    }

    // Find the start of the property name
    let mut name_start = position;
    while name_start > 0 {
        let ch = bytes[name_start - 1] as char;
        if is_identifier_char(ch) || ch.is_ascii_digit() {
            name_start -= 1;
        } else {
            break;
        }
    }

    // Skip back over whitespace and comment lines
    let mut before_name = name_start;
    while before_name > 0 && (bytes[before_name - 1] as char).is_ascii_whitespace() {
        if bytes[before_name - 1] as char == '\n' {
            let mut line_start = before_name - 1;
            while line_start > 0 && bytes[line_start - 1] as char != '\n' {
                line_start -= 1;
            }
            let line_slice = &input[line_start..before_name - 1];
            if line_slice.contains("//") {
                before_name = line_start;
                continue;
            }
        }
        before_name -= 1;
    }

    let token_before_name = if before_name > 0 { bytes[before_name - 1] as char } else { '\0' };
    token_before_name == '{' || token_before_name == ','
}

/// Skip type annotation after a colon
fn skip_type_annotation(bytes: &[u8], start_index: usize) -> usize {
    let mut index = start_index;
    let length = bytes.len();

    // Skip initial whitespace
    while index < length && (bytes[index] as char).is_ascii_whitespace() {
        index += 1;
    }

    let mut angle_depth = 0;
    let mut paren_depth = 0;
    let mut bracket_depth = 0;
    let mut brace_depth = 0;

    while index < length {
        let ch = bytes[index] as char;
        match ch {
            '<' => angle_depth += 1,
            '>' => {
                if angle_depth > 0 {
                    angle_depth -= 1;
                }
            }
            '(' => paren_depth += 1,
            ')' => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                } else {
                    break;
                }
            }
            '[' => bracket_depth += 1,
            ']' => {
                if bracket_depth > 0 {
                    bracket_depth -= 1;
                }
            }
            '{' => brace_depth += 1,
            '}' => {
                if brace_depth > 0 {
                    brace_depth -= 1;
                } else {
                    break;
                }
            }
            '=' | ',' | ';' | '\n' => {
                if angle_depth == 0
                    && paren_depth == 0
                    && bracket_depth == 0
                    && brace_depth == 0
                {
                    break;
                }
            }
            _ => {}
        }
        index += 1;
    }

    index
}

pub fn remove_type_annotations(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut index = 0;
    let bytes = input.as_bytes();
    let length = bytes.len();
    let mut state = ParserState::default();

    while index < length {
        let current_char = bytes[index] as char;

        // Handle comments first
        if handle_comments(&mut state, input, bytes, current_char, &mut index, &mut output) {
            continue;
        }

        // Handle strings
        if handle_strings(&mut state, input, current_char, &mut index, &mut output) {
            continue;
        }

        // If inside any string, just copy
        if state.in_any_string() {
            push_char_from(input, &mut index, &mut output);
            continue;
        }

        // Handle type annotations after colons
        if current_char == ':' {
            // Check if this is an object literal property
            if is_object_literal_property(input, bytes, index) {
                output.push(':');
                index += 1;
                continue;
            }

            // Look behind for identifier or ')' (indicating a type annotation context)
            let mut position = index;
            while position > 0 && (bytes[position - 1] as char).is_ascii_whitespace() {
                position -= 1;
            }
            let prev_char = if position > 0 { bytes[position - 1] as char } else { '\0' };

            let looks_like_type_context = is_identifier_char(prev_char) || prev_char == ')';
            if looks_like_type_context {
                // Skip the type annotation
                index = skip_type_annotation(bytes, index + 1);
                
                // Insert space before '=' if needed
                let next_delim = if index < length { bytes[index] as char } else { '\0' };
                if next_delim == '=' {
                    if let Some(last_char) = output.chars().last() {
                        if !last_char.is_ascii_whitespace() {
                            output.push(' ');
                        }
                    } else {
                        output.push(' ');
                    }
                }
                continue;
            }
        }
        
        push_char_from(input, &mut index, &mut output);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::remove_type_annotations;

    #[test]
    fn preserves_object_literal_properties() {
        let ts = r#"
function f(){
	return {
		startX: x,
		startY: y,
		startDx: vx,
		startDy: vy
	};
}
		"#;
        let js = remove_type_annotations(ts);
        assert!(js.contains("startX: x"));
        assert!(js.contains("startY: y"));
        assert!(js.contains("startDx: vx"));
        assert!(js.contains("startDy: vy"));
    }

    #[test]
    fn preserves_object_literal_entries_in_conversion() {
        let ts = r#"
(function(){
	function g(){
		return { startX: x, startY: y, startDx: vx, startDy: vy };
	}
})();
		"#;
        let js = remove_type_annotations(ts);
        assert!(js.contains("startX: x"));
        assert!(js.contains("startY: y"));
        assert!(js.contains("startDx: vx"));
        assert!(js.contains("startDy: vy"));
    }
}
