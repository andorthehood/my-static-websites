use crate::converters::typescript::utils::push_char_from;

/// Represents the state of string and comment parsing
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
    ch: char,
    state: &mut ParseState,
    out: &mut String,
) -> bool {
    // Handle exiting comments
    if state.in_line_comment {
        push_char_from(input, i, out);
        if ch == '\n' {
            state.in_line_comment = false;
        }
        return true;
    }
    if state.in_block_comment {
        push_char_from(input, i, out);
        if ch == '*' && *i < len && b[*i] as char == '/' {
            out.push('/');
            *i += 1;
            state.in_block_comment = false;
        }
        return true;
    }

    // Enter comments if not in string
    if !state.is_in_string() && ch == '/' && *i + 1 < len {
        let n = b[*i + 1] as char;
        if n == '/' {
            state.in_line_comment = true;
            out.push(ch);
            out.push(n);
            *i += 2;
            return true;
        }
        if n == '*' {
            state.in_block_comment = true;
            out.push(ch);
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
    ch: char,
    state: &mut ParseState,
    out: &mut String,
) -> bool {
    // String toggles
    if !state.in_double && !state.in_backtick && ch == '\'' {
        state.in_single = !state.in_single;
        push_char_from(input, i, out);
        return true;
    }
    if !state.in_single && !state.in_backtick && ch == '"' {
        state.in_double = !state.in_double;
        push_char_from(input, i, out);
        return true;
    }
    if !state.in_single && !state.in_double && ch == '`' {
        state.in_backtick = !state.in_backtick;
        push_char_from(input, i, out);
        return true;
    }
    false
}

/// Processes querySelector calls and removes generics
fn handle_query_selector(
    input: &str,
    b: &[u8],
    len: usize,
    i: &mut usize,
    out: &mut String,
) -> bool {
    if *i + 12 <= len
        && input
            .get(*i..)
            .is_some_and(|s| s.starts_with("querySelector"))
    {
        out.push_str("querySelector");
        *i += "querySelector".len();

        // Optional "All"
        if *i + 3 <= len && input.get(*i..).is_some_and(|s| s.starts_with("All")) {
            out.push_str("All");
            *i += 3;
        }

        // Skip spaces
        while *i < len && (b[*i] as char).is_ascii_whitespace() {
            *i += 1;
        }

        // Remove generic if present
        if *i < len && b[*i] as char == '<' {
            skip_generic_brackets(b, len, i);
            // Skip spaces after generic
            while *i < len && (b[*i] as char).is_ascii_whitespace() {
                *i += 1;
            }
        }
        return true;
    }
    false
}

/// Skips over balanced generic brackets
fn skip_generic_brackets(b: &[u8], len: usize, i: &mut usize) {
    let mut depth = 0;
    while *i < len {
        let ch = b[*i] as char;
        if ch == '<' {
            depth += 1;
        } else if ch == '>' {
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

pub fn remove_query_selector_generics(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();
    let mut state = ParseState::new();

    while i < len {
        let ch = b[i] as char;

        // Handle comments first
        if handle_comments(input, b, len, &mut i, ch, &mut state, &mut out) {
            continue;
        }

        // Handle strings
        if handle_strings(input, &mut i, ch, &mut state, &mut out) {
            continue;
        }

        // If inside any string, just copy
        if state.is_in_string() {
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        // Handle querySelector calls
        if handle_query_selector(input, b, len, &mut i, &mut out) {
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
