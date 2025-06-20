/// Converts double line breaks to HTML paragraphs.
///
/// # Arguments
/// * `input` - The input string with double line breaks
///
/// # Returns
/// * `String` - The HTML string with paragraphs
#[cfg(test)]
pub fn double_line_breaks_to_html(input: &str) -> String {
    input
        .split("\n\n")
        .map(|paragraph| format!("<p>{}</p>", paragraph))
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_line_breaks_to_html() {
        let markdown = r#"This is a line.
This is another line.

This is a new paragraph."#;
        let expected_html =
            "<p>This is a line.\nThis is another line.</p><p>This is a new paragraph.</p>";

        assert_eq!(double_line_breaks_to_html(markdown), expected_html);
    }
}
