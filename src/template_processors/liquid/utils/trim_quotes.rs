#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("trim_quotes_x86_64.s"));

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn trim_quotes_scan(ptr: *const u8, len: usize, out_start: *mut usize, out_end: *mut usize);
}

/// Removes surrounding quotes from a string if present
/// Handles both single and double quotes
pub fn trim_quotes(s: &str) -> &str {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let mut start_idx: usize = 0;
        let mut end_idx: usize = s.len();
        trim_quotes_scan(
            s.as_ptr(),
            s.len(),
            &mut start_idx as *mut usize,
            &mut end_idx as *mut usize,
        );
        return s.get_unchecked(start_idx..end_idx);
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        let bytes = s.as_bytes();
        let mut start = 0usize;
        let mut end = bytes.len();

        while start < end {
            let b = bytes[start];
            if b == b'"' || b == b'\'' {
                start += 1;
            } else {
                break;
            }
        }

        while end > start {
            let b = bytes[end - 1];
            if b == b'"' || b == b'\'' {
                end -= 1;
            } else {
                break;
            }
        }

        &s[start..end]
    }
}
