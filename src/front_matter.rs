use std::collections::HashMap;

/// Removes surrounding single or double quotes from a string.
fn trim_quotes(s: &str) -> String {
    if (s.starts_with('\'') && s.ends_with('\'') || s.starts_with('"') && s.ends_with('"'))
        && s.len() > 1
    {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Parses the front matter of a document into a HashMap.
pub fn parse_front_matter(front_matter: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in front_matter.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = trim_quotes(parts[1].trim());
            map.insert(key, value);
        }
    }

    map
}

/// Extracts the YAML front matter from a Markdown string.
pub fn extract_front_matter(markdown: &str) -> Option<&str> {
    if let Some(start) = markdown.find("---") {
        if let Some(end) = markdown[start + 3..].find("---") {
            return Some(&markdown[start + 3..start + 3 + end].trim());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_front_matter_success() {
        let front_matter =
            "title: Example\nauthor: Andor Polgar\nlocation: 'Zandvoort, Netherlands'";
        let parsed = parse_front_matter(front_matter);

        assert_eq!(parsed.get("title"), Some(&"Example".to_string()));
        assert_eq!(parsed.get("author"), Some(&"Andor Polgar".to_string()));
        assert_eq!(parsed.get("camera"), None);
        assert_eq!(
            parsed.get("location"),
            Some(&"Zandvoort, Netherlands".to_string())
        );
    }

    #[test]
    fn test_extract_front_matter() {
        let markdown_with_front_matter = r#"
---
title: 'My Blog Post'
author: "John Doe"
date: 2024-03-04
---
This is the content of the blog post.
"#;

        let expected_front_matter = "title: 'My Blog Post'\nauthor: \"John Doe\"\ndate: 2024-03-04";
        assert_eq!(
            extract_front_matter(markdown_with_front_matter),
            Some(expected_front_matter)
        );

        let markdown_without_front_matter = "This is a regular markdown without front matter.";
        assert_eq!(extract_front_matter(markdown_without_front_matter), None);
    }
}
