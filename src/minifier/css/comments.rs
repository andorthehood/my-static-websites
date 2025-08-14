/// Handles CSS comment detection and state management
pub struct CommentHandler {
    pub in_comment: bool,
}

impl CommentHandler {
    pub fn new() -> Self {
        Self { in_comment: false }
    }

    /// Handles the start of a potential CSS comment (/)
    /// Returns true if the character should be added to the result
    pub fn handle_comment_start(
        &mut self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        in_string: bool,
    ) -> bool {
        if !in_string && !self.in_comment {
            if chars.peek() == Some(&'*') {
                chars.next(); // consume the '*'
                self.in_comment = true;
                false // Skip the comment entirely (don't add to result)
            } else {
                true // Not a comment, add the '/' to result
            }
        } else {
            !self.in_comment // Only add if not in comment
        }
    }

    /// Handles the potential end of a CSS comment (*)
    /// Returns true if the character should be added to the result
    pub fn handle_comment_end(
        &mut self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        in_string: bool,
    ) -> bool {
        if self.in_comment && !in_string {
            if chars.peek() == Some(&'/') {
                chars.next(); // consume the '/'
                self.in_comment = false;
            }
            false // Skip comment content (don't add to result)
        } else {
            !self.in_comment // Only add if not in comment
        }
    }

    /// Returns true if currently inside a comment
    pub fn is_in_comment(&self) -> bool {
        self.in_comment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_detection() {
        let mut handler = CommentHandler::new();

        // Should not be in comment initially
        assert!(!handler.is_in_comment());

        // Set up as if we're in a comment
        handler.in_comment = true;
        assert!(handler.is_in_comment());

        // Create chars iterator positioned at '/' (after the '*' in "*/"
        let mut chars = "/ some text".chars().peekable();

        // Handle the end of comment when we encounter '*' and next is '/'
        let should_add = handler.handle_comment_end(&mut chars, false);
        assert!(!should_add); // Should not add comment content
        assert!(!handler.is_in_comment()); // Should exit comment state after consuming '/'
    }

    #[test]
    fn test_comment_start_detection() {
        let mut handler = CommentHandler::new();
        let mut chars = "* this is a comment */".chars().peekable();

        // Handle start of comment
        let should_add = handler.handle_comment_start(&mut chars, false);
        assert!(!should_add); // Should not add comment start
        assert!(handler.is_in_comment()); // Should be in comment state
    }

    #[test]
    fn test_not_comment_when_in_string() {
        let mut handler = CommentHandler::new();
        let mut chars = "* not a comment".chars().peekable();

        // Should not detect comment when in string
        let should_add = handler.handle_comment_start(&mut chars, true);
        assert!(should_add); // Should add the character since we're in a string
        assert!(!handler.is_in_comment()); // Should not enter comment state
    }
}
