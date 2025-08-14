use crate::converters::typescript::is_identifier_char::is_identifier_byte;

#[inline]
pub fn is_identifier_char(c: char) -> bool {
    c.is_ascii() && is_identifier_byte(c as u8)
}

#[inline]
pub fn push_char_from(input: &str, index: &mut usize, out: &mut String) {
    if let Some(ch) = input.get(*index..).and_then(|s| s.chars().next()) {
        out.push(ch);
        *index += ch.len_utf8();
    } else {
        *index += 1; // should not happen, but avoid infinite loop
    }
}

#[cfg(test)]
mod tests {
    use super::{is_identifier_char, push_char_from};

    #[test]
    fn is_identifier_char_basic() {
        assert!(is_identifier_char('a'));
        assert!(is_identifier_char('_'));
        assert!(is_identifier_char('$'));
        assert!(!is_identifier_char('-'));
    }

    #[test]
    fn push_char_from_handles_utf8() {
        let s = "🎉 ok";
        let mut i = 0;
        let mut out = String::new();
        push_char_from(s, &mut i, &mut out);
        assert_eq!(out, "🎉");
        assert_eq!(i, "🎉".len());
    }
}
