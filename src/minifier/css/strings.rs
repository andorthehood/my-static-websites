/// Handles CSS string literal detection and state management
pub struct StringHandler {
    pub in_string: bool,
    pub string_delimiter: char,
}

impl StringHandler {
    pub fn new() -> Self {
        Self {
            in_string: false,
            string_delimiter: '\0',
        }
    }

    /// Handles string delimiters (quotes)
    /// Returns true if the character should be added to the result
    pub fn handle_quote(&mut self, ch: char, prev_char: char, in_comment: bool) -> bool {
        if !in_comment {
            match (ch, self.in_string) {
                ('"' | '\'', false) => {
                    // Starting a string
                    self.in_string = true;
                    self.string_delimiter = ch;
                }
                ('"' | '\'', true) if ch == self.string_delimiter && prev_char != '\\' => {
                    // Ending a string (not escaped)
                    self.in_string = false;
                    self.string_delimiter = '\0';
                }
                _ => {} // Either not a quote or escaped quote
            }
        }
        true // Always add quotes to result
    }

    /// Returns true if currently inside a string literal
    pub fn is_in_string(&self) -> bool {
        self.in_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_detection() {
        let mut handler = StringHandler::new();
        
        // Should not be in string initially
        assert!(!handler.is_in_string());

        // Start string with double quote
        handler.handle_quote('"', '\0', false);
        assert!(handler.is_in_string());
        assert_eq!(handler.string_delimiter, '"');

        // End string with matching quote
        handler.handle_quote('"', 'a', false); // prev_char is not backslash
        assert!(!handler.is_in_string());
        assert_eq!(handler.string_delimiter, '\0');
    }

    #[test]
    fn test_single_quote_strings() {
        let mut handler = StringHandler::new();
        
        // Start string with single quote
        handler.handle_quote('\'', '\0', false);
        assert!(handler.is_in_string());
        assert_eq!(handler.string_delimiter, '\'');

        // End string with matching quote
        handler.handle_quote('\'', 'a', false);
        assert!(!handler.is_in_string());
    }

    #[test]
    fn test_escaped_quotes() {
        let mut handler = StringHandler::new();
        
        // Start string
        handler.handle_quote('"', '\0', false);
        assert!(handler.is_in_string());

        // Try to end with escaped quote - should remain in string
        handler.handle_quote('"', '\\', false);
        assert!(handler.is_in_string()); // Should still be in string

        // End with unescaped quote
        handler.handle_quote('"', 'a', false);
        assert!(!handler.is_in_string());
    }

    #[test]
    fn test_quotes_in_comments_ignored() {
        let mut handler = StringHandler::new();
        
        // Quote in comment should be ignored
        handler.handle_quote('"', '\0', true);
        assert!(!handler.is_in_string()); // Should not enter string state
    }

    #[test]
    fn test_mismatched_quotes() {
        let mut handler = StringHandler::new();
        
        // Start with double quote
        handler.handle_quote('"', '\0', false);
        assert!(handler.is_in_string());

        // Try to end with single quote - should remain in string
        handler.handle_quote('\'', 'a', false);
        assert!(handler.is_in_string()); // Should still be in string

        // End with correct quote
        handler.handle_quote('"', 'a', false);
        assert!(!handler.is_in_string());
    }
}