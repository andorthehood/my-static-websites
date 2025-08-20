use crate::converters::typescript::utils::{is_identifier_char, push_char_from};

#[allow(clippy::many_single_char_names)]
#[allow(clippy::too_many_lines)]
pub fn remove_type_annotations(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();

    // Track strings and comments to avoid modifying inside them
    let mut in_single = false;
    let mut in_double = false;
    let mut in_backtick = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while i < len {
        let c = b[i] as char;

        // Handle exiting comments
        if in_line_comment {
            push_char_from(input, &mut i, &mut out);
            if c == '\n' {
                in_line_comment = false;
            }
            continue;
        }
        if in_block_comment {
            push_char_from(input, &mut i, &mut out);
            if c == '*' && i < len && b[i] as char == '/' {
                out.push('/');
                i += 1;
                in_block_comment = false;
            }
            continue;
        }

        // Handle entering comments when not in strings
        if !in_single && !in_double && !in_backtick
            && c == '/' && i + 1 < len {
                let n = b[i + 1] as char;
                if n == '/' {
                    in_line_comment = true;
                    out.push(c);
                    out.push(n);
                    i += 2;
                    continue;
                }
                if n == '*' {
                    in_block_comment = true;
                    out.push(c);
                    out.push(n);
                    i += 2;
                    continue;
                }
            }

        // Handle string state toggles
        if !in_double && !in_backtick && c == '\'' {
            in_single = !in_single;
            push_char_from(input, &mut i, &mut out);
            continue;
        }
        if !in_single && !in_backtick && c == '"' {
            in_double = !in_double;
            push_char_from(input, &mut i, &mut out);
            continue;
        }
        if !in_single && !in_double && c == '`' {
            in_backtick = !in_backtick;
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        // If inside any string, just copy
        if in_single || in_double || in_backtick {
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        if c == ':' {
            // Look behind for something that looks like an identifier or ')' (return type)
            let mut j = i;
            while j > 0 && (b[j - 1] as char).is_ascii_whitespace() {
                j -= 1;
            }
            let prev_char = if j > 0 { b[j - 1] as char } else { '\0' };

            // Detect if this colon is part of an object literal property like `{ key: value }`.
            // Walk back over the property name to find the token immediately before it, skipping comment-only lines.
            let mut name_start = j;
            while name_start > 0 {
                let ch = b[name_start - 1] as char;
                if is_identifier_char(ch) || ch.is_ascii_digit() {
                    name_start -= 1;
                } else {
                    break;
                }
            }

            // Move back over whitespace, skipping preceding line if it contains a line comment (// ...\n)
            let mut k = name_start;
            while k > 0 && (b[k - 1] as char).is_ascii_whitespace() {
                if b[k - 1] as char == '\n' {
                    let mut line_start = k - 1;
                    while line_start > 0 && b[line_start - 1] as char != '\n' {
                        line_start -= 1;
                    }
                    let line_slice = &input[line_start..k - 1];
                    if line_slice.contains("//") {
                        k = line_start;
                        continue;
                    }
                }
                k -= 1;
            }

            let token_before_name = if k > 0 { b[k - 1] as char } else { '\0' };

            // If the token before the property name is '{' or ',', it's an object literal key.
            if token_before_name == '{' || token_before_name == ',' {
                out.push(':');
                i += 1;
                continue;
            }

            let looks_like_type_context = is_identifier_char(prev_char) || prev_char == ')';
            if looks_like_type_context {
                // Skip the type until a stopping delimiter at top-level depth
                i += 1; // skip ':'
                while i < len && (b[i] as char).is_ascii_whitespace() {
                    i += 1;
                }

                let mut k = i;
                let mut angle_depth = 0;
                let mut paren_depth = 0;
                let mut bracket_depth = 0;
                let mut brace_depth = 0;

                while k < len {
                    let ch = b[k] as char;
                    match ch {
                        '<' => angle_depth += 1,
                        '>' => {
                            if angle_depth > 0 {
                                angle_depth -= 1;
                            }
                        }
                        '(' => paren_depth += 1,
                        ')' => {
                            if paren_depth > 0 {
                                paren_depth -= 1;
                            } else {
                                break;
                            }
                        }
                        '[' => bracket_depth += 1,
                        ']' => {
                            if bracket_depth > 0 {
                                bracket_depth -= 1;
                            }
                        }
                        '{' => brace_depth += 1,
                        '}' => {
                            if brace_depth > 0 {
                                brace_depth -= 1;
                            } else {
                                break;
                            }
                        }
                        '=' | ',' | ';' | '\n' => {
                            if angle_depth == 0
                                && paren_depth == 0
                                && bracket_depth == 0
                                && brace_depth == 0
                            {
                                break;
                            }
                        }
                        _ => {}
                    }
                    k += 1;
                }

                // Advance i to the delimiter but do not consume it
                // Insert a single space if the next delimiter is '=' to preserve 'name ='
                let next_delim = if k < len { b[k] as char } else { '\0' };
                i = k;
                if next_delim == '=' {
                    if let Some(last_out) = out.chars().last() {
                        if !last_out.is_ascii_whitespace() {
                            out.push(' ');
                        }
                    } else {
                        out.push(' ');
                    }
                }
                continue;
            }
        }
        push_char_from(input, &mut i, &mut out);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::remove_type_annotations;

    #[test]
    fn preserves_object_literal_properties() {
        let ts = r#"
function f(){
	return {
		startX: x,
		startY: y,
		startDx: vx,
		startDy: vy
	};
}
		"#;
        let js = remove_type_annotations(ts);
        assert!(js.contains("startX: x"));
        assert!(js.contains("startY: y"));
        assert!(js.contains("startDx: vx"));
        assert!(js.contains("startDy: vy"));
    }

    #[test]
    fn preserves_object_literal_entries_in_conversion() {
        let ts = r#"
(function(){
	function g(){
		return { startX: x, startY: y, startDx: vx, startDy: vy };
	}
})();
		"#;
        let js = remove_type_annotations(ts);
        assert!(js.contains("startX: x"));
        assert!(js.contains("startY: y"));
        assert!(js.contains("startDx: vx"));
        assert!(js.contains("startDy: vy"));
    }
}
