/// Converts single line breaks to HTML `<br />` tags.
///
/// # Arguments
/// * `input` - The input string with single line breaks
///
/// # Returns
/// * `String` - The HTML string with `<br />` tags
pub fn single_line_breaks_to_html(input: &str) -> String {
    input.replace('\n', "<br />")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_breaks_to_html() {
        let markdown = r#"This is a line.
This is another line.

This is a new paragraph."#;
        let expected_html =
            "This is a line.<br />This is another line.<br /><br />This is a new paragraph.";

        assert_eq!(single_line_breaks_to_html(markdown), expected_html);
    }
}
