/// Converts markdown lists to HTML unordered lists.
///
/// # Arguments
/// * `input` - The input string containing markdown lists
///
/// # Returns
/// * `String` - The HTML string with converted lists
pub fn list_to_html(input: &str) -> String {
    let mut result = String::new();
    let mut in_list = false;

    for line in input.lines() {
        if line.trim_start().starts_with('-') {
            if !in_list {
                in_list = true;
                result.push_str("<ul>");
            }
            result.push_str("<li>");
            result.push_str(line.trim_start_matches('-').trim());
            result.push_str("</li>");
        } else {
            if in_list {
                in_list = false;
                result.push_str("</ul>");
            }
            result.push_str(line);
        }
    }

    if in_list {
        result.push_str("</ul>");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_to_html() {
        let markdown = r#"
- Item 1
- Item 2
- Item 3

Some other text.

- Item 1
- Item 2
"#;

        let expected_html = "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>Some other text.<ul><li>Item 1</li><li>Item 2</li></ul>";
        assert_eq!(list_to_html(markdown), expected_html);
    }
}
