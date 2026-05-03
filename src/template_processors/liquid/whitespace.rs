/// Applies Liquid-style whitespace trimming around tags and variables.
///
/// Supports `{%-`, `{{-`, `-%}`, and `-}}` by trimming whitespace before or
/// after the tag/variable while normalizing the delimiters back to `{%`, `{{`,
/// `%}`, and `}}` for the existing processors.
pub fn process_liquid_whitespace_trim(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    let mut trim_next = false;

    while i < input.len() {
        if trim_next {
            while i < input.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            trim_next = false;
        }

        if input[i..].starts_with("{%-") {
            if let Some((end, trim_right)) = read_trimmed_liquid_delimiter(input, i + 3, "%}") {
                trim_ascii_whitespace_end(&mut result);
                result.push_str("{%");
                result.push_str(&input[i + 3..end]);
                result.push_str("%}");
                trim_next = trim_right;
                i = end + if trim_right { 3 } else { 2 };
                continue;
            }
        }

        if input[i..].starts_with("{{-") {
            if let Some((end, trim_right)) = read_trimmed_liquid_delimiter(input, i + 3, "}}") {
                trim_ascii_whitespace_end(&mut result);
                result.push_str("{{");
                result.push_str(&input[i + 3..end]);
                result.push_str("}}");
                trim_next = trim_right;
                i = end + if trim_right { 3 } else { 2 };
                continue;
            }
        }

        if input[i..].starts_with("{%") {
            if let Some((end, trim_right)) = read_trimmed_liquid_delimiter(input, i + 2, "%}") {
                result.push_str("{%");
                result.push_str(&input[i + 2..end]);
                result.push_str("%}");
                trim_next = trim_right;
                i = end + if trim_right { 3 } else { 2 };
                continue;
            }
        }

        if input[i..].starts_with("{{") {
            if let Some((end, trim_right)) = read_trimmed_liquid_delimiter(input, i + 2, "}}") {
                result.push_str("{{");
                result.push_str(&input[i + 2..end]);
                result.push_str("}}");
                trim_next = trim_right;
                i = end + if trim_right { 3 } else { 2 };
                continue;
            }
        }

        let ch = input[i..].chars().next().unwrap();
        result.push(ch);
        i += ch.len_utf8();
    }

    result
}

fn read_trimmed_liquid_delimiter(
    input: &str,
    content_start: usize,
    close: &str,
) -> Option<(usize, bool)> {
    let close_start = input[content_start..].find(close)? + content_start;
    let has_right_trim = close_start > content_start && input[..close_start].ends_with('-');
    let content_end = if has_right_trim {
        close_start - 1
    } else {
        close_start
    };

    Some((content_end, has_right_trim))
}

fn trim_ascii_whitespace_end(value: &mut String) {
    while value
        .as_bytes()
        .last()
        .is_some_and(|byte| byte.is_ascii_whitespace())
    {
        value.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_left_on_tag() {
        let input = "A \n {%- if show %}B{% endif %}";
        let result = process_liquid_whitespace_trim(input);
        assert_eq!(result, "A{% if show %}B{% endif %}");
    }

    #[test]
    fn test_trim_right_on_tag() {
        let input = "{% if show -%}\n  B{% endif %}";
        let result = process_liquid_whitespace_trim(input);
        assert_eq!(result, "{% if show %}B{% endif %}");
    }

    #[test]
    fn test_trim_right_on_variable() {
        let input = "{{ name -}}\n  B";
        let result = process_liquid_whitespace_trim(input);
        assert_eq!(result, "{{ name }}B");
    }

    #[test]
    fn test_trim_preserves_unclosed_tags() {
        let input = "A {%- if show";
        let result = process_liquid_whitespace_trim(input);
        assert_eq!(result, "A {%- if show");
    }
}
