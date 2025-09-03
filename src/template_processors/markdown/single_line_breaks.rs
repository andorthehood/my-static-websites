#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("single_line_breaks_x86_64.s"));

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn count_newlines(input: *const u8, input_len: usize) -> usize;
    fn single_line_breaks_scan(
        input: *const u8,
        input_len: usize,
        output: *mut u8,
        output_capacity: usize,
    ) -> usize;
}

#[cfg(target_arch = "x86_64")]
/// Converts single line breaks to HTML `<br />` tags - `x86_64` assembly optimized version.
///
/// # Arguments
/// * `input` - The input string with single line breaks
///
/// # Returns
/// * `String` - The HTML string with `<br />` tags
pub fn single_line_breaks_to_html(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let input_bytes = input.as_bytes();

    // Count newlines to calculate output size needed
    // Each '\n' becomes "<br />" (1 char -> 6 chars, so +5 per newline)
    let newline_count = unsafe { count_newlines(input_bytes.as_ptr(), input_bytes.len()) };
    let output_capacity = input_bytes.len() + newline_count * 5;

    let mut output = vec![0u8; output_capacity];

    let actual_len = unsafe {
        single_line_breaks_scan(
            input_bytes.as_ptr(),
            input_bytes.len(),
            output.as_mut_ptr(),
            output_capacity,
        )
    };

    if actual_len == usize::MAX {
        // Fallback to Rust implementation if assembly failed
        return input.replace('\n', "<br />");
    }

    output.truncate(actual_len);
    // SAFETY: We only write valid ASCII characters in the assembly function
    unsafe { String::from_utf8_unchecked(output) }
}

#[cfg(not(target_arch = "x86_64"))]
/// Converts single line breaks to HTML `<br />` tags - pure Rust fallback version.
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

    #[test]
    fn test_single_line_breaks_edge_cases() {
        // Test empty string
        assert_eq!(single_line_breaks_to_html(""), "");

        // Test single newline
        assert_eq!(single_line_breaks_to_html("\n"), "<br />");

        // Test multiple consecutive newlines
        assert_eq!(single_line_breaks_to_html("\n\n\n"), "<br /><br /><br />");

        // Test string without newlines
        assert_eq!(
            single_line_breaks_to_html("no newlines here"),
            "no newlines here"
        );

        // Test newline at start
        assert_eq!(
            single_line_breaks_to_html("\nstart with newline"),
            "<br />start with newline"
        );

        // Test newline at end
        assert_eq!(
            single_line_breaks_to_html("end with newline\n"),
            "end with newline<br />"
        );

        // Test mixed content
        assert_eq!(
            single_line_breaks_to_html("line1\nline2\nline3"),
            "line1<br />line2<br />line3"
        );
    }

    #[test]
    fn test_single_line_breaks_large_input() {
        // Test with larger input to ensure assembly handles it correctly
        let input = (0..1000).fold(String::new(), |mut acc, i| {
            use std::fmt::Write;
            write!(&mut acc, "Line {i}\n").unwrap();
            acc
        });
        let result = single_line_breaks_to_html(&input);

        // Verify the result contains the expected number of <br /> tags
        let br_count = result.matches("<br />").count();
        assert_eq!(br_count, 1000);

        // Verify some sample content
        assert!(result.contains("Line 0<br />"));
        assert!(result.contains("Line 999<br />"));
    }
}
