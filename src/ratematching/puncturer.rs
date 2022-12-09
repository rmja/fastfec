/// Rate-Matching puncturer
pub struct Puncturer {
    state: usize,
    /// The width of the puncturing pattern.
    width: usize,
}

impl Puncturer {
    /// Create a new puncturer.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the interleaver pattern
    /// * `pattern` - The interleaver pattern. Bit 0 (LSB) is the first bit in the puncture pattern, bit 1 the seconds, etc. up to `width` bits.
    ///
    /// # Examples
    /// Pattern 0b0010 (width: 4) will puncture bit 1 and output bit 0, 2 and 3.
    pub const fn new(width: usize, pattern: usize) -> Self {
        Self {
            state: pattern,
            width,
        }
    }

    /// Get whether a bit should be output, i.e. not punctured.
    pub fn read_output(&mut self) -> bool {
        let output = self.state & 1;
        self.state >>= 1;
        self.state |= output << (self.width - 1);
        output == 0
    }
}

impl Default for Puncturer {
    /// Create a default puncturer that does not puncture.
    fn default() -> Self {
        Self { state: 0, width: 1 }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn default_puncturer_does_not_puncture() {
        // Given
        let mut puncturer = Puncturer::default();

        // When

        // Then
        assert_eq!(true, puncturer.read_output());
        assert_eq!(true, puncturer.read_output());
    }

    #[test]
    fn can_puncture() {
        // Given
        let mut puncturer = Puncturer::new(3, 0b101);

        // When

        // Then
        assert_eq!(false, puncturer.read_output());
        assert_eq!(true, puncturer.read_output());
        assert_eq!(false, puncturer.read_output());

        assert_eq!(false, puncturer.read_output());
        assert_eq!(true, puncturer.read_output());
        assert_eq!(false, puncturer.read_output());
    }
}
