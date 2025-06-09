pub fn double_line_breaks_to_html(input: &str) -> String {
    input
        .split("\n\n")
        .map(|paragraph| format!("<p>{}</p>", paragraph))
        .collect::<Vec<_>>()
        .join("")
}

pub fn single_line_breaks_to_html(input: &str) -> String {
    input.replace("\n", "<br />")
}

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
            result.push_str(&line.trim_start_matches('-').trim());
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

pub fn markdown_to_html(input: &str) -> String {
    //let mut html = double_line_breaks_to_html(input);
    let mut html = list_to_html(input);
    html = single_line_breaks_to_html(&html);
    html
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

    #[test]
    fn test_double_line_breaks_to_html() {
        let markdown = r#"This is a line.
This is another line.

This is a new paragraph."#;
        let expected_html =
            "<p>This is a line.\nThis is another line.</p><p>This is a new paragraph.</p>";

        assert_eq!(double_line_breaks_to_html(markdown), expected_html);
    }

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
