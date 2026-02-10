use crate::converters::typescript::utils::push_char_from;

mod colon;
mod comments;
mod depth_counters;
mod parse_state;
mod property_detection;
mod strings;
mod type_skipping;

use colon::handle_colon;
use comments::handle_comments;
use parse_state::ParseState;
use strings::handle_strings;

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
