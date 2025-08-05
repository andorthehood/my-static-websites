use std::iter::Peekable;
use std::str::Chars;

/// Skips to the end of an "unless" block by finding the matching "endunless" tag
pub fn skip_to_endunless(chars: &mut Peekable<Chars>) {
    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'%') {
            chars.next(); // consume '%'
            let mut tag_content = String::new();
            while let Some(tc) = chars.next() {
                if tc == '%' && chars.peek() == Some(&'}') {
                    chars.next(); // consume '}'
                    if tag_content.trim() == "endunless" {
                        return;
                    }
                    break;
                } else {
                    tag_content.push(tc);
                }
            }
        }
    }
}

/// Reads content until finding an "endunless" tag
pub fn read_until_endunless(chars: &mut Peekable<Chars>) -> String {
    let mut content = String::new();

    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'%') {
            // Potential tag
            let tag_start = content.len();
            content.push(c);
            content.push(chars.next().unwrap()); // push '%'

            let mut tag_content = String::new();
            while let Some(tc) = chars.next() {
                if tc == '%' && chars.peek() == Some(&'}') {
                    chars.next(); // consume '}'
                    if tag_content.trim() == "endunless" {
                        // Remove the tag we just added and return
                        content.truncate(tag_start);
                        return content;
                    } else {
                        content.push(tc);
                        content.push('}');
                    }
                    break;
                } else {
                    tag_content.push(tc);
                    content.push(tc);
                }
            }
        } else {
            content.push(c);
        }
    }

    content
}

/// Reads a liquid tag's content and returns it along with whether the closing tag was found
pub fn read_liquid_tag_content(chars: &mut Peekable<Chars>) -> (String, bool) {
    let mut tag_content = String::new();
    let mut found_closing = false;

    // Collect tag content until we find %}
    while let Some(c) = chars.next() {
        if c == '%' && chars.peek() == Some(&'}') {
            chars.next(); // Skip '}'
            found_closing = true;
            break;
        }
        tag_content.push(c);
    }

    (tag_content, found_closing)
}

/// Advances the character iterator to skip whitespace characters
pub fn advance_past_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_to_endunless() {
        let content = "{% endunless %} remaining";
        let mut chars = content.chars().peekable();

        skip_to_endunless(&mut chars);

        let remaining: String = chars.collect();
        assert_eq!(remaining, " remaining");
    }

    #[test]
    fn test_read_until_endunless() {
        let content = "content inside {% endunless %} after";
        let mut chars = content.chars().peekable();

        let result = read_until_endunless(&mut chars);
        assert_eq!(result, "content inside ");

        let remaining: String = chars.collect();
        assert_eq!(remaining, " after");
    }

    #[test]
    fn test_read_liquid_tag_content() {
        let mut chars = " if condition %}".chars().peekable();
        let (content, found_closing) = read_liquid_tag_content(&mut chars);

        assert_eq!(content, " if condition ");
        assert!(found_closing);
    }

    #[test]
    fn test_read_liquid_tag_content_unclosed() {
        let mut chars = " if condition".chars().peekable();
        let (content, found_closing) = read_liquid_tag_content(&mut chars);

        assert_eq!(content, " if condition");
        assert!(!found_closing);
    }

    #[test]
    fn test_advance_past_whitespace() {
        let mut chars = "   hello".chars().peekable();
        advance_past_whitespace(&mut chars);
        assert_eq!(chars.next(), Some('h'));
    }
}
