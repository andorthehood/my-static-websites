mod is_identifier_char;
use crate::converters::typescript::is_identifier_char::is_identifier_byte;

/// Minimal TypeScript-to-JavaScript stripper tailored for constructs used in router.ts.
/// This does not fully parse TS; it heuristically removes:
/// - `interface ... { ... }` blocks
/// - Generic annotations after `querySelector`/`querySelectorAll`, e.g. `<HTMLElement>`
/// - Generic arguments after identifiers when directly followed by a call, e.g. `Promise<void>(...)`
/// - Parameter and return type annotations in functions and arrow functions
/// - Variable type annotations in `const`/`let`/`var` declarations
/// - `as Type` casts (e.g., `(style as HTMLLinkElement)` -> `(style)`)
/// - Postfix non-null assertions like `value!` or `call()`
///
/// It intentionally does NOT implement enums or other TS features.
pub fn strip_typescript_types(input: &str) -> String {
    let without_interfaces = remove_interface_blocks(input);
    let without_generics = remove_query_selector_generics(&without_interfaces);
    let without_call_generics = remove_generics_before_calls(&without_generics);
    let without_casts = remove_as_casts(&without_call_generics);
    let without_types = remove_type_annotations(&without_casts);
    let without_non_null = remove_postfix_non_null(&without_types);
    if input.contains("interface PageData") {
        eprintln!(
            "[DEBUG] strip_typescript_types output:\n{}",
            without_non_null
        );
    }
    without_non_null
}

fn is_identifier_char(c: char) -> bool {
    c.is_ascii() && is_identifier_byte(c as u8)
}

#[inline]
fn push_char_from(input: &str, index: &mut usize, out: &mut String) {
    if let Some(ch) = input.get(*index..).and_then(|s| s.chars().next()) {
        out.push(ch);
        *index += ch.len_utf8();
    } else {
        *index += 1; // should not happen, but avoid infinite loop
    }
}

fn remove_interface_blocks(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Track strings and comments to avoid false positives
    let mut in_single = false;
    let mut in_double = false;
    let mut in_backtick = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while i < len {
        let c = bytes[i] as char;

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
            if c == '*' && i < len && bytes[i] as char == '/' {
                out.push('/');
                i += 1;
                in_block_comment = false;
            }
            continue;
        }

        // Handle string states
        if !in_single && !in_double && !in_backtick {
            if c == '/' && i + 1 < len {
                let n = bytes[i + 1] as char;
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
        }
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

        if !in_single && !in_double && !in_backtick {
            // Check for "interface" keyword
            if c == 'i'
                && (input
                    .get(i..)
                    .map_or(false, |s| s.starts_with("interface "))
                    || input
                        .get(i..)
                        .map_or(false, |s| s.starts_with("interface\t"))
                    || input
                        .get(i..)
                        .map_or(false, |s| s.starts_with("interface\n")))
            {
                // Skip keyword
                i += "interface".len();
                // Skip whitespace and name
                while i < len && (bytes[i] as char).is_ascii_whitespace() {
                    i += 1;
                }
                while i < len && is_identifier_char(bytes[i] as char) {
                    i += 1;
                }
                while i < len && (bytes[i] as char).is_ascii_whitespace() {
                    i += 1;
                }
                // Expect block starting with '{'
                if i < len && bytes[i] as char == '{' {
                    // Skip balanced braces
                    let mut depth = 0;
                    while i < len {
                        let ch = bytes[i] as char;
                        if ch == '{' {
                            depth += 1;
                        }
                        if ch == '}' {
                            depth -= 1;
                            i += 1;
                            if depth == 0 {
                                break;
                            }
                            continue;
                        }
                        i += 1;
                    }
                    // Skip trailing whitespace
                    while i < len && (bytes[i] as char).is_ascii_whitespace() {
                        i += 1;
                    }
                    continue; // do not copy interface block
                }
            }
        }

        push_char_from(input, &mut i, &mut out);
    }

    out
}

