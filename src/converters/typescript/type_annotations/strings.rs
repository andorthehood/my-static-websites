use crate::converters::typescript::utils::push_char_from;

use super::parse_state::ParseState;

/// Handles string literal parsing and state updates.
/// Returns `true` if a string delimiter was handled (indicating the caller should continue to next iteration).
pub fn handle_strings(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggles_single_quote_state() {
        let input = "'";
        let mut i = 0;
        let mut state = ParseState::new();
        let mut out = String::new();

        let handled = handle_strings(input, &mut i, '\'', &mut state, &mut out);

        assert!(handled);
        assert!(state.in_single);
        assert_eq!(i, 1);
        assert_eq!(out, "'");

        // Toggle off
        i = 0;
        let handled = handle_strings(input, &mut i, '\'', &mut state, &mut out);
        assert!(handled);
        assert!(!state.in_single);
    }

    #[test]
    fn toggles_double_quote_state() {
        let input = "\"";
        let mut i = 0;
        let mut state = ParseState::new();
        let mut out = String::new();

        let handled = handle_strings(input, &mut i, '"', &mut state, &mut out);

        assert!(handled);
        assert!(state.in_double);
        assert_eq!(i, 1);
        assert_eq!(out, "\"");
    }

    #[test]
    fn toggles_backtick_state() {
        let input = "`";
        let mut i = 0;
        let mut state = ParseState::new();
        let mut out = String::new();

        let handled = handle_strings(input, &mut i, '`', &mut state, &mut out);

        assert!(handled);
        assert!(state.in_backtick);
        assert_eq!(i, 1);
        assert_eq!(out, "`");
    }

    #[test]
    fn ignores_single_quote_when_in_double_quote() {
        let input = "'";
        let mut i = 0;
        let mut state = ParseState::new();
        state.in_double = true;
        let mut out = String::new();

        let handled = handle_strings(input, &mut i, '\'', &mut state, &mut out);

        assert!(!handled);
        assert!(!state.in_single);
        assert!(state.in_double);
    }

    #[test]
    fn ignores_double_quote_when_in_single_quote() {
        let input = "\"";
        let mut i = 0;
        let mut state = ParseState::new();
        state.in_single = true;
        let mut out = String::new();

        let handled = handle_strings(input, &mut i, '"', &mut state, &mut out);

        assert!(!handled);
        assert!(state.in_single);
        assert!(!state.in_double);
    }

    #[test]
    fn ignores_backtick_when_in_single_quote() {
        let input = "`";
        let mut i = 0;
        let mut state = ParseState::new();
        state.in_single = true;
        let mut out = String::new();

        let handled = handle_strings(input, &mut i, '`', &mut state, &mut out);

        assert!(!handled);
        assert!(state.in_single);
        assert!(!state.in_backtick);
    }

    #[test]
    fn returns_false_for_non_string_char() {
        let input = "a";
        let mut i = 0;
        let mut state = ParseState::new();
        let mut out = String::new();

        let handled = handle_strings(input, &mut i, 'a', &mut state, &mut out);

        assert!(!handled);
    }
}
