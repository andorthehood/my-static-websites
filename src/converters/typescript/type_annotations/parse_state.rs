/// Represents the state of string and comment parsing
#[allow(clippy::struct_excessive_bools)]
pub struct ParseState {
    pub in_single: bool,
    pub in_double: bool,
    pub in_backtick: bool,
    pub in_line_comment: bool,
    pub in_block_comment: bool,
}

impl ParseState {
    pub fn new() -> Self {
        Self {
            in_single: false,
            in_double: false,
            in_backtick: false,
            in_line_comment: false,
            in_block_comment: false,
        }
    }

    pub fn is_in_string(&self) -> bool {
        self.in_single || self.in_double || self.in_backtick
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state_is_all_false() {
        let state = ParseState::new();
        assert!(!state.in_single);
        assert!(!state.in_double);
        assert!(!state.in_backtick);
        assert!(!state.in_line_comment);
        assert!(!state.in_block_comment);
    }

    #[test]
    fn is_in_string_detects_single_quote() {
        let mut state = ParseState::new();
        assert!(!state.is_in_string());
        state.in_single = true;
        assert!(state.is_in_string());
    }

    #[test]
    fn is_in_string_detects_double_quote() {
        let mut state = ParseState::new();
        state.in_double = true;
        assert!(state.is_in_string());
    }

    #[test]
    fn is_in_string_detects_backtick() {
        let mut state = ParseState::new();
        state.in_backtick = true;
        assert!(state.is_in_string());
    }

    #[test]
    fn comments_do_not_affect_is_in_string() {
        let mut state = ParseState::new();
        state.in_line_comment = true;
        assert!(!state.is_in_string());
        state.in_block_comment = true;
        assert!(!state.is_in_string());
    }
}
