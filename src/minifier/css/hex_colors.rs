#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("../optimize_hex_color_x86_64.s"));

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn optimize_hex_color_scan(ptr: *const u8, len: usize, can_shorten: *mut u8) -> usize;
}

#[cfg(target_arch = "x86_64")]
/// Optimizes a hex color by shortening it from 6 digits to 3 digits when possible
/// Returns the optimized color string (without the # prefix)
pub fn optimize_hex_color(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    // Collect up to 6 characters to create a buffer for scanning
    let mut temp_chars = Vec::new();
    let mut peek_count = 0;

    // Peek ahead to see what characters we have
    loop {
        if peek_count >= 6 {
            break;
        }
        if let Some(&next_ch) = chars.peek() {
            if next_ch.is_ascii_hexdigit() {
                temp_chars.push(next_ch as u8);
                peek_count += 1;
                chars.next(); // consume the character
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if temp_chars.is_empty() {
        return String::new();
    }

    // Use assembly to check if we can shorten
    let mut can_shorten: u8 = 0;
    let consumed =
        unsafe { optimize_hex_color_scan(temp_chars.as_ptr(), temp_chars.len(), &mut can_shorten) };

    // Convert consumed bytes back to chars
    let color_chars: Vec<char> = temp_chars[..consumed].iter().map(|&b| b as char).collect();

    // If we have exactly 6 hex digits and can shorten
    if consumed == 6 && can_shorten != 0 {
        // Return the shortened version
        format!("{}{}{}", color_chars[0], color_chars[2], color_chars[4])
    } else {
        // Return the full version
        color_chars.into_iter().collect()
    }
}

#[cfg(not(target_arch = "x86_64"))]
/// Optimizes a hex color by shortening it from 6 digits to 3 digits when possible
/// Returns the optimized color string (without the # prefix)
pub fn optimize_hex_color(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    // Collect the next 6 characters to see if it's a hex color
    let mut color_chars = Vec::new();
    for _ in 0..6 {
        if let Some(&next_ch) = chars.peek() {
            if next_ch.is_ascii_hexdigit() {
                color_chars.push(chars.next().unwrap());
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // If we have exactly 6 hex digits, check if we can shorten it
    if color_chars.len() == 6 {
        let can_shorten = color_chars[0] == color_chars[1]
            && color_chars[2] == color_chars[3]
            && color_chars[4] == color_chars[5];

        if can_shorten {
            // Return the shortened version
            format!("{}{}{}", color_chars[0], color_chars[2], color_chars[4])
        } else {
            // Return the full version
            color_chars.into_iter().collect()
        }
    } else {
        // Not a 6-digit hex color, return as-is
        color_chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_hex_color_function() {
        // Test shortenable colors
        let mut chars = "999999".chars().peekable();
        assert_eq!(optimize_hex_color(&mut chars), "999");

        let mut chars = "aabbcc".chars().peekable();
        assert_eq!(optimize_hex_color(&mut chars), "abc");

        let mut chars = "000000".chars().peekable();
        assert_eq!(optimize_hex_color(&mut chars), "000");

        // Test non-shortenable colors
        let mut chars = "123456".chars().peekable();
        assert_eq!(optimize_hex_color(&mut chars), "123456");

        let mut chars = "abcdef".chars().peekable();
        assert_eq!(optimize_hex_color(&mut chars), "abcdef");

        // Test short colors (3 digits)
        let mut chars = "fff".chars().peekable();
        assert_eq!(optimize_hex_color(&mut chars), "fff");

        let mut chars = "000".chars().peekable();
        assert_eq!(optimize_hex_color(&mut chars), "000");

        // Test invalid/incomplete colors
        let mut chars = "12".chars().peekable();
        assert_eq!(optimize_hex_color(&mut chars), "12");

        let mut chars = "1234".chars().peekable();
        assert_eq!(optimize_hex_color(&mut chars), "1234");
    }
}
