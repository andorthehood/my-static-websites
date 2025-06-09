use std::collections::HashMap;

use crate::parsers::{extract_front_matter, parse_front_matter};

/// Extracts the content of a Markdown string, excluding the front matter.
pub fn extract_content(markdown: &str) -> Option<&str> {
    let mut end_of_front_matter = 0;

    if let Some(start) = markdown.find("---") {
        if let Some(end) = markdown[start + 3..].find("---") {
            end_of_front_matter = start + 3 + end + 3; // Skip past the closing '---'
        } else {
            return None; // No closing '---', so not valid front matter
        }
    }

    if end_of_front_matter < markdown.len() {
        Some(&markdown[end_of_front_matter..].trim_start())
    } else {
        None // No content after front matter
    }
}

pub fn parse_markdown_with_front_matter(markdown: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    // Extract and parse the front matter
    if let Some(front_matter_str) = extract_front_matter(markdown) {
        let front_matter = parse_front_matter(front_matter_str);
        for (key, value) in front_matter {
            result.insert(key, value);
        }
    }

    // Extract and add the content
    if let Some(content) = extract_content(markdown) {
        result.insert("content".to_string(), content.to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_content() {
        let markdown_with_front_matter = r#"
---
title: 'My Blog Post'
author: "John Doe"
date: 2024-03-04
---
This is the content of the blog post.
"#;
        let expected_content = "This is the content of the blog post.\n";
        assert_eq!(
            extract_content(markdown_with_front_matter),
            Some(expected_content)
        );

        let markdown_with_incomplete_front_matter = r#"
---
title: 'Incomplete Front Matter
This is the content with incomplete front matter.
"#;
        assert_eq!(extract_content(markdown_with_incomplete_front_matter), None);
    }

    #[test]
    fn test_parse_markdown_with_front_matter() {
        let markdown = r#"
 --- 
 title: Test Post
 author: Jane Doe
 ---

 This is the content of the post.
 Lorem ipsum dolor sit amet."#;

        let parsed = parse_markdown_with_front_matter(markdown);

        assert_eq!(parsed.get("title"), Some(&"Test Post".to_string()));
        assert_eq!(parsed.get("author"), Some(&"Jane Doe".to_string()));
        assert_eq!(
            parsed.get("content"),
            Some(&"This is the content of the post.\n Lorem ipsum dolor sit amet.".to_string())
        );
    }
}
