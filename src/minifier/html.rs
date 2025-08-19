/// State for tracking parser context
#[derive(Default)]
struct HtmlParserState {
    in_tag: bool,
    in_script: bool,
    in_style: bool,
    in_pre: bool,
    in_textarea: bool,
    in_string: bool,
    string_delimiter: char,
    in_comment: bool,
    tag_name: String,
    collecting_tag_name: bool,
}

impl HtmlParserState {
    fn in_preservable_content(&self) -> bool {
        self.in_script || self.in_style || self.in_pre || self.in_textarea
    }

    fn should_preserve_content(&self) -> bool {
        self.in_preservable_content() || self.in_string
    }
}

/// Handle HTML comments
fn handle_comment_start(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    state: &mut HtmlParserState,
    result: &mut String,
) -> bool {
    if chars.peek() == Some(&'!') {
        // Look ahead to see if this is a comment
        let mut lookahead = chars.clone();
        lookahead.next(); // consume '!'
        if lookahead.next() == Some('-') && lookahead.next() == Some('-') {
            // This is a comment, skip it entirely
            chars.next(); // consume '!'
            chars.next(); // consume first '-'
            chars.next(); // consume second '-'
            state.in_comment = true;
            return true;
        }
    }

    if !state.in_comment {
        state.in_tag = true;
        state.collecting_tag_name = true;
        state.tag_name.clear();
        result.push('<');
    }
    true
}

/// Handle comment end
fn handle_comment_end(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    state: &mut HtmlParserState,
) -> bool {
    if state.in_comment && chars.peek() == Some(&'-') {
        chars.next(); // consume second '-'
        if chars.peek() == Some(&'>') {
            chars.next(); // consume '>'
            state.in_comment = false;
        }
        return true;
    }
    false
}

/// Handle tag closing and update parser state
fn handle_tag_close(state: &mut HtmlParserState, result: &mut String) {
    result.push('>');

    // Check if we're entering special content areas
    let tag_lower = state.tag_name.to_lowercase();
    match tag_lower.as_str() {
        "script" => state.in_script = true,
        "style" => state.in_style = true,
        "pre" | "code" => state.in_pre = true,
        "textarea" => state.in_textarea = true,
        _ => {}
    }

    // Check for closing tags
    if let Some(stripped) = state.tag_name.strip_prefix('/') {
        let closing_tag = stripped.to_lowercase();
        match closing_tag.as_str() {
            "script" => state.in_script = false,
            "style" => state.in_style = false,
            "pre" | "code" => state.in_pre = false,
            "textarea" => state.in_textarea = false,
            _ => {}
        }
    }

    state.in_tag = false;
    state.collecting_tag_name = false;
}

/// Handle tag name collection
fn handle_tag_name_collection(
    ch: char,
    state: &mut HtmlParserState,
    result: &mut String,
) -> bool {
    if state.collecting_tag_name && !ch.is_whitespace() && ch != '>' {
        if ch.is_alphabetic() || ch == '/' {
            state.tag_name.push(ch);
        } else {
            state.collecting_tag_name = false;
        }
        result.push(ch);
        return true;
    }
    false
}

