use super::depth_counters::DepthCounters;

/// Skips a type annotation until a stopping delimiter
pub fn skip_type_annotation(b: &[u8], len: usize, i: &mut usize, out: &mut String) {
    *i += 1; // skip ':'
    while *i < len && (b[*i] as char).is_ascii_whitespace() {
        *i += 1;
    }

    let type_start = *i;
    let mut k = *i;
    let mut counters = DepthCounters::new();

    while k < len {
        let ch = b[k] as char;

        // In function signatures, `{` can start the function body after the return type.
        // Do not treat a leading `{` as delimiter so object-literal return types still work.
        if ch == '{' && counters.all_zero() && k > type_start {
            break;
        }

        if counters.update(ch) {
            break; // Hit unmatched delimiter
        }

        if matches!(ch, '=' | ',' | ';' | '\n') && counters.all_zero() {
            break;
        }

        k += 1;
    }

    // Advance i to the delimiter but do not consume it
    // Insert a single space if the next delimiter is '=' to preserve 'name ='
    let next_delim = if k < len { b[k] as char } else { '\0' };
    *i = k;
    if next_delim == '=' {
        if let Some(last_out) = out.chars().last() {
            if !last_out.is_ascii_whitespace() {
                out.push(' ');
            }
        } else {
            out.push(' ');
        }
    }
}

/// Removes an optional marker (`?`) that was already written to output.
pub fn remove_optional_marker_from_output(out: &mut String) {
    let mut trailing_ws = String::new();
    while matches!(out.chars().last(), Some(ch) if ch.is_ascii_whitespace()) {
        if let Some(ch) = out.pop() {
            trailing_ws.push(ch);
        }
    }

    if out.ends_with('?') {
        out.pop();
    }

    // Preserve original spacing after the optional marker.
    for ch in trailing_ws.chars().rev() {
        out.push(ch);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_type_annotation_stops_at_comma() {
        let input = b": string, next";
        let len = input.len();
        let mut i = 0;
        let mut out = String::new();

        skip_type_annotation(input, len, &mut i, &mut out);

        assert_eq!(i, 8); // Stopped at comma
        assert_eq!(input[i] as char, ',');
    }

    #[test]
    fn skip_type_annotation_stops_at_equals() {
        let input = b": string = 'default'";
        let len = input.len();
        let mut i = 0;
        let mut out = String::new();

        skip_type_annotation(input, len, &mut i, &mut out);

        assert_eq!(i, 9); // Stopped at '='
        assert_eq!(input[i] as char, '=');
        assert_eq!(out, " "); // Added space before '='
    }

    #[test]
    fn skip_type_annotation_stops_at_semicolon() {
        let input = b": string;";
        let len = input.len();
        let mut i = 0;
        let mut out = String::new();

        skip_type_annotation(input, len, &mut i, &mut out);

        assert_eq!(i, 8); // Stopped at ';'
        assert_eq!(input[i] as char, ';');
    }

    #[test]
    fn skip_type_annotation_stops_at_newline() {
        let input = b": string\n";
        let len = input.len();
        let mut i = 0;
        let mut out = String::new();

        skip_type_annotation(input, len, &mut i, &mut out);

        assert_eq!(i, 8); // Stopped at newline
        assert_eq!(input[i] as char, '\n');
    }

    #[test]
    fn skip_type_annotation_handles_generic_types() {
        let input = b": Map<string, number>, next";
        let len = input.len();
        let mut i = 0;
        let mut out = String::new();

        skip_type_annotation(input, len, &mut i, &mut out);

        assert_eq!(i, 21); // Stopped at comma after closing '>'
        assert_eq!(input[i] as char, ',');
    }

    #[test]
    fn skip_type_annotation_handles_function_body_brace() {
        let input = b": void { return; }";
        let len = input.len();
        let mut i = 0;
        let mut out = String::new();

        skip_type_annotation(input, len, &mut i, &mut out);

        assert_eq!(i, 7); // Stopped before '{'
        assert_eq!(input[i] as char, '{');
    }

    #[test]
    fn skip_type_annotation_allows_object_literal_return_type() {
        let input = b": { x: number }, next";
        let len = input.len();
        let mut i = 0;
        let mut out = String::new();

        skip_type_annotation(input, len, &mut i, &mut out);

        assert_eq!(i, 15); // Stopped at comma after '}'
        assert_eq!(input[i] as char, ',');
    }

    #[test]
    fn remove_optional_marker_from_output_removes_question_mark() {
        let mut out = String::from("name?");
        remove_optional_marker_from_output(&mut out);
        assert_eq!(out, "name");
    }

    #[test]
    fn remove_optional_marker_preserves_trailing_whitespace() {
        let mut out = String::from("name?  ");
        remove_optional_marker_from_output(&mut out);
        assert_eq!(out, "name  ");
    }

    #[test]
    fn remove_optional_marker_does_nothing_without_question_mark() {
        let mut out = String::from("name");
        remove_optional_marker_from_output(&mut out);
        assert_eq!(out, "name");
    }

    #[test]
    fn remove_optional_marker_handles_empty_string() {
        let mut out = String::new();
        remove_optional_marker_from_output(&mut out);
        assert_eq!(out, "");
    }
}
