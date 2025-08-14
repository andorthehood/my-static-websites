#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("is_identifier_char_x86_64.s"));

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn ts_is_identifier_char(b: u8) -> u8;
}

#[cfg(target_arch = "x86_64")]
pub fn is_identifier_byte(b: u8) -> bool {
    unsafe { ts_is_identifier_char(b) != 0 }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn is_identifier_byte(b: u8) -> bool {
    (b as char).is_ascii_alphanumeric() || b == b'_' || b == b'$'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rust_ref(b: u8) -> bool {
        (b as char).is_ascii_alphanumeric() || b == b'_' || b == b'$'
    }

    #[test]
    fn ascii_map_matches_rust_ref() {
        for b in 0u8..=127u8 {
            assert_eq!(is_identifier_byte(b), rust_ref(b), "mismatch for byte {b}");
        }
    }
}
