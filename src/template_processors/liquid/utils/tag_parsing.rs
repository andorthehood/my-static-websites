use crate::error::{Error, Result};
use std::iter::Peekable;
use std::str::Chars;

/// Represents a parsed liquid tag with its content and position
#[derive(Debug, PartialEq)]
pub struct LiquidTag {
    pub start: usize,
    pub end: usize,
    pub tag_type: String,
    pub content: String,
}

/// Finds the next liquid tag starting from the given position
/// Returns the tag if found, otherwise None
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

/// Skips whitespace characters in a character iterator
pub fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }
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
}
