use crate::template_processors::liquid::utils::find_equal::find_equal_index;

use crate::error::{Error, Result};
use std::iter::Peekable;
use std::str::Chars;

/// Represents a parsed liquid tag with its content and position
#[cfg(test)]
#[derive(Debug, PartialEq)]
pub struct LiquidTag {
    pub start: usize,
    pub end: usize,
    pub tag_type: String,
    pub content: String,
}

/// Represents a parsed tag block with its boundaries and content
#[derive(Debug, PartialEq)]
pub struct TagBlock {
    pub start: usize,
    pub end: usize,
    pub tag_content: String,
    pub inner_content: String,
}

/// Finds the next liquid tag starting from the given position
/// Returns the tag if found, otherwise None
#[cfg(test)]
pub fn find_next_liquid_tag(template: &str, start_pos: usize) -> Option<LiquidTag> {
    let template_slice = &template[start_pos..];
    let tag_start_pos = template_slice.find("{%")?;
    let tag_start = start_pos + tag_start_pos;

    let remaining = &template[tag_start + 2..];
    let tag_end_pos = remaining.find("%}")?;
    let tag_end = tag_start + 2 + tag_end_pos + 2;

    let tag_content = &template[tag_start + 2..tag_end - 2].trim();

    // Extract tag type (first word)
    let tag_type = tag_content
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string();

    Some(LiquidTag {
        start: tag_start,
        end: tag_end,
        tag_type,
        content: tag_content.to_string(),
    })
}

/// Finds a complete tag block (e.g., {% if %}...{% endif %}) starting from a position
pub fn find_tag_block(
    template: &str,
    start_tag: &str,
    end_tag: &str,
    start_pos: usize,
) -> Option<TagBlock> {
    let tag_start = template[start_pos..]
        .find(start_tag)
        .map(|pos| start_pos + pos)?;

    // Find where the opening tag ends
    let opening_tag_end = template[tag_start..]
        .find("%}")
        .map(|pos| tag_start + pos + 2)?;

    // Find the closing tag
    let tag_end = template[opening_tag_end..]
        .find(end_tag)
        .map(|pos| opening_tag_end + pos + end_tag.len())?;

    // Extract tag content (the condition/parameters in the opening tag)
    let tag_content_start = tag_start + start_tag.len();
    let tag_content_end = opening_tag_end - 2; // Before "%}"
    let tag_content = template[tag_content_start..tag_content_end]
        .trim()
        .to_string();

    // Extract inner content
    let inner_content = template[opening_tag_end..tag_end - end_tag.len()].to_string();

    Some(TagBlock {
        start: tag_start,
        end: tag_end,
        tag_content,
        inner_content,
    })
}

/// Skips whitespace characters in a character iterator
pub fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        if !is_ascii_whitespace_char(c) {
            break;
        }
        chars.next();
    }
}

/// Optimized check for ASCII whitespace characters
#[cfg(target_arch = "x86_64")]
fn is_ascii_whitespace_char(c: char) -> bool {
    if !c.is_ascii() {
        return c.is_whitespace(); // fallback for Unicode
    }
    let b = c as u8;
    unsafe { is_ascii_whitespace_scan(b) != 0 }
}

#[cfg(not(target_arch = "x86_64"))]
fn is_ascii_whitespace_char(c: char) -> bool {
    c.is_whitespace()
}

#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("is_ascii_whitespace_x86_64.s"));

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn is_ascii_whitespace_scan(byte: u8) -> u8;
}

/// Reads content until finding a closing liquid tag pattern
pub fn read_until_closing_tag(chars: &mut Peekable<Chars>) -> Result<String> {
    let mut content = String::new();
    let mut found_closing = false;

    while let Some(c) = chars.next() {
        if c == '%' && chars.peek() == Some(&'}') {
            chars.next(); // Skip '}'
            found_closing = true;
            break;
        }
        content.push(c);
    }

    if !found_closing {
        return Err(Error::Liquid("Unclosed liquid tag".to_string()));
    }

    Ok(content)
}

