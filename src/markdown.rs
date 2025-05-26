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

    #[test]
    fn test_markdown_to_html_empty_input() {
        assert_eq!(markdown_to_html(""), "");
    }

    #[test]
    fn test_markdown_to_html_only_single_breaks() {
        let markdown = "Hello\nWorld";
        // list_to_html("Hello\nWorld") results in "HelloWorld" because it processes line by line
        // and concatenates them. No '\n' remains for single_line_breaks_to_html.
        let expected_html = "HelloWorld";
        assert_eq!(markdown_to_html(markdown), expected_html);
    }

    #[test]
    fn test_markdown_to_html_only_list() {
        let markdown = "- Item 1\n- Item 2";
        let expected_html = "<ul><li>Item 1</li><li>Item 2</li></ul>"; // single_line_breaks shouldn't affect this structure
        assert_eq!(markdown_to_html(markdown), expected_html);
    }

    #[test]
    fn test_markdown_to_html_list_with_spaces() {
        let markdown = "-   Item 1  \n- Item 2";
        let expected_html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
        assert_eq!(markdown_to_html(markdown), expected_html);
    }

    #[test]
    fn test_markdown_to_html_list_with_internal_line_breaks() {
        // Current list_to_html processes line by line.
        // single_line_breaks_to_html will convert the \n within list items.
        // The variable 'markdown' here was unused and referred to a more complex case.
        // The actual test uses 'markdown_with_internal_break' defined later.
        // list_to_html output for a simple case: <ul><li>Item 1 still item 1</li><li>Item 2</li></ul> (assuming trim works well)
        // markdown_to_html output:
        // <ul><li>Item 1<br />  still item 1</li><li>Item 2</li></ul> (after single_line_breaks)
        // Let's trace:
        // list_to_html:
        // Line 1: "- Item 1" -> `result = "<ul><li>Item 1</li>"` (in_list = true)
        // Line 2: "  still item 1" -> `result = "<ul><li>Item 1</li>  still item 1"` (in_list = false, because it doesn't start with '-')
        // Line 3: "- Item 2" -> `result = "<ul><li>Item 1</li>  still item 1</ul><ul><li>Item 2</li>"` (in_list = true)
        // This is not what we want. The current list_to_html is basic.
        // The test `test_list_to_html` already shows how non-list lines are handled.
        //
        // If markdown is:
        // - Item 1 line 1
        // - Item 1 line 2
        // This becomes two separate list items.
        //
        // If markdown is:
        // - Item 1 part1\n  Item 1 part2 (this is not standard markdown list continuation)
        //   Our parser would treat "  Item 1 part2" as a separate line not part of the list.
        //
        // Let's test the actual behavior of the composition:
        // The variable 'markdown_input' here was unused.
        // list_to_html("- Item 1\nNon-list line\n- Item 2") = "<ul><li>Item 1</li></ul>Non-list line<ul><li>Item 2</li></ul>"
        // single_line_breaks_to_html applied to that: "<ul><li>Item 1</li></ul>Non-list line<br /><ul><li>Item 2</li></ul>"
        // This seems to be an error in my manual trace for `test_list_to_html` in the original code.
        // Let's re-verify `test_list_to_html`'s expected output.
        // Original test:
        // let markdown = "\n- Item 1\n- Item 2\n- Item 3\n\nSome other text.\n\n- Item 1\n- Item 2\n";
        // let expected_html = "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>Some other text.<ul><li>Item 1</li><li>Item 2</li></ul>";
        // This test implies that blank lines and non-list lines correctly terminate the list.
        // And `markdown_to_html` applies `single_line_breaks_to_html` to the output of `list_to_html`.
        //
        // So, for "list item with \n":
        let markdown_with_internal_break = "- Item 1\n  still part of item 1 conceptually, but not by current parser\n- Item 2";
        // list_to_html(markdown_with_internal_break):
        // 1. "- Item 1" -> `<ul><li>Item 1</li>`
        // 2. "  still part of item 1..." -> `</ul>  still part of item 1...` (list ends)
        // 3. "- Item 2" -> `<ul><li>Item 2</li></ul>` (new list starts)
        // So, list_to_html(markdown_with_internal_break) = "<ul><li>Item 1</li></ul>  still part of item 1 conceptually, but not by current parser<ul><li>Item 2</li></ul>"
        // Then, markdown_to_html applies single_line_breaks to this string. Since there are no '\n'
        // characters in the output of list_to_html for this input, single_line_breaks_to_html does nothing.
        let expected = "<ul><li>Item 1</li></ul>  still part of item 1 conceptually, but not by current parser<ul><li>Item 2</li></ul>";
        assert_eq!(markdown_to_html(markdown_with_internal_break), expected);
    }

    #[test]
    fn test_markdown_to_html_text_before_and_after_list() {
        let markdown = "Text before\n- Item 1\n- Item 2\nText after";
        // list_to_html: "Text before<ul><li>Item 1</li><li>Item 2</li></ul>Text after"
        // markdown_to_html: "Text before<br /><ul><li>Item 1</li><li>Item 2</li></ul>Text after"
        // No, list_to_html:
        // 1. "Text before" -> `Text before`
        // 2. "- Item 1" -> `<ul><li>Item 1</li>`
        // 3. "- Item 2" -> `<li>Item 2</li>`
        // 4. "Text after" -> `</ul>Text after`
        // So, list_to_html(markdown) = "Text before<ul><li>Item 1</li><li>Item 2</li></ul>Text after"
        // Then, markdown_to_html applies single_line_breaks:
        // "Text before<br /><ul><li>Item 1</li><li>Item 2</li></ul>Text after"
        // This needs careful checking of how `input.lines()` and `push_str(line)` interact.
        // If "Text before" is a line, then `result.push_str(line)` is called.
        // `list_to_html` logic:
        // line = "Text before" -> !in_list, result.push_str("Text before")
        // line = "- Item 1"    -> in_list=true, result.push_str("<ul><li>Item 1</li>")
        // line = "- Item 2"    -> result.push_str("<li>Item 2</li>")
        // line = "Text after"  -> in_list=false, result.push_str("</ul>Text after")
        // Correct. list_to_html output is "Text before<ul><li>Item 1</li><li>Item 2</li></ul>Text after"
        // No '\n' characters remain in this string for single_line_breaks_to_html to process.
        let expected_html = "Text before<ul><li>Item 1</li><li>Item 2</li></ul>Text after";
        assert_eq!(markdown_to_html(markdown), expected_html);
    }
}
