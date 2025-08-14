#[cfg(target_arch = "x86_64")]
use core::arch::asm;

/// Removes surrounding quotes from a string if present
/// Handles both single and double quotes
pub fn trim_quotes(s: &str) -> &str {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let ptr = s.as_ptr();
        let len = s.len();
        let mut start_idx: usize = 0;
        let mut end_idx: usize = len;

        asm!(
            // Leading trim: advance start_idx while bytes are ' or "
            "2:",
            "cmp {end}, 0",
            "je 4f",
            "mov dl, BYTE PTR [{ptr} + {start}]",
            "cmp dl, 0x22",       // '"'
            "je 3f",
            "cmp dl, 0x27",       // '\''
            "jne 4f",
            "3:",
            "inc {start}",
            "cmp {start}, {end}",
            "jb 2b",
            // Trailing trim: decrement end_idx while last byte is ' or "
            "4:",
            "5:",
            "cmp {end}, {start}",
            "jbe 6f",
            "mov dl, BYTE PTR [{ptr} + {end} - 1]",
            "cmp dl, 0x22",
            "je 7f",
            "cmp dl, 0x27",
            "jne 6f",
            "7:",
            "dec {end}",
            "jmp 5b",
            "6:",
            ptr = in(reg) ptr,
            start = inout(reg) start_idx,
            end = inout(reg) end_idx,
            out("rdx") _,
            options(nostack)
        );

        // Safety: start_idx and end_idx are computed within [0, len] and start <= end
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
