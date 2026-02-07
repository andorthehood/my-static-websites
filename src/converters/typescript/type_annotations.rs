use crate::converters::typescript::utils::{is_identifier_char, push_char_from};

/// Represents the state of string and comment parsing
#[allow(clippy::struct_excessive_bools)]
struct ParseState {
    in_single: bool,
    in_double: bool,
    in_backtick: bool,
    in_line_comment: bool,
    in_block_comment: bool,
}

impl ParseState {
    fn new() -> Self {
        Self {
            in_single: false,
            in_double: false,
            in_backtick: false,
            in_line_comment: false,
            in_block_comment: false,
        }
    }

    fn is_in_string(&self) -> bool {
        self.in_single || self.in_double || self.in_backtick
    }
}

/// Represents depth counters for balanced delimiter tracking
#[allow(clippy::struct_field_names)]
struct DepthCounters {
    angle_depth: i32,
    paren_depth: i32,
    bracket_depth: i32,
    brace_depth: i32,
}

impl DepthCounters {
    fn new() -> Self {
        Self {
            angle_depth: 0,
            paren_depth: 0,
            bracket_depth: 0,
            brace_depth: 0,
        }
    }

    fn all_zero(&self) -> bool {
        self.angle_depth == 0
            && self.paren_depth == 0
            && self.bracket_depth == 0
            && self.brace_depth == 0
    }

    fn update(&mut self, ch: char) -> bool {
        match ch {
            '<' => self.angle_depth += 1,
            '>' => {
                if self.angle_depth > 0 {
                    self.angle_depth -= 1;
                }
            }
            '(' => self.paren_depth += 1,
            ')' => {
                if self.paren_depth > 0 {
                    self.paren_depth -= 1;
                } else {
                    return true; // Signal to break
                }
            }
            '[' => self.bracket_depth += 1,
            ']' => {
                if self.bracket_depth > 0 {
                    self.bracket_depth -= 1;
                }
            }
            '{' => self.brace_depth += 1,
            '}' => {
                if self.brace_depth > 0 {
                    self.brace_depth -= 1;
                } else {
                    return true; // Signal to break
                }
            }
            _ => {}
        }
        false
    }
}

/// Handles comment parsing and state updates
fn handle_comments(
    input: &str,
    b: &[u8],
    len: usize,
    i: &mut usize,
    c: char,
    state: &mut ParseState,
    out: &mut String,
) -> bool {
    // Handle exiting comments
    if state.in_line_comment {
        push_char_from(input, i, out);
        if c == '\n' {
            state.in_line_comment = false;
        }
        return true;
    }
    if state.in_block_comment {
        push_char_from(input, i, out);
        if c == '*' && *i < len && b[*i] as char == '/' {
            out.push('/');
            *i += 1;
            state.in_block_comment = false;
        }
        return true;
    }

    // Handle entering comments when not in strings
    if !state.is_in_string() && c == '/' && *i + 1 < len {
        let n = b[*i + 1] as char;
        if n == '/' {
            state.in_line_comment = true;
            out.push(c);
            out.push(n);
            *i += 2;
            return true;
        }
        if n == '*' {
            state.in_block_comment = true;
            out.push(c);
            out.push(n);
            *i += 2;
            return true;
        }
    }
    false
}

/// Handles string literal parsing and state updates
fn handle_strings(
    input: &str,
    i: &mut usize,
    c: char,
    state: &mut ParseState,
    out: &mut String,
) -> bool {
    if !state.in_double && !state.in_backtick && c == '\'' {
        state.in_single = !state.in_single;
        push_char_from(input, i, out);
        return true;
    }
    if !state.in_single && !state.in_backtick && c == '"' {
        state.in_double = !state.in_double;
        push_char_from(input, i, out);
        return true;
    }
    if !state.in_single && !state.in_double && c == '`' {
        state.in_backtick = !state.in_backtick;
        push_char_from(input, i, out);
        return true;
    }
    false
}

