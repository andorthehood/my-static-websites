use crate::converters::typescript::utils::push_char_from;

use super::parse_state::ParseState;

/// Handles comment parsing and state updates.
/// Returns `true` if a comment was handled (indicating the caller should continue to next iteration).
pub fn handle_comments(
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

    // Handle entering comments when not in strings
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enters_line_comment_on_double_slash() {
        let input = "// comment\n";
        let b = input.as_bytes();
        let mut i = 0;
        let mut state = ParseState::new();
        let mut out = String::new();

        let handled = handle_comments(input, b, b.len(), &mut i, '/', &mut state, &mut out);

        assert!(handled);
        assert!(state.in_line_comment);
        assert_eq!(i, 2);
        assert_eq!(out, "//");
    }

    #[test]
    fn exits_line_comment_on_newline() {
        let input = "comment\n";
        let b = input.as_bytes();
        let mut i = 0;
        let mut state = ParseState::new();
        state.in_line_comment = true;
        let mut out = String::new();

        let handled = handle_comments(input, b, b.len(), &mut i, 'c', &mut state, &mut out);

        assert!(handled);
        assert!(state.in_line_comment);
        assert_eq!(out, "c");

        // Continue to newline
        i = 7;
        let handled = handle_comments(input, b, b.len(), &mut i, '\n', &mut state, &mut out);
        assert!(handled);
        assert!(!state.in_line_comment);
    }

    #[test]
    fn enters_block_comment_on_slash_star() {
        let input = "/* comment */";
        let b = input.as_bytes();
        let mut i = 0;
        let mut state = ParseState::new();
        let mut out = String::new();

        let handled = handle_comments(input, b, b.len(), &mut i, '/', &mut state, &mut out);

        assert!(handled);
        assert!(state.in_block_comment);
        assert_eq!(i, 2);
        assert_eq!(out, "/*");
    }

    #[test]
    fn exits_block_comment_on_star_slash() {
        let input = " comment */";
        let b = input.as_bytes();
        let mut i = 9; // Point to '*' in "*/"
        let mut state = ParseState::new();
        state.in_block_comment = true;
        let mut out = String::new();

        let handled = handle_comments(input, b, b.len(), &mut i, '*', &mut state, &mut out);

        assert!(handled);
        assert!(!state.in_block_comment);
        assert_eq!(i, 11); // Advanced past '/'
        assert_eq!(out, "*/");
    }

    #[test]
    fn does_not_enter_comment_inside_string() {
        let input = "\"//\"";
        let b = input.as_bytes();
        let mut i = 1; // Point to first '/'
        let mut state = ParseState::new();
        state.in_double = true; // Inside double-quoted string
        let mut out = String::new();

        let handled = handle_comments(input, b, b.len(), &mut i, '/', &mut state, &mut out);

        assert!(!handled);
        assert!(!state.in_line_comment);
        assert!(!state.in_block_comment);
    }

    #[test]
    fn returns_false_when_no_comment_detected() {
        let input = "abc";
        let b = input.as_bytes();
        let mut i = 0;
        let mut state = ParseState::new();
        let mut out = String::new();

        let handled = handle_comments(input, b, b.len(), &mut i, 'a', &mut state, &mut out);

        assert!(!handled);
    }
}
