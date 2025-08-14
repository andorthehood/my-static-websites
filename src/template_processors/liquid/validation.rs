#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("validation_x86_64.s"));

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn liquid_is_valid_variable_name(ptr: *const u8, len: usize) -> u8;
}

/// Validates if a variable name follows Liquid naming conventions.
/// Supports simple names and dot notation (including numeric indices like users.0.name).
fn is_valid_variable_name_rust(name: &str) -> bool {
    let mut chars = name.chars();
    if let Some(first) = chars.next() {
        (first.is_alphabetic() || first == '_')
            && chars.all(|c| c.is_alphanumeric() || c == '_' || c == '.')
    } else {
        false
    }
}

#[cfg(target_arch = "x86_64")]
/// Validates if a variable name follows Liquid naming conventions with an x86_64 fast path.
pub fn is_valid_variable_name(name: &str) -> bool {
    if !name.is_ascii() {
        // Preserve Unicode behavior via the Rust implementation
        return is_valid_variable_name_rust(name);
    }
    if name.is_empty() {
        return false;
    }
    unsafe { liquid_is_valid_variable_name(name.as_ptr(), name.len()) != 0 }
}

#[cfg(not(target_arch = "x86_64"))]
/// Validates if a variable name follows Liquid naming conventions (pure Rust).
pub fn is_valid_variable_name(name: &str) -> bool {
    is_valid_variable_name_rust(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_variable_name() {
        assert!(!is_valid_variable_name(""));
        assert!(!is_valid_variable_name("1name"));
        assert!(!is_valid_variable_name("name@"));
        assert!(!is_valid_variable_name("name space"));
        assert!(!is_valid_variable_name("name[0]"));
        assert!(!is_valid_variable_name("name[abc]"));
        assert!(!is_valid_variable_name("name-value"));
    }

    #[test]
    fn test_is_valid_variable_name() {
        assert!(is_valid_variable_name("name"));
        assert!(is_valid_variable_name("_name"));
        assert!(is_valid_variable_name("name123"));
        assert!(is_valid_variable_name("user.name"));
        assert!(is_valid_variable_name("deeply.nested.value"));
        assert!(is_valid_variable_name("people.0.name"));
        assert!(is_valid_variable_name("data.123.details.value"));
        assert!(is_valid_variable_name("items.0.1.title"));
    }
}
