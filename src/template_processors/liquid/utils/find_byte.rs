#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("find_byte_x86_64.s"));

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn find_byte_scan(ptr: *const u8, len: usize, byte: u8) -> usize;
}

#[cfg(target_arch = "x86_64")]
pub fn find_byte_index(haystack: &[u8], needle: u8) -> Option<usize> {
    let idx = unsafe { find_byte_scan(haystack.as_ptr(), haystack.len(), needle) };
    if idx == usize::MAX {
        None
    } else {
        Some(idx)
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn find_byte_index(haystack: &[u8], needle: u8) -> Option<usize> {
    haystack.iter().position(|&b| b == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_colon_position() {
        let s = b"abc:def";
        assert_eq!(find_byte_index(s, b':'), Some(3));
    }

    #[test]
    fn returns_none_when_not_found() {
        let s = b"abcdef";
        assert_eq!(find_byte_index(s, b':'), None);
    }

    #[test]
    fn handles_empty() {
        let s = b"";
        assert_eq!(find_byte_index(s, b':'), None);
    }

    #[test]
    fn finds_first_of_many() {
        let s = b"a:b:c";
        assert_eq!(find_byte_index(s, b':'), Some(1));
    }
}