/// Handle string delimiters inside tags
fn handle_string_delimiters(
    ch: char,
    state: &mut HtmlParserState,
    prev_char: char,
    result: &mut String,
) -> bool {
    if (ch == '"' || ch == '\'') && state.in_tag && !state.in_comment {
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

/// Check if a character represents content (text, emoji, unicode, etc.)
fn is_content_char(c: char) -> bool {
    c.is_alphanumeric() || c.len_utf8() > 1 || c.is_alphabetic()
}

/// Handle whitespace outside tags
fn handle_content_whitespace(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    state: &HtmlParserState,
    result: &mut String,
) {
    if state.in_tag || state.in_preservable_content() {
        return;
    }

    // Skip consecutive whitespace
    while chars.peek().is_some_and(|c| c.is_whitespace()) {
        chars.next();
    }

    let next_char = chars.peek().unwrap_or(&'\0');

    if !result.is_empty() {
        let last_char = result.chars().last().unwrap_or('\0');

        // Preserve space between:
        // - content characters (words, emojis, unicode)
        // - after punctuation (comma, period, etc.) and before content
        // - content and tags
        let should_preserve_space = (is_content_char(last_char) && is_content_char(*next_char))
            || (is_content_char(last_char) && *next_char == '<')
            || (last_char == '>' && is_content_char(*next_char))
            || (matches!(last_char, ',' | '.' | ';' | ':' | '!' | '?')
                && is_content_char(*next_char));

        if should_preserve_space {
            result.push(' ');
        }
    }
}

/// Handle whitespace inside tags
fn handle_tag_whitespace(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    state: &mut HtmlParserState,
    result: &mut String,
) {
    if !state.in_tag || state.in_string {
        return;
    }

    // Whitespace after the tag name means we've finished collecting it
    state.collecting_tag_name = false;
    let next_char = chars.peek().unwrap_or(&'\0');

    if !result.is_empty() {
        let last_char = result.chars().last().unwrap_or('\0');

        // Preserve single space between attributes
        if !last_char.is_whitespace() && !next_char.is_whitespace() && *next_char != '>' {
            result.push(' ');
        }
    }
}

/// Minifies HTML by removing unnecessary whitespace while preserving functionality
pub fn minify_html(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut chars = html.chars().peekable();
    let mut state = HtmlParserState::default();
    let mut prev_char = '\0';

    while let Some(ch) = chars.next() {
        match ch {
            // Handle HTML comments
            '<' if !state.in_string => {
                if handle_comment_start(&mut chars, &mut state, &mut result) {
                    continue;
                }
            }

            // End HTML comment
            '-' => {
                if handle_comment_end(&mut chars, &mut state) {
                    continue;
                }
            }

            // Skip comment content
            _ if state.in_comment => continue,

            // Handle end of tag
            '>' if state.in_tag && !state.in_string && !state.in_comment => {
                handle_tag_close(&mut state, &mut result);
                prev_char = ch;
                continue;
            }

            // Collect tag names
            _ if handle_tag_name_collection(ch, &mut state, &mut result) => {
                prev_char = ch;
                continue;
            }

            // Handle quotes inside tags
            _ if handle_string_delimiters(ch, &mut state, prev_char, &mut result) => {
                prev_char = ch;
                continue;
            }

            // Preserve content inside special tags and strings
            _ if state.should_preserve_content() => {
                result.push(ch);
                prev_char = ch;
                continue;
            }

            // Handle whitespace - the main minification logic
            ' ' | '\t' | '\r' | '\n' => {
                if state.in_tag {
                    handle_tag_whitespace(&mut chars, &mut state, &mut result);
                } else {
                    handle_content_whitespace(&mut chars, &state, &mut result);
                }
                prev_char = ch;
                continue;
            }

            // Handle other characters
            _ if !state.in_comment => {
                if state.collecting_tag_name && !ch.is_alphabetic() && ch != '/' {
                    state.collecting_tag_name = false;
                }
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
        let html = "<div>   Hello   World   </div>";
        let result = minify_html(html);
        // Should remove excessive whitespace but preserve space between words
        assert!(result.contains("Hello World"));
        assert!(!result.contains("   "));
        assert!(result.len() < html.len());
    }

    #[test]
    fn test_newline_removal() {
        let html = "<div>\n    Hello World\n</div>";
        let result = minify_html(html);
        // Should remove newlines and excessive whitespace
        assert!(result.contains("Hello World"));
        assert!(!result.contains('\n'));
        assert!(result.len() < html.len());
    }

    #[test]
    fn test_comment_removal() {
        let html = "<!-- This is a comment --><div>Hello World</div><!-- Another comment -->";
        let expected = "<div>Hello World</div>";
        assert_eq!(minify_html(html), expected);
    }

    #[test]
    fn test_preserve_pre_content() {
        let html = "<pre>    Hello    World    </pre>";
        let expected = "<pre>    Hello    World    </pre>";
        assert_eq!(minify_html(html), expected);
    }

    #[test]
    fn test_preserve_script_content() {
        let html = "<script>  var x = 1;  \n  var y = 2;  </script>";
        let expected = "<script>  var x = 1;  \n  var y = 2;  </script>";
        assert_eq!(minify_html(html), expected);
    }

    #[test]
    fn test_preserve_style_content() {
        let html = "<style>  .class {  color: red;  }  </style>";
        let expected = "<style>  .class {  color: red;  }  </style>";
        assert_eq!(minify_html(html), expected);
    }

    #[test]
    fn test_preserve_textarea_content() {
        let html = "<textarea>    This has    spaces    </textarea>";
        let expected = "<textarea>    This has    spaces    </textarea>";
        assert_eq!(minify_html(html), expected);
    }

    #[test]
    fn test_preserve_attribute_values() {
        let html = r#"<div class="my class" id="test">Hello</div>"#;
        let expected = r#"<div class="my class" id="test">Hello</div>"#;
        assert_eq!(minify_html(html), expected);
    }

    #[test]
    fn test_preserve_single_quotes() {
        let html = r#"<div class='my class' id='test'>Hello</div>"#;
        let expected = r#"<div class='my class' id='test'>Hello</div>"#;
        assert_eq!(minify_html(html), expected);
    }

    #[test]
    fn test_complex_html() {
        let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <!-- Meta tags -->
            <title>Test Page</title>
            <style>
                .container {
                    margin: 10px;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Hello World</h1>
                <p>This is a test paragraph.</p>
                <pre>
                    This content should
                    preserve    formatting
                </pre>
                <script>
                    var message = "Hello World";
                    console.log(message);
                </script>
            </div>
        </body>
        </html>
        "#;

        let result = minify_html(html);
        assert!(!result.contains("<!--"));
        assert!(!result.contains("-->"));
        assert!(result.contains("<title>Test Page</title>"));
        assert!(result.contains("preserve    formatting"));
        assert!(result.contains(r#"var message = "Hello World";"#));
        assert!(result.len() < html.len());
    }

    #[test]
    fn test_multiple_spaces_between_tags() {
        let html = "<div>   </div>   <span>   </span>";
        let expected = "<div></div><span></span>";
        assert_eq!(minify_html(html), expected);
    }

    #[test]
    fn test_nested_tags() {
        let html = "<div>  <span>  Hello  </span>  </div>";
        let result = minify_html(html);
        // Should remove excessive whitespace around and within tags
        assert!(result.contains("<span>"));
        assert!(result.contains("Hello"));
        assert!(!result.contains("  "));
        assert!(result.len() < html.len());
    }

    #[test]
    fn test_self_closing_tags() {
        let html = "<img src='test.jpg' />  <br />  <hr />";
        let expected = "<img src='test.jpg' /><br /><hr />";
        assert_eq!(minify_html(html), expected);
    }

    #[test]
    fn test_preserve_significant_whitespace() {
        let html = "<p>Hello <strong>bold</strong> world</p>";
        let expected = "<p>Hello <strong>bold</strong> world</p>";
        assert_eq!(minify_html(html), expected);
    }

    #[test]
    fn test_mixed_content() {
        let html = r#"<div>
            <script>
                // This is JavaScript
                var x = 1;
            </script>
            <style>
                /* This is CSS */
                .test { color: red; }
            </style>
            <p>Regular HTML content</p>
        </div>"#;

        let result = minify_html(html);
        assert!(result.contains("var x = 1;"));
        assert!(result.contains(".test { color: red; }"));
        assert!(result.contains("<p>Regular HTML content</p>"));
        assert!(result.len() < html.len());
    }

    #[test]
    fn test_style_tag_with_following_elements() {
        let html = r#"<style>
body { margin: 0; }
</style>
<link rel="test" />"#;
        let result = minify_html(html);
        // The CSS should be preserved but the newline between tags should be removed
        assert!(result.contains("body { margin: 0; }"));
        assert!(result.contains("<link"));
        // The key test: after </style> there should be no newline before <link
        assert!(!result.contains("</style>\n<link"));
        assert!(result.len() < html.len());
    }

    #[test]
    fn test_preserve_spaces_after_punctuation() {
        let html = r#"<p>Hello, world. How are you? Fine; thanks: great!</p>"#;
        let result = minify_html(html);

        // Spaces after punctuation should be preserved
        assert!(result.contains("Hello, world"));
        assert!(result.contains("world. How"));
        assert!(result.contains("you? Fine"));
        assert!(result.contains("Fine; thanks"));
        assert!(result.contains("thanks: great"));

        // Should not contain double spaces or other whitespace issues
        assert!(!result.contains("  "));

        println!("Result: {}", result);
    }

    #[test]
    fn test_preserve_spaces_around_emojis() {
        let html = r#"<p>Hello 🌍 world! I love 📸 photography.</p>"#;
        let result = minify_html(html);

        // Spaces around emojis should be preserved
        assert!(result.contains("Hello 🌍 world"));
        assert!(result.contains("love 📸 photography"));

        // Should not contain missing spaces around emojis
        assert!(!result.contains("Hello🌍world"));
        assert!(!result.contains("love📸photography"));

        println!("Result: {}", result);
    }

    #[test]
    fn test_preserve_pre_with_class_content() {
        let html = "<pre class=\"pretty\">\n    line 1\n    line 2\n</pre>";
        let expected_contains = "line 1\n    line 2";
        let result = minify_html(html);
        assert!(result.contains(expected_contains));
    }

    #[test]
    fn test_preserve_code_with_class_content() {
        let html = "<code class=\"lang-rs\">fn main() {\n    println!(\\\"hi\\\");\n}</code>";
        let expected_contains = "main() {\n    println!";
        let result = minify_html(html);
        assert!(result.contains(expected_contains));
    }
}
