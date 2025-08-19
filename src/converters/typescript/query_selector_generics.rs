use crate::converters::typescript::utils::push_char_from;

/// Parser state for tracking strings and comments
#[derive(Default)]
struct QuerySelectorParserState {
    in_single_quote: bool,
    in_double_quote: bool,
    in_backtick: bool,
    in_line_comment: bool,
    in_block_comment: bool,
}

impl QuerySelectorParserState {
    fn in_any_string(&self) -> bool {
        self.in_single_quote || self.in_double_quote || self.in_backtick
    }
}

/// Handle comment parsing and state updates
fn handle_query_selector_comments(
    state: &mut QuerySelectorParserState,
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

    // Enter comments if not in string
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
fn handle_query_selector_strings(
    state: &mut QuerySelectorParserState,
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

/// Skip whitespace characters and return the new position
fn skip_query_selector_whitespace(bytes: &[u8], start_index: usize) -> usize {
    let mut index = start_index;
    let length = bytes.len();

    while index < length && (bytes[index] as char).is_ascii_whitespace() {
        index += 1;
    }

    index
}

/// Skip balanced angle brackets and return the new position
fn skip_balanced_angle_brackets(bytes: &[u8], start_index: usize) -> usize {
    let mut index = start_index;
    let mut depth = 0;
    let length = bytes.len();

    while index < length {
        let current_char = bytes[index] as char;
        match current_char {
            '<' => depth += 1,
            '>' => {
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

/// Process querySelector methods and remove their generics
fn process_query_selector_method(
    input: &str,
    bytes: &[u8],
    index: &mut usize,
    output: &mut String,
) -> bool {
    let length = bytes.len();
    let query_selector_len = "querySelector".len();

    if *index + query_selector_len <= length
        && input
            .get(*index..)
            .is_some_and(|s| s.starts_with("querySelector"))
    {
        output.push_str("querySelector");
        *index += query_selector_len;

        // Optional "All"
        if *index + 3 <= length && input.get(*index..).is_some_and(|s| s.starts_with("All")) {
            output.push_str("All");
            *index += 3;
        }

        // Skip spaces
        *index = skip_query_selector_whitespace(bytes, *index);

        // Remove generic if present
        if *index < length && bytes[*index] as char == '<' {
            *index = skip_balanced_angle_brackets(bytes, *index);

            // Skip spaces after generic
            *index = skip_query_selector_whitespace(bytes, *index);
        }

        return true;
    }

    false
}

pub fn remove_query_selector_generics(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut index = 0;
    let bytes = input.as_bytes();
    let length = bytes.len();
    let mut state = QuerySelectorParserState::default();

    while index < length {
        let current_char = bytes[index] as char;

        // Handle comments first
        if handle_query_selector_comments(&mut state, input, bytes, current_char, &mut index, &mut output) {
            continue;
        }

        // Handle strings
        if handle_query_selector_strings(&mut state, input, current_char, &mut index, &mut output) {
            continue;
        }

        // If inside any string, just copy
        if state.in_any_string() {
            push_char_from(input, &mut index, &mut output);
            continue;
        }

        // Process querySelector methods
        if process_query_selector_method(input, bytes, &mut index, &mut output) {
            continue;
        }

        push_char_from(input, &mut index, &mut output);
    }

    output
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
