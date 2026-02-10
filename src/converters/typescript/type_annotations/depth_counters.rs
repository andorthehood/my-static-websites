/// Represents depth counters for balanced delimiter tracking
#[allow(clippy::struct_field_names)]
pub struct DepthCounters {
    pub angle_depth: i32,
    pub paren_depth: i32,
    pub bracket_depth: i32,
    pub brace_depth: i32,
}

impl DepthCounters {
    pub fn new() -> Self {
        Self {
            angle_depth: 0,
            paren_depth: 0,
            bracket_depth: 0,
            brace_depth: 0,
        }
    }

    pub fn all_zero(&self) -> bool {
        self.angle_depth == 0
            && self.paren_depth == 0
            && self.bracket_depth == 0
            && self.brace_depth == 0
    }

    /// Updates depth counters based on the character.
    /// Returns `true` if an unmatched closing delimiter is encountered (signals to break).
    pub fn update(&mut self, ch: char) -> bool {
        match ch {
            '<' => self.angle_depth += 1,
            '>' => {
                if self.angle_depth > 0 {
                    self.angle_depth -= 1;
                }
            }
            '(' => self.paren_depth += 1,
            ')' => {
                if self.paren_depth > 0 {
                    self.paren_depth -= 1;
                } else {
                    return true; // Signal to break
                }
            }
            '[' => self.bracket_depth += 1,
            ']' => {
                if self.bracket_depth > 0 {
                    self.bracket_depth -= 1;
                }
            }
            '{' => self.brace_depth += 1,
            '}' => {
                if self.brace_depth > 0 {
                    self.brace_depth -= 1;
                } else {
                    return true; // Signal to break
                }
            }
            _ => {}
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_counters_are_zero() {
        let counters = DepthCounters::new();
        assert_eq!(counters.angle_depth, 0);
        assert_eq!(counters.paren_depth, 0);
        assert_eq!(counters.bracket_depth, 0);
        assert_eq!(counters.brace_depth, 0);
        assert!(counters.all_zero());
    }

    #[test]
    fn update_increments_angle_depth() {
        let mut counters = DepthCounters::new();
        assert!(!counters.update('<'));
        assert_eq!(counters.angle_depth, 1);
        assert!(!counters.all_zero());
    }

    #[test]
    fn update_decrements_angle_depth() {
        let mut counters = DepthCounters::new();
        counters.angle_depth = 1;
        assert!(!counters.update('>'));
        assert_eq!(counters.angle_depth, 0);
        assert!(counters.all_zero());
    }

    #[test]
    fn update_increments_paren_depth() {
        let mut counters = DepthCounters::new();
        assert!(!counters.update('('));
        assert_eq!(counters.paren_depth, 1);
    }

    #[test]
    fn update_decrements_paren_depth() {
        let mut counters = DepthCounters::new();
        counters.paren_depth = 1;
        assert!(!counters.update(')'));
        assert_eq!(counters.paren_depth, 0);
    }

    #[test]
    fn update_signals_break_on_unmatched_closing_paren() {
        let mut counters = DepthCounters::new();
        assert!(counters.update(')'));
        assert_eq!(counters.paren_depth, 0);
    }

    #[test]
    fn update_increments_bracket_depth() {
        let mut counters = DepthCounters::new();
        assert!(!counters.update('['));
        assert_eq!(counters.bracket_depth, 1);
    }

    #[test]
    fn update_decrements_bracket_depth() {
        let mut counters = DepthCounters::new();
        counters.bracket_depth = 1;
        assert!(!counters.update(']'));
        assert_eq!(counters.bracket_depth, 0);
    }

    #[test]
    fn update_increments_brace_depth() {
        let mut counters = DepthCounters::new();
        assert!(!counters.update('{'));
        assert_eq!(counters.brace_depth, 1);
    }

    #[test]
    fn update_decrements_brace_depth() {
        let mut counters = DepthCounters::new();
        counters.brace_depth = 1;
        assert!(!counters.update('}'));
        assert_eq!(counters.brace_depth, 0);
    }

    #[test]
    fn update_signals_break_on_unmatched_closing_brace() {
        let mut counters = DepthCounters::new();
        assert!(counters.update('}'));
        assert_eq!(counters.brace_depth, 0);
    }

    #[test]
    fn all_zero_detects_mixed_depths() {
        let mut counters = DepthCounters::new();
        counters.angle_depth = 1;
        assert!(!counters.all_zero());
        counters.angle_depth = 0;
        counters.paren_depth = 1;
        assert!(!counters.all_zero());
    }
}
