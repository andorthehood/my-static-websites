#[cfg(target_arch = "x86_64")]
extern "C" {
    fn trim_quotes_scan(ptr: *const u8, len: usize, out_start: *mut usize, out_end: *mut usize);
}

#[cfg(target_arch = "x86_64")]
fn scan_trim_quotes_asm(s: &str) -> (usize, usize) {
    let mut start = 0usize;
    let mut end = s.len();
    unsafe {
        trim_quotes_scan(
            s.as_ptr(),
            s.len(),
            &mut start as *mut usize,
            &mut end as *mut usize,
        )
    };
    (start, end)
}

#[cfg(not(target_arch = "x86_64"))]
fn scan_trim_quotes_asm(s: &str) -> (usize, usize) {
    let bytes = s.as_bytes();
    let mut start = 0usize;
    let mut end = bytes.len();
    while start < end && (bytes[start] == b'\'' || bytes[start] == b'\"') {
        start += 1;
    }
    while end > start && (bytes[end - 1] == b'\'' || bytes[end - 1] == b'\"') {
        end -= 1;
    }
    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template_processors::liquid::utils::quote_utils::trim_quotes;

    #[test]
    fn matches_trim_quotes_behavior() {
        let cases = [
            "\"hello\"",
            "'hello'",
            "",
            "noquotes",
            "\"mix'",
            "''",
            "\"\"",
        ];
        for &c in &cases {
            let (start, end) = scan_trim_quotes_asm(c);
            assert_eq!(&c[start..end], trim_quotes(c));
        }
    }
}
