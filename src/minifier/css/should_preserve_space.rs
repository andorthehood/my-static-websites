#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("should_preserve_space_x86_64.s"));

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn should_preserve_space_scan(last_char: u8, next_char: u8) -> u8;
}

/// Determines if a space should be preserved between characters - x86_64 assembly optimized version
#[cfg(target_arch = "x86_64")]
pub fn should_preserve_space_asm(result: &str, next_char: char) -> bool {
    if result.is_empty() {
        return false;
    }

    let last_char = result.chars().last().unwrap_or('\0');
    
    // Use assembly optimization for ASCII characters
    if last_char.is_ascii() && next_char.is_ascii() {
        let last_byte = last_char as u8;
        let next_byte = next_char as u8;
        let preserve = unsafe { should_preserve_space_scan(last_byte, next_byte) };
        return preserve != 0;
    }
    
    // Fallback to Rust implementation for non-ASCII characters
    should_preserve_space_rust_fallback(result, next_char)
}

/// Determines if a space should be preserved between characters - pure Rust fallback version
#[cfg(not(target_arch = "x86_64"))]
pub fn should_preserve_space_asm(result: &str, next_char: char) -> bool {
    should_preserve_space_rust_fallback(result, next_char)
}

/// Pure Rust implementation of space preservation logic
pub fn should_preserve_space_rust_fallback(result: &str, next_char: char) -> bool {
    if result.is_empty() {
        return false;
    }

    let last_char = result.chars().last().unwrap_or('\0');

    // Preserve space in specific cases where it's needed for CSS to work correctly
    // Between a number/percentage and a word (e.g., "100% 2px", "1rem solid")
    (last_char.is_ascii_digit() || last_char == '%') && next_char.is_alphabetic() ||
    // Between a percentage and a number (e.g., "100% 2px")
    last_char == '%' && next_char.is_ascii_digit() ||
    // Between words and numbers (e.g., "solid #fff", "auto 10px")
    last_char.is_alphabetic() && (next_char.is_ascii_digit() || next_char == '#') ||
    // Between measurement units and words (e.g., "px solid", "rem auto")
    (last_char == 'x' || last_char == 'm' || last_char == '%') && next_char.is_alphabetic() ||
    // Between closing parenthesis and other values (e.g., ") 50%")
    last_char == ')' && (next_char.is_ascii_digit() || next_char.is_alphabetic()) ||
    // Between values in functions like rgba() or linear-gradient()
    last_char == ',' && next_char == '#' ||
    // Between numbers and hash colors (e.g., "0 #fff")
    last_char.is_ascii_digit() && next_char == '#' ||
    // Between CSS selectors (e.g., ".foo .bar" should not become ".foo.bar")
    (last_char.is_alphanumeric() || last_char == ']' || last_char == ')') && next_char == '.' ||
    // Between CSS selectors with IDs (e.g., "div #id" but not "color: #fff")
    last_char.is_alphabetic() && next_char == '#' ||
    // Before negative numbers - simplified to just before minus signs after certain characters
    (last_char.is_ascii_digit() || last_char == 'm' || last_char == 'x' || last_char == '%') && next_char == '-' ||
    // Between words and negative numbers (e.g., "inset -1rem")
    last_char.is_alphabetic() && next_char == '-' ||
    // Between alphanumeric characters where CSS requires spaces
    (last_char.is_alphanumeric() && next_char.is_alphanumeric() &&
     !matches!(next_char, '{' | '}' | ';' | ':' | ',' | '(' | ')' | '[' | ']' | '>' | '+' | '~' | '*' | '/' | '='))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembly_optimization_direct() {
        // Test that assembly and Rust implementations give same results for common cases
        let test_cases = [
            ("color:10px", 's'), // digit + alphabetic
            ("width:100%", '2'), // percent + digit  
            ("border:solid", '#'), // alphabetic + hash
            ("calc(100% - 10px)", '5'), // parenthesis + digit
            ("rgba(255,0,0,0.5),", '#'), // comma + hash
            ("margin:10px", '-'), // digit/unit + minus
            ("div", '#'), // alphabetic + hash (ID selector)
            ("", 'a'), // empty string case
        ];

        for (result_str, next_char) in test_cases {
            let assembly_result = should_preserve_space_asm(result_str, next_char);
            let rust_result = should_preserve_space_rust_fallback(result_str, next_char);
            assert_eq!(assembly_result, rust_result, 
                "Assembly and Rust results differ for '{}' + '{}'", result_str, next_char);
        }
    }
}