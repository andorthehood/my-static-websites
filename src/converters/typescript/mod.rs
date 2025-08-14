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
/// - Postfix non-null assertions like `value!` or `call()!`
///
/// It intentionally does NOT implement enums or other TS features.
pub fn strip_typescript_types(input: &str) -> String {
    let without_interfaces = remove_interface_blocks(input);
    let without_generics = remove_query_selector_generics(&without_interfaces);
    let without_call_generics = remove_generics_before_calls(&without_generics);
    let without_casts = remove_as_casts(&without_call_generics);
    let without_types = remove_type_annotations(&without_casts);
    let without_non_null = remove_postfix_non_null(&without_types);
    without_non_null
}

fn is_identifier_char(c: char) -> bool {
    c.is_ascii() && is_identifier_byte(c as u8)
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
            out.push(c);
            i += 1;
            if c == '\n' {
                in_line_comment = false;
            }
            continue;
        }
        if in_block_comment {
            out.push(c);
            i += 1;
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
            out.push(c);
            i += 1;
            continue;
        }
        if !in_single && !in_backtick && c == '"' {
            in_double = !in_double;
            out.push(c);
            i += 1;
            continue;
        }
        if !in_single && !in_double && c == '`' {
            in_backtick = !in_backtick;
            out.push(c);
            i += 1;
            continue;
        }

        if !in_single && !in_double && !in_backtick {
            // Check for "interface" keyword
            if c == 'i' && input[i..].starts_with("interface ")
                || input[i..].starts_with("interface\t")
                || input[i..].starts_with("interface\n")
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

        out.push(c);
        i += 1;
    }

    out
}

fn remove_query_selector_generics(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();
    while i < len {
        if i + 12 <= len && input[i..].starts_with("querySelector") {
            out.push_str("querySelector");
            i += "querySelector".len();
            // Optional "All"
            if i + 3 <= len && input[i..].starts_with("All") {
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
                    let ch = b[i] as char;
                    if ch == '<' {
                        depth += 1;
                    }
                    if ch == '>' {
                        depth -= 1;
                        i += 1;
                        if depth == 0 {
                            break;
                        }
                        i += 0;
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
        out.push(b[i] as char);
        i += 1;
    }
    out
}

fn remove_generics_before_calls(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();

    while i < len {
        let c = b[i] as char;
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
            out.push_str(&input[start_ident..i]);

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
                    let ch = b[k] as char;
                    if ch == '<' {
                        depth += 1;
                    } else if ch == '>' {
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
                        // Not a call context, keep original generic
                        out.push_str(&input[j..k]);
                        i = k;
                        continue;
                    }
                }
                // If not valid generic, fall through (will copy next char in outer loop)
            }
            continue;
        }
        out.push(c);
        i += 1;
    }

    out
}

fn remove_as_casts(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    let len = bytes.len();

    while i < len {
        if i + 2 < len && bytes[i].is_ascii_whitespace() && input[i + 1..].starts_with("as ") {
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
        if i + 4 < len && input[i..].starts_with(" as ") {
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
        out.push(bytes[i] as char);
        i += 1;
    }

    out
}

fn remove_type_annotations(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let b = input.as_bytes();
    let len = b.len();

    while i < len {
        let c = b[i] as char;
        if c == ':' {
            // Look behind for something that looks like an identifier or ')' (return type)
            let mut j = i;
            while j > 0 && (b[j - 1] as char).is_ascii_whitespace() {
                j -= 1;
            }
            let prev = if j > 0 { b[j - 1] as char } else { '\0' };
            let looks_like_type_context = is_identifier_char(prev) || prev == ')';
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
                        ',' | '=' | ';' | '\n' => {
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
        out.push(c);
        i += 1;
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
        out.push(c);
        i += 1;
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
}
