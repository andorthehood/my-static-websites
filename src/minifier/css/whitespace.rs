use super::should_preserve_space::should_preserve_space_asm;

/// Handles CSS whitespace preservation rules
pub struct WhitespaceHandler;

impl WhitespaceHandler {
    /// Determines if a space should be preserved between the last character in result
    /// and the next character being processed
    pub fn should_preserve_space(result: &str, next_char: char) -> bool {
        should_preserve_space_asm(result, next_char)
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preserve_space_between_percentage_and_number() {
        let result = "width:100%";
        assert!(WhitespaceHandler::should_preserve_space(result, '2'));
    }

    #[test]
    fn test_preserve_space_between_number_and_word() {
        let result = "border:1px";
        assert!(WhitespaceHandler::should_preserve_space(result, 's')); // "solid"
    }

    #[test]
    fn test_preserve_space_between_word_and_hash() {
        let result = "border:1px solid";
        assert!(WhitespaceHandler::should_preserve_space(result, '#'));
    }

    #[test]
    fn test_preserve_space_between_selectors() {
        let result = ".foo";
        assert!(WhitespaceHandler::should_preserve_space(result, '.'));
    }

    #[test]
    fn test_preserve_space_before_negative() {
        let result = "margin:10px";
        assert!(WhitespaceHandler::should_preserve_space(result, '-'));
    }

    #[test]
    fn test_no_space_between_property_and_colon() {
        let result = "color";
        assert!(!WhitespaceHandler::should_preserve_space(result, ':'));
    }

    #[test]
    fn test_no_space_in_empty_result() {
        let result = "";
        assert!(!WhitespaceHandler::should_preserve_space(result, 'a'));
    }

    #[test]
    fn test_preserve_space_after_comma_before_hash() {
        let result = "background:linear-gradient(rgba(255,0,0,0.5),";
        assert!(WhitespaceHandler::should_preserve_space(result, '#'));
    }

    #[test]
    fn test_preserve_space_between_units_and_words() {
        let result = "margin:10px";
        assert!(WhitespaceHandler::should_preserve_space(result, 'a')); // "auto"
    }

    #[test]
    fn test_preserve_space_with_rem_units() {
        let result = "padding:1rem";
        assert!(WhitespaceHandler::should_preserve_space(result, 's')); // "solid"
    }

    #[test]
    fn test_preserve_space_after_parenthesis() {
        let result = "calc(100% - 10px)";
        assert!(WhitespaceHandler::should_preserve_space(result, '5')); // "50%"
    }

    #[test]
    fn test_preserve_space_between_id_selectors() {
        let result = "div";
        assert!(WhitespaceHandler::should_preserve_space(result, '#')); // "#myid"
    }
}
