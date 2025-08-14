#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("find_equal_x86_64.s"));

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn find_equal_scan(ptr: *const u8, len: usize) -> usize;
}

#[cfg(target_arch = "x86_64")]
pub fn find_equal_index(haystack: &[u8]) -> Option<usize> {
    let idx = unsafe { find_equal_scan(haystack.as_ptr(), haystack.len()) };
    if idx == usize::MAX {
        None
    } else {
        Some(idx)
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn find_equal_index(haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == b'=')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_equal_position() {
        let s = b"a=b";
        assert_eq!(find_equal_index(s), Some(1));
    }

    #[test]
    fn returns_none_when_not_found() {
        let s = b"abc";
        assert_eq!(find_equal_index(s), None);
    }

    #[test]
    fn handles_empty() {
        let s = b"";
        assert_eq!(find_equal_index(s), None);
    }

    #[test]
    fn finds_first_of_many() {
        let s = b"a==b";
        assert_eq!(find_equal_index(s), Some(1));
    }
}