fn remove_query_selector_generics(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();

    // Track strings and comments
    let mut in_single = false;
    let mut in_double = false;
    let mut in_backtick = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while i < len {
        let ch = b[i] as char;

        // Handle exiting comments
        if in_line_comment {
            push_char_from(input, &mut i, &mut out);
            if ch == '\n' {
                in_line_comment = false;
            }
            continue;
        }
        if in_block_comment {
            push_char_from(input, &mut i, &mut out);
            if ch == '*' && i < len && b[i] as char == '/' {
                out.push('/');
                i += 1;
                in_block_comment = false;
            }
            continue;
        }

        // Enter comments if not in string
        if !in_single && !in_double && !in_backtick {
            if ch == '/' && i + 1 < len {
                let n = b[i + 1] as char;
                if n == '/' {
                    in_line_comment = true;
                    out.push(ch);
                    out.push(n);
                    i += 2;
                    continue;
                }
                if n == '*' {
                    in_block_comment = true;
                    out.push(ch);
                    out.push(n);
                    i += 2;
                    continue;
                }
            }
        }

        // String toggles
        if !in_double && !in_backtick && ch == '\'' {
            in_single = !in_single;
            push_char_from(input, &mut i, &mut out);
            continue;
        }
        if !in_single && !in_backtick && ch == '"' {
            in_double = !in_double;
            push_char_from(input, &mut i, &mut out);
            continue;
        }
        if !in_single && !in_double && ch == '`' {
            in_backtick = !in_backtick;
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        // If inside any string, just copy
        if in_single || in_double || in_backtick {
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        if i + 12 <= len
            && input
                .get(i..)
                .map_or(false, |s| s.starts_with("querySelector"))
        {
            out.push_str("querySelector");
            i += "querySelector".len();
            // Optional "All"
            if i + 3 <= len && input.get(i..).map_or(false, |s| s.starts_with("All")) {
                out.push_str("All");
                i += 3;
            }
            // Skip spaces
            while i < len && (b[i] as char).is_ascii_whitespace() {
                i += 1;
            }
            // Remove generic if present
            if i < len && b[i] as char == '<' {
                let mut depth = 0;
                while i < len {
                    let ch2 = b[i] as char;
                    if ch2 == '<' {
                        depth += 1;
                    }
                    if ch2 == '>' {
                        depth -= 1;
                        i += 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    i += 1;
                }
                // Skip spaces
                while i < len && (b[i] as char).is_ascii_whitespace() {
                    i += 1;
                }
            }
            continue;
        }
        push_char_from(input, &mut i, &mut out);
    }
    out
}

fn remove_generics_before_calls(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();

    // Track strings and comments
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

        // Enter comments when not in strings
        if !in_single && !in_double && !in_backtick {
            if c == '/' && i + 1 < len {
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
        }

        // String toggles
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

        // If inside strings, just copy
        if in_single || in_double || in_backtick {
            push_char_from(input, &mut i, &mut out);
            continue;
        }

        // Detect start of identifier (and ensure previous is not identifier char)
        if (c.is_ascii_alphabetic() || c == '_' || c == '$')
            && (i == 0 || !is_identifier_char(b[i - 1] as char))
        {
            // Read identifier
            let start_ident = i;
            i += 1;
            while i < len && is_identifier_char(b[i] as char) {
                i += 1;
            }
            // Copy identifier to output
            if let Ok(ident_str) = std::str::from_utf8(&b[start_ident..i]) {
                out.push_str(ident_str);
            }

            // Skip whitespace
            let mut j = i;
            while j < len && (b[j] as char).is_ascii_whitespace() {
                j += 1;
            }
            // If next is '<', try to parse generic and remove it only if next non-space after generic is '('
            if j < len && b[j] as char == '<' {
                let mut k = j;
                let mut depth = 0;
                let mut valid = false;
                while k < len {
                    let ch2 = b[k] as char;
                    if ch2 == '<' {
                        depth += 1;
                    } else if ch2 == '>' {
                        depth -= 1;
                        if depth == 0 {
                            k += 1;
                            valid = true;
                            break;
                        }
                    }
                    k += 1;
                }
                if valid {
                    // Check next non-space
                    let mut m = k;
                    while m < len && (b[m] as char).is_ascii_whitespace() {
                        m += 1;
                    }
                    if m < len && b[m] as char == '(' {
                        // Drop the generic by advancing i to k (after '>')
                        i = k;
                        continue;
                    } else {
                        // Not a call context, keep original including whitespace
                        if let Ok(orig) = std::str::from_utf8(&b[i..k]) {
                            out.push_str(orig);
                        }
                        i = k;
                        continue;
                    }
                }
                // If not valid generic, fall through
            }
            continue;
        }
        push_char_from(input, &mut i, &mut out);
    }

    out
}

fn remove_as_casts(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Track strings and comments to avoid modifying inside them
    let mut in_single = false;
    let mut in_double = false;
    let mut in_backtick = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while i < len {
        let c = bytes[i] as char;

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
            if c == '*' && i < len && bytes[i] as char == '/' {
                out.push('/');
                i += 1;
                in_block_comment = false;
            }
            continue;
        }

        // Handle entering comments when not in strings
        if !in_single && !in_double && !in_backtick {
            if c == '/' && i + 1 < len {
                let n = bytes[i + 1] as char;
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

        if i + 2 < len
            && bytes[i].is_ascii_whitespace()
            && input.get(i + 1..).map_or(false, |s| s.starts_with("as "))
        {
            // Found " as ": remove until a terminator character
            i += 1 + 3; // skip space + "as "
            while i < len {
                let ch = bytes[i] as char;
                if ch == ')' || ch == ';' || ch == ',' || ch == '\n' || ch == '.' || ch == ']' {
                    break;
                }
                i += 1;
            }
            continue; // do not copy the removed type
        }
        // Handle "(ident as Type)" where there might not be leading space before 'as'
        if i + 4 < len && input.get(i..).map_or(false, |s| s.starts_with(" as ")) {
            i += 4;
            while i < len {
                let ch = bytes[i] as char;
                if ch == ')' || ch == ';' || ch == ',' || ch == '\n' || ch == '.' || ch == ']' {
                    break;
                }
                i += 1;
            }
            continue;
        }
        push_char_from(input, &mut i, &mut out);
    }

    out
}

fn remove_type_annotations(input: &str) -> String {
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
        if !in_single && !in_double && !in_backtick {
            if c == '/' && i + 1 < len {
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
            loop {
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
                break;
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
                        '=' => {
                            if angle_depth == 0
                                && paren_depth == 0
                                && bracket_depth == 0
                                && brace_depth == 0
                            {
                                break;
                            }
                        }
                        ',' | ';' | '\n' => {
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

fn remove_postfix_non_null(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();

    while i < len {
        let c = b[i] as char;
        if c == '!' {
            // Check previous non-space char
            let mut j = i;
            while j > 0 && (b[j - 1] as char).is_ascii_whitespace() {
                j -= 1;
            }
            let prev = if j > 0 { b[j - 1] as char } else { '\0' };
            // Check next non-space char
            let mut k = i + 1;
            while k < len && (b[k] as char).is_ascii_whitespace() {
                k += 1;
            }
            let next = if k < len { b[k] as char } else { '\0' };

            let prev_allows_postfix = prev == ')' || prev == ']' || is_identifier_char(prev);
            let next_is_terminator = next == '.'
                || next == ';'
                || next == ','
                || next == ')'
                || next == ']'
                || next == '\n';
            if prev_allows_postfix && next_is_terminator {
                // Drop this '!'
                i += 1;
                continue;
            }
        }
        push_char_from(input, &mut i, &mut out);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::strip_typescript_types;

    #[test]
    fn strips_router_like_types() {
        let ts = r#"
export {};

interface PageData {
	content: string;
	css?: string;
}

const pageSpecificStyleTags: HTMLLinkElement[] = [];
const pageCache: Map<string, PageData> = new Map();

function handleStyleTags(data: PageData): Promise<void> {
	pageSpecificStyleTags.forEach((style: HTMLLinkElement) => style.remove());
	return new Promise<void>((resolve) => {
		const head = document.querySelector('head') as HTMLHeadElement | null;
		const style = document.createElement('link');
		pageSpecificStyleTags.push(style as HTMLLinkElement);
		(style as HTMLLinkElement).onload = () => {};
	});
}

function replaceContent(data: PageData): void {
	const content = document.querySelector<HTMLElement>('.content');
}

function handleLinkClick(event: Event): void {
	const link = event.currentTarget as HTMLAnchorElement | null;
	fetch('/x').then((response: Response) => response.json()).then((data: PageData) => {});
}

(function(){
	const links = document.querySelectorAll<HTMLAnchorElement>('a');
	const data = pageCache.get('k')!;
})();
		"#;

        let js = strip_typescript_types(ts);

        assert!(!js.contains("interface PageData"));
        assert!(!js.contains(": HTMLLinkElement[]"));
        assert!(!js.contains(": Map<string, PageData>"));
        assert!(!js.contains(": PageData"));
        assert!(!js.contains("Promise<void>"));
        assert!(!js.contains("as HTMLHeadElement"));
        assert!(!js.contains("as HTMLLinkElement"));
        assert!(!js.contains("as EventListener"));
        assert!(!js.contains("<HTMLElement>"));
        assert!(!js.contains("<HTMLAnchorElement>"));
        assert!(!js.contains(")!"));

        // Spot-check a few expected transformations
        assert!(js.contains("const pageSpecificStyleTags = []"));
        // Avoid brittle formatting checks for browser APIs; converter is covered by copier tests as well
    }

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
        let js = strip_typescript_types(ts);
        assert!(js.contains("startX: x"));
        assert!(js.contains("startY: y"));
        assert!(js.contains("startDx: vx"));
        assert!(js.contains("startDy: vy"));
    }

    #[test]
    fn preserves_non_ascii_emoji_in_template() {
        let ts = "console.log(`🎉 CORNER HIT! Frame ${frameCount}`);";
        let js = strip_typescript_types(ts);
        assert!(js.contains("🎉 CORNER HIT!"));
    }

    #[test]
    fn preserves_url_in_string_literal() {
        let ts = r#"
(function(){
	setInterval(function(){
		const u='https://static.llllllllllll.com/andor/assets/clippy/swaying.gif?c=' + Date.now();
		console.log(u);
	},8000);
})();
		"#;
        let js = strip_typescript_types(ts);
        assert!(js.contains("https://static.llllllllllll.com/andor/assets/clippy/swaying.gif?c="));
    }

    #[test]
    fn does_not_strip_type_like_sequences_inside_strings_and_templates() {
        let ts = r#"
(function(){
	const a = "as HTMLLinkElement : number <T> ! interface X { a: string }";
	const b = 'querySelector<HTMLElement> as Type : string !';
	const c = `template keeps as Cast<T> : number and bang!`;
	console.log(a,b,c);
})();
		"#;
        let js = strip_typescript_types(ts);
        eprintln!("[DEBUG] JS output for type-like sequences test:\n{}", js);
        assert!(js.contains("as HTMLLinkElement : number <T> ! interface X { a: string }"));
        assert!(js.contains("querySelector<HTMLElement> as Type : string !"));
        assert!(js.contains("template keeps as Cast<T> : number and bang!"));
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
        let js = strip_typescript_types(ts);
        assert!(js.contains("startX: x"));
        assert!(js.contains("startY: y"));
        assert!(js.contains("startDx: vx"));
        assert!(js.contains("startDy: vy"));
    }
}
