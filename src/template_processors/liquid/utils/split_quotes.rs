#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("split_quotes_x86_64.s"));

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn split_quotes_scan(
        ptr: *const u8,
        len: usize,
        splits: *mut usize,
        max_splits: usize,
    ) -> usize;
}

/// Splits a string on commas while respecting quotes - x86_64 assembly optimized version
#[cfg(target_arch = "x86_64")]
pub fn split_respecting_quotes(input: &str) -> Vec<String> {
    let input_bytes = input.as_bytes();
    let mut splits = [0usize; 32]; // Support up to 32 parts

    let split_count = unsafe {
        split_quotes_scan(
            input_bytes.as_ptr(),
            input_bytes.len(),
            splits.as_mut_ptr(),
            32,
        )
    };

    let mut parts = Vec::new();
    let mut start = 0;

    // Process each split position
    for i in 0..split_count {
        let comma_pos = splits[i];
        if comma_pos > start {
            let part = &input[start..comma_pos];
            let trimmed = part.trim();
            if !trimmed.is_empty() {
                parts.push(trimmed.to_string());
            }
        }
        start = comma_pos + 1; // Skip the comma
    }

    // Handle the last part after the final comma
    if start < input.len() {
        let part = &input[start..];
        let trimmed = part.trim();
        if !trimmed.is_empty() {
            parts.push(trimmed.to_string());
        }
    }

    parts
}

/// Splits a string on commas while respecting quotes - pure Rust fallback version
#[cfg(not(target_arch = "x86_64"))]
pub fn split_respecting_quotes(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '"';

    for ch in input.chars() {
        match ch {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
                current.push(ch);
            }
            ch if in_quotes && ch == quote_char => {
                in_quotes = false;
                current.push(ch);
            }
            ',' if !in_quotes => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_respecting_quotes() {
        let result = split_respecting_quotes("\"active\", \"true\"");
        assert_eq!(result, vec!["\"active\"", "\"true\""]);

        let result = split_respecting_quotes("active, true, \"quoted, value\"");
        assert_eq!(result, vec!["active", "true", "\"quoted, value\""]);

        let result = split_respecting_quotes("simple, values");
        assert_eq!(result, vec!["simple", "values"]);
    }

    #[test]
    fn test_split_respecting_quotes_edge_cases() {
        // Test empty string
        let result = split_respecting_quotes("");
        assert_eq!(result, Vec::<String>::new());

        // Test no commas
        let result = split_respecting_quotes("simple string");
        assert_eq!(result, vec!["simple string"]);

        // Test nested quotes
        let result = split_respecting_quotes("\"outer 'inner' outer\", next");
        assert_eq!(result, vec!["\"outer 'inner' outer\"", "next"]);

        // Test quoted commas
        let result = split_respecting_quotes("\"part1, still part1\", \"part2, still part2\"");
        assert_eq!(
            result,
            vec!["\"part1, still part1\"", "\"part2, still part2\""]
        );

        // Test single quotes
        let result = split_respecting_quotes("'single, quote', normal");
        assert_eq!(result, vec!["'single, quote'", "normal"]);
    }
}
