/// Minifies CSS by removing unnecessary whitespace while preserving functionality
pub fn minify_css(css: &str) -> String {
    let mut result = String::with_capacity(css.len());
    let mut chars = css.chars().peekable();
    let mut in_string = false;
    let mut string_delimiter = '\0';
    let mut in_comment = false;
    let mut prev_char = '\0';

    while let Some(ch) = chars.next() {
        match ch {
            // Handle string literals (preserve whitespace inside strings)
            '"' | '\'' if !in_comment => {
                if !in_string {
                    in_string = true;
                    string_delimiter = ch;
                } else if ch == string_delimiter && prev_char != '\\' {
                    in_string = false;
                    string_delimiter = '\0';
                }
                result.push(ch);
            }

            // Handle CSS comments /* ... */
            '/' if !in_string && !in_comment => {
                if chars.peek() == Some(&'*') {
                    chars.next(); // consume the '*'
                    in_comment = true;
                    // Skip the comment entirely (don't add to result)
                } else {
                    result.push(ch);
                }
            }

            '*' if in_comment && !in_string => {
                if chars.peek() == Some(&'/') {
                    chars.next(); // consume the '/'
                    in_comment = false;
                }
                // Skip comment content (don't add to result)
            }

            // Skip comment content
            _ if in_comment => {
                // Do nothing, skip comment content
            }

            // Handle whitespace - skip all whitespace when not in strings
            ' ' | '\t' | '\r' | '\n' if !in_string => {
                // Skip all whitespace - we'll add back only necessary spaces
                // Look ahead to see if we need to preserve a space
                let next_char = chars.peek().unwrap_or(&'\0');

                if !result.is_empty() {
                    let last_char = result.chars().last().unwrap_or('\0');

                    // Only preserve space between two word characters or letters/numbers
                    // This handles cases like "font-family: Arial, sans-serif"
                    if last_char.is_alphanumeric() && next_char.is_alphanumeric() {
                        // But not if the next character is something that doesn't need a space before it
                        if !matches!(
                            next_char,
                            '{' | '}'
                                | ';'
                                | ':'
                                | ','
                                | '('
                                | ')'
                                | '['
                                | ']'
                                | '>'
                                | '+'
                                | '~'
                                | '*'
                                | '/'
                                | '='
                        ) {
                            result.push(' ');
                        }
                    }
                }
            }

            // Handle other characters
            _ if !in_comment => {
                result.push(ch);
            }

            // Skip everything else (comment content)
            _ => {}
        }

        prev_char = ch;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_whitespace_removal() {
        let css = "body   {   margin:   0;   padding:   0;   }";
        let expected = "body{margin:0;padding:0;}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_newline_removal() {
        let css = "body {\n    margin: 0;\n    padding: 0;\n}";
        let expected = "body{margin:0;padding:0;}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_comment_removal() {
        let css = "/* This is a comment */\nbody {\n    margin: 0; /* another comment */\n    padding: 0;\n}";
        let expected = "body{margin:0;padding:0;}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_preserve_strings() {
        let css = "body::before { content: \"Hello   World\"; }";
        let expected = "body::before{content:\"Hello   World\";}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_preserve_single_quote_strings() {
        let css = "body::before { content: 'Hello   World'; }";
        let expected = "body::before{content:'Hello   World';}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_complex_css() {
        let css = r#"
        /* Main styles */
        .container {
            display: flex;
            justify-content: center;
            align-items: center;
        }
        
        .button {
            background-color: #007bff;
            color: white;
            padding: 10px 20px;
            border: none;
            border-radius: 4px;
        }
        
        .button:hover {
            background-color: #0056b3;
        }
        "#;

        let result = minify_css(css);
        assert!(!result.contains("/*"));
        assert!(!result.contains("*/"));
        assert!(!result.contains('\n'));
        assert!(result.contains(".container{display:flex"));
        assert!(result.contains("background-color:#007bff"));
    }

    #[test]
    fn test_preserve_necessary_spaces() {
        let css = "h1, h2, h3 { font-family: Arial, sans-serif; }";
        let expected = "h1,h2,h3{font-family:Arial,sans-serif;}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_escaped_quotes_in_strings() {
        let css = r#"body::before { content: "He said \"Hello\""; }"#;
        let expected = r#"body::before{content:"He said \"Hello\"";}"#;
        assert_eq!(minify_css(css), expected);
    }
}
