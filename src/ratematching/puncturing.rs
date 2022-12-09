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
        return output == 0;
    }
}

impl Default for Puncturer {
    /// Create a default puncturer that does not puncture.
    fn default() -> Self {
        Self {
            state: 0,
            width: 0,
        }
    }
}
