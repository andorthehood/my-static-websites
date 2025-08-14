use super::comments::CommentHandler;
use super::hex_colors::optimize_hex_color;
use super::strings::StringHandler;
use super::whitespace::WhitespaceHandler;

/// Minifies CSS by removing unnecessary whitespace while preserving functionality
pub fn minify_css(css: &str) -> String {
    let mut result = String::with_capacity(css.len());
    let mut chars = css.chars().peekable();
    let mut string_handler = StringHandler::new();
    let mut comment_handler = CommentHandler::new();
    let mut prev_char = '\0';

    while let Some(ch) = chars.next() {
        match ch {
            // Handle string literals (preserve whitespace inside strings)
            '"' | '\'' => {
                if string_handler.handle_quote(ch, prev_char, comment_handler.is_in_comment()) {
                    result.push(ch);
                }
            }

            // Handle CSS comments /* ... */
            '/' => {
                if comment_handler.handle_comment_start(&mut chars, string_handler.is_in_string()) {
                    result.push(ch);
                }
            }

            '*' => {
                if comment_handler.handle_comment_end(&mut chars, string_handler.is_in_string()) {
                    result.push(ch);
                }
            }

            // Skip comment content
            _ if comment_handler.is_in_comment() => {
                // Do nothing, skip comment content
            }

            // Handle hex colors for optimization
            '#' if !string_handler.is_in_string() && !comment_handler.is_in_comment() => {
                result.push('#');
                let optimized_color = optimize_hex_color(&mut chars);
                result.push_str(&optimized_color);
            }

            // Handle whitespace - skip all whitespace when not in strings
            ' ' | '\t' | '\r' | '\n' if !string_handler.is_in_string() => {
                // Skip all whitespace - we'll add back only necessary spaces
                let next_char = chars.peek().unwrap_or(&'\0');

                if WhitespaceHandler::should_preserve_space(&result, *next_char) {
                    result.push(' ');
                }
            }

            // Handle other characters
            _ if !comment_handler.is_in_comment() => {
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
        let expected = "box-shadow:inset 1rem 1rem 0px #fff;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_preserve_spaces_after_comma_before_hash() {
        let css = "background: linear-gradient(rgba(255,0,0,0.5), #ff0000);";
        let expected = "background:linear-gradient(rgba(255,0,0,0.5), #f00);";
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
    fn test_preserve_spaces_before_negative_numbers() {
        let css = "box-shadow: -1rem -1rem 0 #999999;";
        let expected = "box-shadow:-1rem -1rem 0 #999;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_preserve_spaces_between_numbers_and_hash() {
        let css = "box-shadow: 1rem 1rem 0 #999999;";
        let expected = "box-shadow:1rem 1rem 0 #999;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_zero_followed_by_hash() {
        let css = "box-shadow: 0 #fff;";
        let expected = "box-shadow:0 #fff;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_complex_box_shadow_cases() {
        let css = "box-shadow: 1rem 1rem 0 #cccccc;";
        let expected = "box-shadow:1rem 1rem 0 #ccc;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_three_value_box_shadow() {
        let css = "box-shadow: 0 1rem 0 #999999;";
        let expected = "box-shadow:0 1rem 0 #999;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_bare_zero_before_hash() {
        let css = "margin: 0 #ff0000;";
        let expected = "margin:0 #f00;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_full_css_like_original() {
        let css = ".bar{width:9rem;background:#ffffff;box-shadow:0 1rem 0 #999999;height:1rem;margin-bottom:1.5rem;}";
        let expected = ".bar{width:9rem;background:#fff;box-shadow:0 1rem 0 #999;height:1rem;margin-bottom:1.5rem;}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_css_descendant_selectors() {
        let css = ".foo .bar { color: red; }";
        let expected = ".foo .bar{color:red;}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_css_multiple_descendant_selectors() {
        let css = ".header .title .text { font-size: 12px; }";
        let expected = ".header .title .text{font-size:12px;}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_css_compound_vs_descendant_selectors() {
        let css = ".foo.bar { color: blue; } .foo .bar { color: red; }";
        let expected = ".foo.bar{color:blue;}.foo .bar{color:red;}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_css_id_selectors() {
        let css = "div #myid { color: green; } #parent .child { color: yellow; }";
        let expected = "div #myid{color:green;}#parent .child{color:yellow;}";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_hex_color_optimization() {
        let css = "color: #999999; background: #aabbcc;";
        let expected = "color:#999;background:#abc;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_hex_color_no_optimization() {
        let css = "color: #123456; background: #abcdef;";
        let expected = "color:#123456;background:#abcdef;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_mixed_hex_colors() {
        let css = "border: 1px solid #000000; color: #ff00ff; background: #123abc;";
        let expected = "border:1px solid #000;color:#f0f;background:#123abc;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_short_hex_colors_preserved() {
        let css = "color: #fff; background: #000;";
        let expected = "color:#fff;background:#000;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_complex_box_shadow_with_negative_values() {
        let css = "box-shadow: inset -1rem -1rem 0px #999999;";
        let expected = "box-shadow:inset -1rem -1rem 0px #999;";
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_escaped_quotes_in_strings() {
        let css = r#"body::before { content: "He said \"Hello\""; }"#;
        let expected = r#"body::before{content:"He said \"Hello\"";}"#;
        assert_eq!(minify_css(css), expected);
    }

    #[test]
    fn test_assembly_integration() {
        // Test CSS with hex colors that should be optimized
        let test_css = "color: #999999; background: #aabbcc; border: #123456;";
        let minified = minify_css(test_css);
        
        // Expected: color:#999;background:#abc;border:#123456;
        let expected = "color:#999;background:#abc;border:#123456;";
        
        assert_eq!(minified, expected, "Assembly-optimized hex color function should work correctly");
    }
}