/// Finds the start of the property name by walking backwards
fn find_property_name_start(b: &[u8], mut j: usize) -> usize {
    while j > 0 {
        let ch = b[j - 1] as char;
        if is_identifier_char(ch) || ch.is_ascii_digit() {
            j -= 1;
        } else {
            break;
        }
    }
    j
}

/// Skips whitespace backwards, handling comment lines
fn skip_whitespace_and_comments(input: &str, b: &[u8], mut k: usize) -> usize {
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
    k
}

/// Checks if this colon is part of an object literal property
fn is_object_literal_property(input: &str, b: &[u8], i: usize) -> bool {
    let mut j = i;
    while j > 0 && (b[j - 1] as char).is_ascii_whitespace() {
        j -= 1;
    }

    let name_start = find_property_name_start(b, j);
    let k = skip_whitespace_and_comments(input, b, name_start);
    let token_before_name = if k > 0 { b[k - 1] as char } else { '\0' };

    token_before_name == '{' || token_before_name == ','
}

/// Skips a type annotation until a stopping delimiter
fn skip_type_annotation(b: &[u8], len: usize, i: &mut usize, out: &mut String) {
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
fn remove_optional_marker_from_output(out: &mut String) {
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

/// Processes colon characters and handles type annotation removal
fn handle_colon(input: &str, b: &[u8], len: usize, i: &mut usize, out: &mut String) -> bool {
    // If this is an object literal property, keep the colon
    if is_object_literal_property(input, b, *i) {
        out.push(':');
        *i += 1;
        return true;
    }

    // Look behind for something that looks like an identifier or ')' (return type)
    let mut j = *i;
    while j > 0 && (b[j - 1] as char).is_ascii_whitespace() {
        j -= 1;
    }
    let prev_char = if j > 0 { b[j - 1] as char } else { '\0' };

    let mut looks_like_type_context = is_identifier_char(prev_char) || prev_char == ')';

    // Support optional parameters/properties like `name?: string`.
    if !looks_like_type_context && prev_char == '?' {
        let mut k = j.saturating_sub(1); // points to the '?'
        while k > 0 && (b[k - 1] as char).is_ascii_whitespace() {
            k -= 1;
        }
        let before_optional = if k > 0 { b[k - 1] as char } else { '\0' };
        if is_identifier_char(before_optional) || before_optional == ')' {
            remove_optional_marker_from_output(out);
            looks_like_type_context = true;
        }
    }

    if looks_like_type_context {
        skip_type_annotation(b, len, i, out);
        return true;
    }

    false
}

pub fn remove_type_annotations(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();
    let mut state = ParseState::new();

    while i < len {
        let c = b[i] as char;

        // Handle comments first
        if handle_comments(input, b, len, &mut i, c, &mut state, &mut out) {
            continue;
        }

        // Handle strings
        if handle_strings(input, &mut i, c, &mut state, &mut out) {
            continue;
        }

        // If inside any string, just copy
        if state.is_in_string() {
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        // Handle type annotations
        if c == ':' && handle_colon(input, b, len, &mut i, &mut out) {
            continue;
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

    #[test]
    fn strips_optional_parameter_type_annotation() {
        let ts = r#"
function navigateToJson(json, fetchUrl?: string) {
    return [json, fetchUrl];
}
        "#;
        let js = remove_type_annotations(ts);
        assert!(js.contains("function navigateToJson(json, fetchUrl)"));
        assert!(!js.contains("?: string"));
        assert!(!js.contains("fetchUrl?"));
    }

    #[test]
    fn strips_return_type_without_removing_function_body() {
        let ts = r#"
function handleStyleTags(data): Promise<void> {
    const pageSpecificStyleTags = document.querySelectorAll('link.page-specific-css');
    return new Promise((resolve) => resolve());
}
        "#;
        let js = remove_type_annotations(ts);
        assert!(js.contains("function handleStyleTags(data)"));
        assert!(js.contains(
            "const pageSpecificStyleTags = document.querySelectorAll('link.page-specific-css');"
        ));
        assert!(js.contains("return new Promise((resolve) => resolve());"));
        assert!(!js.contains(": Promise<void>"));
    }
}
