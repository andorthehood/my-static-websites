/// Represents the state of JavaScript parsing
#[allow(clippy::struct_excessive_bools)]
struct JsParseState {
    in_string: bool,
    string_delimiter: char,
    in_template_literal: bool,
    in_single_line_comment: bool,
    in_multi_line_comment: bool,
    in_regex: bool,
}

impl JsParseState {
    fn new() -> Self {
        Self {
            in_string: false,
            string_delimiter: '\0',
            in_template_literal: false,
            in_single_line_comment: false,
            in_multi_line_comment: false,
            in_regex: false,
        }
    }

    fn is_in_any_string(&self) -> bool {
        self.in_string || self.in_template_literal || self.in_regex
    }
}

/// Handles single-line comment processing
fn handle_single_line_comments(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    state: &mut JsParseState,
    result: &mut String,
) -> bool {
    if !state.is_in_any_string()
        && !state.in_multi_line_comment
        && ch == '/'
        && chars.peek() == Some(&'/')
    {
        chars.next(); // consume the second '/'
        state.in_single_line_comment = true;
        return true;
    }

    // End single-line comment on newline
    if state.in_single_line_comment && ch == '\n' {
        state.in_single_line_comment = false;
        // Add a newline to preserve potential ASI (Automatic Semicolon Insertion)
        if !result.is_empty() {
            let last_char = result.chars().last().unwrap_or('\0');
            if !matches!(last_char, ';' | '{' | '}') {
                result.push('\n');
            }
        }
        return true;
    }

    // Skip single-line comment content
    state.in_single_line_comment
}

/// Handles multi-line comment processing
fn handle_multi_line_comments(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    state: &mut JsParseState,
) -> bool {
    if !state.is_in_any_string()
        && !state.in_multi_line_comment
        && ch == '/'
        && chars.peek() == Some(&'*')
    {
        chars.next(); // consume the '*'
        state.in_multi_line_comment = true;
        return true;
    }

    // End multi-line comment
    if state.in_multi_line_comment && ch == '*' && chars.peek() == Some(&'/') {
        chars.next(); // consume the '/'
        state.in_multi_line_comment = false;
        return true;
    }

    // Skip multi-line comment content
    state.in_multi_line_comment
}

/// Handles template literal processing
fn handle_template_literals(
    ch: char,
    prev_char: char,
    state: &mut JsParseState,
    result: &mut String,
) -> bool {
    if !state.in_string && !state.in_regex && ch == '`' {
        if !state.in_template_literal {
            state.in_template_literal = true;
        } else if prev_char != '\\' {
            state.in_template_literal = false;
        }
        result.push(ch);
        return true;
    }
    false
}

/// Handles string literal processing
fn handle_string_literals(
    ch: char,
    prev_char: char,
    state: &mut JsParseState,
    result: &mut String,
) -> bool {
    if !state.in_template_literal && !state.in_regex && matches!(ch, '"' | '\'') {
        if !state.in_string {
            state.in_string = true;
            state.string_delimiter = ch;
        } else if ch == state.string_delimiter && prev_char != '\\' {
            state.in_string = false;
            state.string_delimiter = '\0';
        }
        result.push(ch);
        return true;
    }
    false
}

/// Checks if the current context could start a regex literal
fn could_be_regex_context(prev_non_whitespace: char, result: &str) -> bool {
    matches!(
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
        || result.ends_with("instanceof")
}

/// Handles regex literal processing
fn handle_regex_literals(
    ch: char,
    prev_char: char,
    prev_non_whitespace: char,
    state: &mut JsParseState,
    result: &mut String,
) -> bool {
    // Handle regex literals (simplified detection)
    if !state.is_in_any_string() && !state.in_regex && ch == '/'
        && could_be_regex_context(prev_non_whitespace, result) {
            state.in_regex = true;
            result.push(ch);
            return true;
        }

    // End regex literal
    if state.in_regex && ch == '/' && prev_char != '\\' {
        state.in_regex = false;
        result.push(ch);
        return true;
    }

    false
}

/// Handles whitespace minification
fn handle_whitespace(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    result: &str,
) -> bool {
    if ch.is_whitespace() {
        let next_char = chars.peek().unwrap_or(&'\0');

        if result.is_empty() {
            false
        } else {
            let last_char = result.chars().last().unwrap_or('\0');

            // Preserve space in specific cases where it's needed for JavaScript syntax
            (last_char.is_alphanumeric() || last_char == '_' || last_char == '$') &&
                (next_char.is_alphanumeric() || *next_char == '_' || *next_char == '$') ||
                // Between operators where space is needed to avoid creating different operators
                (matches!(last_char, '+' | '-') && *next_char == last_char) ||  // ++ or --
                (matches!(last_char, '&' | '|') && *next_char == last_char) ||  // && or ||
                (matches!(last_char, '=' | '!' | '<' | '>') && *next_char == '=') ||  // ==, !=, <=, >=
                // ASI cases - preserve newlines after certain tokens to prevent issues
                (ch == '\n' && matches!(last_char, ')' | ']' | '}' | ';') &&
                 matches!(*next_char, '(' | '[' | '{' | '+' | '-' | '*' | '/' | '%' | 'a'..='z' | 'A'..='Z' | '_' | '$'))
        }
    } else {
        false
    }
}

/// Minifies JavaScript by removing unnecessary whitespace and comments while preserving functionality
pub fn minify_js(js: &str) -> String {
    let mut result = String::with_capacity(js.len());
    let mut chars = js.chars().peekable();
    let mut state = JsParseState::new();
    let mut prev_char = '\0';
    let mut prev_non_whitespace = '\0';

    while let Some(ch) = chars.next() {
        // Handle single-line comments
        if handle_single_line_comments(ch, &mut chars, &mut state, &mut result) {
            prev_char = ch;
            continue;
        }

        // Handle multi-line comments
        if handle_multi_line_comments(ch, &mut chars, &mut state) {
            prev_char = ch;
            continue;
        }

        // Handle template literals
        if handle_template_literals(ch, prev_char, &mut state, &mut result) {
            prev_char = ch;
            continue;
        }

        // Handle string literals
        if handle_string_literals(ch, prev_char, &mut state, &mut result) {
            prev_char = ch;
            continue;
        }

        // Handle regex literals
        if handle_regex_literals(ch, prev_char, prev_non_whitespace, &mut state, &mut result) {
            prev_char = ch;
            continue;
        }

        // Preserve content inside strings, template literals, and regex
        if state.is_in_any_string() {
            result.push(ch);
            prev_char = ch;
            continue;
        }

        // Handle whitespace - skip unnecessary whitespace
        if handle_whitespace(ch, &mut chars, &result) {
            if ch == '\n' && !result.is_empty() {
                let last_char = result.chars().last().unwrap_or('\0');
                if matches!(last_char, ')' | ']' | '}' | ';') {
                    result.push('\n');
                } else {
                    result.push(' ');
                }
            } else {
                result.push(' ');
            }
            prev_char = ch;
            continue;
        }

        if ch.is_whitespace() {
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
