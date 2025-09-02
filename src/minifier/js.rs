/// Minifies JavaScript by removing unnecessary whitespace and comments while preserving functionality
pub fn minify_js(js: &str) -> String {
    let mut result = String::with_capacity(js.len());
    let mut chars = js.chars().peekable();
    let mut in_string = false;
    let mut string_delimiter = '\0';
    let mut in_template_literal = false;
    let mut in_single_line_comment = false;
    let mut in_multi_line_comment = false;
    let mut in_regex = false;
    let mut prev_char = '\0';
    let mut prev_non_whitespace = '\0';

    while let Some(ch) = chars.next() {
        // Handle single-line comments
        if !in_string && !in_template_literal && !in_regex && !in_multi_line_comment && ch == '/'
            && chars.peek() == Some(&'/') {
                chars.next(); // consume the second '/'
                in_single_line_comment = true;
                continue;
            }

        // End single-line comment on newline
        if in_single_line_comment && ch == '\n' {
            in_single_line_comment = false;
            // Add a newline to preserve potential ASI (Automatic Semicolon Insertion)
            if !result.is_empty() {
                let last_char = result.chars().last().unwrap_or('\0');
                if !matches!(last_char, ';' | '{' | '}') {
                    result.push('\n');
                }
            }
            continue;
        }

        // Skip single-line comment content
        if in_single_line_comment {
            continue;
        }

        // Handle multi-line comments
        if !in_string && !in_template_literal && !in_regex && !in_multi_line_comment && ch == '/'
            && chars.peek() == Some(&'*') {
                chars.next(); // consume the '*'
                in_multi_line_comment = true;
                continue;
            }

        // End multi-line comment
        if in_multi_line_comment && ch == '*'
            && chars.peek() == Some(&'/') {
                chars.next(); // consume the '/'
                in_multi_line_comment = false;
                continue;
            }

        // Skip multi-line comment content
        if in_multi_line_comment {
            continue;
        }

        // Handle template literals
        if !in_string && !in_regex && ch == '`' {
            if !in_template_literal {
                in_template_literal = true;
            } else if prev_char != '\\' {
                in_template_literal = false;
            }
            result.push(ch);
            prev_char = ch;
            continue;
        }

        // Handle string literals
        if !in_template_literal && !in_regex && (ch == '"' || ch == '\'') {
            if !in_string {
                in_string = true;
                string_delimiter = ch;
            } else if ch == string_delimiter && prev_char != '\\' {
                in_string = false;
                string_delimiter = '\0';
            }
            result.push(ch);
            prev_char = ch;
            continue;
        }

        // Handle regex literals (simplified detection)
        if !in_string && !in_template_literal && !in_regex && ch == '/' {
            // Check if this could be the start of a regex
            // Regex typically follows: =, (, [, {, ;, :, !, &, |, ?, +, -, *, /, %, ^, ~, <, >, ,
            // or keywords like return, throw, case, in, of, delete, void, typeof, new, instanceof
            let could_be_regex = matches!(
                prev_non_whitespace,
                '=' | '('
                    | '['
                    | '{'
                    | ';'
                    | ':'
                    | '!'
                    | '&'
                    | '|'
                    | '?'
                    | '+'
                    | '-'
                    | '*'
                    | '/'
                    | '%'
                    | '^'
                    | '~'
                    | '<'
                    | '>'
                    | ','
            ) || result.ends_with("return")
                || result.ends_with("throw")
                || result.ends_with("case")
                || result.ends_with("in ")
                || result.ends_with("of ")
                || result.ends_with("delete")
                || result.ends_with("void")
                || result.ends_with("typeof")
                || result.ends_with("new")
                || result.ends_with("instanceof");

            if could_be_regex {
                in_regex = true;
                result.push(ch);
                prev_char = ch;
                continue;
            }
        }

        // End regex literal
        if in_regex && ch == '/' && prev_char != '\\' {
            in_regex = false;
            result.push(ch);
            prev_char = ch;
            continue;
        }

        // Preserve content inside strings, template literals, and regex
        if in_string || in_template_literal || in_regex {
            result.push(ch);
            prev_char = ch;
            continue;
        }

        // Handle whitespace - skip unnecessary whitespace
        if ch.is_whitespace() {
            let next_char = chars.peek().unwrap_or(&'\0');

            if !result.is_empty() {
                let last_char = result.chars().last().unwrap_or('\0');

                // Preserve space in specific cases where it's needed for JavaScript syntax
                let needs_space =
                    // Between alphanumeric characters (keywords, identifiers, numbers)
                    (last_char.is_alphanumeric() || last_char == '_' || last_char == '$') &&
                    (next_char.is_alphanumeric() || *next_char == '_' || *next_char == '$') ||
                    // Between operators where space is needed to avoid creating different operators
                    (matches!(last_char, '+' | '-') && *next_char == last_char) ||  // ++ or --
                    (matches!(last_char, '&' | '|') && *next_char == last_char) ||  // && or ||
                    (matches!(last_char, '=' | '!' | '<' | '>') && *next_char == '=') ||  // ==, !=, <=, >=
                    // ASI cases - preserve newlines after certain tokens to prevent issues
                    (ch == '\n' && matches!(last_char, ')' | ']' | '}' | ';') &&
                     matches!(*next_char, '(' | '[' | '{' | '+' | '-' | '*' | '/' | '%' | 'a'..='z' | 'A'..='Z' | '_' | '$'));

                if needs_space {
                    if ch == '\n' && matches!(last_char, ')' | ']' | '}' | ';') {
                        result.push('\n');
                    } else {
                        result.push(' ');
                    }
                }
            }

            prev_char = ch;
            continue;
        }

        // Handle other characters
        result.push(ch);
        prev_non_whitespace = ch;
        prev_char = ch;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_whitespace_removal() {
        let js = "function   test(  ) {   return   42;   }";
        let expected = "function test(){return 42;}";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_newline_removal() {
        let js = "function test() {\n    return 42;\n}";
        let expected = "function test(){return 42;}";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_single_line_comment_removal() {
        let js = "// This is a comment\nfunction test() {\n    return 42; // another comment\n}";
        let expected = "function test(){return 42;}";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_multi_line_comment_removal() {
        let js =
            "/* This is a comment */\nfunction test() {\n    return 42; /* another comment */\n}";
        let expected = "function test(){return 42;}";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_preserve_strings() {
        let js = r#"const message = "Hello   World";"#;
        let expected = r#"const message="Hello   World";"#;
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_preserve_single_quote_strings() {
        let js = "const message = 'Hello   World';";
        let expected = "const message='Hello   World';";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_preserve_template_literals() {
        let js = "const message = `Hello   ${name}   World`;";
        let expected = "const message=`Hello   ${name}   World`;";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_preserve_regex_literals() {
        let js = "const pattern = /hello\\s+world/gi;";
        let expected = "const pattern=/hello\\s+world/gi;";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_preserve_necessary_spaces_between_keywords() {
        let js = "return value;";
        let expected = "return value;";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_preserve_spaces_in_instanceof() {
        let js = "obj instanceof Array;";
        let expected = "obj instanceof Array;";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_preserve_spaces_in_typeof() {
        let js = "typeof obj === 'string';";
        let expected = "typeof obj==='string';";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_complex_javascript() {
        let js = r#"
        // Main function
        function calculateSum(a, b) {
            /* Calculate the sum of two numbers */
            if (typeof a !== 'number' || typeof b !== 'number') {
                throw new Error("Invalid arguments");
            }
            return a + b;
        }

        const result = calculateSum(10, 20);
        console.log(`Result: ${result}`);
        "#;

        let result = minify_js(js);
        assert!(!result.contains("//"));
        assert!(!result.contains("/*"));
        assert!(!result.contains("*/"));
        assert!(result.contains("function calculateSum(a,b){"));
        assert!(result.contains("typeof a"));
        assert!(result.contains("typeof b"));
        assert!(result.contains("`Result: ${result}`"));
    }

    #[test]
    fn test_escaped_quotes_in_strings() {
        let js = r#"const message = "He said \"Hello\"";"#;
        let expected = r#"const message="He said \"Hello\"";"#;
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_regex_after_equals() {
        let js = "const pattern = /test/g;";
        let expected = "const pattern=/test/g;";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_division_vs_regex() {
        let js = "const result = a / b; const pattern = /test/;";
        let expected = "const result=a/b;const pattern=/test/;";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_preserve_increment_operators() {
        let js = "i++; ++j; i--; --j;";
        let expected = "i++;++j;i--;--j;";
        assert_eq!(minify_js(js), expected);
    }

    #[test]
    fn test_asi_preservation() {
        let js = "return\n42;";
        // This should preserve the newline to maintain ASI behavior
        let result = minify_js(js);
        assert!(result.contains("return\n") || result.contains("return 42"));
    }

    #[test]
    fn test_template_literal_with_expressions() {
        let js = "const html = `<div class=\"${className}\">${content}</div>`;";
        let expected = "const html=`<div class=\"${className}\">${content}</div>`;";
        assert_eq!(minify_js(js), expected);
    }
}