/// Detects if the current position in a character iterator is at the start of a liquid tag
#[cfg(test)]
pub fn detect_liquid_tag_start(chars: &mut Peekable<Chars>) -> bool {
    if let Some(&'{') = chars.peek() {
        let mut temp_chars = chars.clone();
        temp_chars.next(); // consume '{'
        if let Some(&'%') = temp_chars.peek() {
            chars.next(); // consume '{'
            chars.next(); // consume '%'
            return true;
        }
    }
    false
}

/// Parses an assignment expression (variable = value)
pub fn parse_assignment(content: &str) -> Option<(String, String)> {
    let idx = find_equal_index(content.as_bytes())?;
    let (left, right) = content.split_at(idx);
    // ensure there is exactly one '='
    if right[1..].bytes().any(|b| b == b'=') {
        return None;
    }
    Some((left.trim().to_string(), right[1..].trim().to_string()))
}

/// Parses a for loop expression (item in collection)
#[cfg(test)]
pub fn parse_for_loop_parts(content: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = content.split(" in ").collect();
    if parts.len() == 2 {
        Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
    } else {
        None
    }
}

/// Checks if a string represents a specific tag type
#[cfg(test)]
pub fn is_tag_type(tag_content: &str, tag_type: &str) -> bool {
    tag_content.trim().starts_with(tag_type)
}

/// Extracts the condition or parameter part from a tag
pub fn extract_tag_parameter(tag_content: &str, tag_type: &str) -> Option<String> {
    let trimmed = tag_content.trim();
    if let Some(stripped) = trimmed.strip_prefix(tag_type) {
        let param = stripped.trim();
        if param.is_empty() {
            None
        } else {
            Some(param.to_string())
        }
    } else {
        None
    }
}

/// Extracts the inner content of a full liquid tag string for a given tag name.
/// Example: given "{% include header.liquid %}", tag_name "include" -> returns Some("header.liquid").
pub fn extract_tag_inner<'a>(full_tag: &'a str, tag_name: &str) -> Option<&'a str> {
    let trimmed = full_tag.trim();
    let prefix = format!("{{% {}", tag_name);
    if !trimmed.starts_with(&prefix) || !trimmed.ends_with("%}") {
        return None;
    }
    Some(trimmed[prefix.len()..trimmed.len() - 2].trim())
}

