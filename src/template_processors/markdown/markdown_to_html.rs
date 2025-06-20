use super::list_to_html::list_to_html;
use super::single_line_breaks::single_line_breaks_to_html;

/// Converts markdown to HTML by applying various transformations.
///
/// # Arguments
/// * `input` - The markdown string to convert
///
/// # Returns
/// * `String` - The HTML string
pub fn markdown_to_html(input: &str) -> String {
    //let mut html = double_line_breaks_to_html(input);
    let mut html = list_to_html(input);
    html = single_line_breaks_to_html(&html);
    html
}
