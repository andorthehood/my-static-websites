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
                let next_char = chars.peek().unwrap_or(&'\0');

                if !result.is_empty() {
                    let last_char = result.chars().last().unwrap_or('\0');

                    // Preserve space in specific cases where it's needed for CSS to work correctly
                    let needs_space =
                        // Between a number/percentage and a word (e.g., "100% 2px", "1rem solid")
                        (last_char.is_ascii_digit() || last_char == '%') && next_char.is_alphabetic() ||
                        // Between a percentage and a number (e.g., "100% 2px")
                        last_char == '%' && next_char.is_ascii_digit() ||
                        // Between words and numbers (e.g., "solid #fff", "auto 10px")
                        last_char.is_alphabetic() && (next_char.is_ascii_digit() || *next_char == '#') ||
                        // Between measurement units and words (e.g., "px solid", "rem auto")
                        (last_char == 'x' || last_char == 'm' || last_char == '%') && next_char.is_alphabetic() ||
                        // Between closing parenthesis and other values (e.g., ") 50%")
                        last_char == ')' && (next_char.is_ascii_digit() || next_char.is_alphabetic()) ||
                        // Between values in functions like rgba() or linear-gradient()
                        last_char == ',' && *next_char == '#' ||
                        // Between alphanumeric characters where CSS requires spaces
                        (last_char.is_alphanumeric() && next_char.is_alphanumeric() &&
                         !matches!(next_char, '{' | '}' | ';' | ':' | ',' | '(' | ')' | '[' | ']' | '>' | '+' | '~' | '*' | '/' | '='));

                    if needs_space {
                        result.push(' ');
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
    fn test_preserve_spaces_between_values_and_colors() {
        let css = "border: 1px solid #fff;";
        let expected = "border:1px solid #fff;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_preserve_spaces_in_background_size() {
        let css = "background-size: 100% 2px, 3px 100%;";
        let expected = "background-size:100% 2px,3px 100%;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_preserve_spaces_in_box_shadow() {
        let css = "box-shadow: inset 1rem 1rem 0px #ffffff;";
        let expected = "box-shadow:inset 1rem 1rem 0px #ffffff;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_preserve_spaces_after_comma_before_hash() {
        let css = "background: linear-gradient(rgba(255,0,0,0.5), #ff0000);";
        let expected = "background:linear-gradient(rgba(255,0,0,0.5), #ff0000);";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_preserve_spaces_between_units_and_words() {
        let css = "margin: 10px auto 20px solid;";
        let expected = "margin:10px auto 20px solid;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_preserve_spaces_with_percentages() {
        let css =
            "background: linear-gradient(rgba(237,239,239,0) 50%, rgba(255,255,255,0.25) 50%);";
        let expected =
            "background:linear-gradient(rgba(237,239,239,0) 50%,rgba(255,255,255,0.25) 50%);";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_escaped_quotes_in_strings() {
        let css = r#"body::before { content: "He said \"Hello\""; }"#;
        let expected = r#"body::before{content:"He said \"Hello\"";}"#;
        assert_eq!(minify_css(css), expected);
    }
}