/// Reads a nested balanced block for arbitrary start/end keywords using a character iterator.
/// Increments depth when encountering `{% <start_keyword> ... %}` and decrements on `{% <end_keyword> %}`.
/// Returns the collected inner content (excluding the closing end tag) at depth 0.
pub fn read_nested_block(
    chars: &mut Peekable<Chars>,
    start_keyword: &str,
    end_keyword: &str,
) -> Result<String> {
    let mut content = String::new();
    let mut depth: i32 = 1;

    while depth > 0 {
        let Some(c) = chars.next() else {
            return Err(Error::Liquid(format!(
                "Unclosed block - missing {{% {} %}}",
                end_keyword
            )));
        };

        if c == '{' && chars.peek() == Some(&'%') {
            chars.next(); // consume '%'
            let mut inner_tag = String::new();

            // Read tag content until %}
            while let Some(tc) = chars.next() {
                if tc == '%' && chars.peek() == Some(&'}') {
                    chars.next(); // consume '}'
                    break;
                }
                inner_tag.push(tc);
            }

            let trimmed = inner_tag.trim();
            if trimmed.starts_with(start_keyword) {
                depth += 1;
            } else if trimmed == end_keyword {
                depth -= 1;
            }

            if depth > 0 {
                content.push_str("{% ");
                content.push_str(trimmed);
                content.push_str(" %}");
            }
        } else if depth > 0 {
            content.push(c);
        }
    }

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_next_liquid_tag() {
        let template = "Hello {% if condition %} world";
        let tag = find_next_liquid_tag(template, 0).unwrap();

        assert_eq!(tag.start, 6);
        assert_eq!(tag.end, 24);
        assert_eq!(tag.tag_type, "if");
        assert_eq!(tag.content, "if condition");
    }

    #[test]
    fn test_find_next_liquid_tag_not_found() {
        let template = "Hello world";
        let tag = find_next_liquid_tag(template, 0);
        assert!(tag.is_none());
    }

    #[test]
    fn test_skip_whitespace() {
        let mut chars = "   hello".chars().peekable();
        skip_whitespace(&mut chars);
        assert_eq!(chars.next(), Some('h'));
    }

    #[test]
    fn test_read_until_closing_tag() {
        let mut chars = " if condition %}".chars().peekable();
        let content = read_until_closing_tag(&mut chars).unwrap();
        assert_eq!(content, " if condition ");
    }

    #[test]
    fn test_read_until_closing_tag_unclosed() {
        let mut chars = " if condition".chars().peekable();
        let result = read_until_closing_tag(&mut chars);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_tag_block() {
        let template = "before {% if condition %}content{% endif %} after";
        let result = find_tag_block(template, "{% if", "{% endif %}", 0).unwrap();

        assert_eq!(result.start, 7);
        assert_eq!(result.end, 43);
        assert_eq!(result.tag_content, "condition");
        assert_eq!(result.inner_content, "content");
    }

    #[test]
    fn test_detect_liquid_tag_start() {
        let mut chars = "{%".chars().peekable();
        assert!(detect_liquid_tag_start(&mut chars));

        let mut chars = "no".chars().peekable();
        assert!(!detect_liquid_tag_start(&mut chars));
    }

    #[test]
    fn test_detect_liquid_tag_start_negative_when_brace_not_percent() {
        let mut chars = "{x".chars().peekable();
        assert!(!detect_liquid_tag_start(&mut chars));
        // iterator should remain at start since nothing consumed
        assert_eq!(chars.next(), Some('{'));
    }

    #[test]
    fn test_parse_assignment() {
        let result = parse_assignment("variable = value").unwrap();
        assert_eq!(result, ("variable".to_string(), "value".to_string()));

        assert!(parse_assignment("invalid").is_none());
    }

    #[test]
    fn test_parse_assignment_with_extra_equals_returns_none() {
        assert!(parse_assignment("a=b=c").is_none());
    }

    #[test]
    fn test_parse_for_loop_parts() {
        let result = parse_for_loop_parts("item in collection").unwrap();
        assert_eq!(result, ("item".to_string(), "collection".to_string()));

        assert!(parse_for_loop_parts("invalid").is_none());
    }

    #[test]
    fn test_is_tag_type() {
        assert!(is_tag_type("for item in items", "for"));
        assert!(is_tag_type("  unless condition  ", "unless"));
        assert!(!is_tag_type("assign var = val", "for"));
    }

    #[test]
    fn test_extract_tag_parameter() {
        let result = extract_tag_parameter("if condition", "if").unwrap();
        assert_eq!(result, "condition");

        let result = extract_tag_parameter("for item in items", "for").unwrap();
        assert_eq!(result, "item in items");

        assert!(extract_tag_parameter("if", "if").is_none());
        assert!(extract_tag_parameter("assign var = val", "if").is_none());
    }

    #[test]
    fn test_extract_tag_inner() {
        assert_eq!(
            extract_tag_inner("{% include header %}", "include"),
            Some("header")
        );
        assert_eq!(extract_tag_inner("{% include header", "include"), None);
        assert_eq!(extract_tag_inner("not a tag", "include"), None);
    }

    #[test]
    fn test_read_nested_block_for_endfor() {
        let mut chars = " inner {% for x in y %} nested {% endfor %} tail {% endfor %} after"
            .chars()
            .peekable();
        // simulate that we've already consumed the outer start tag, so depth starts at 1
        let content = read_nested_block(&mut chars, "for ", "endfor").unwrap();
        assert_eq!(content, " inner {% for x in y %} nested {% endfor %} tail ");
        let remaining: String = chars.collect();
        assert_eq!(remaining, " after");
    }
}
