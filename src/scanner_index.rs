/* Date Created: 03/06/2025. */

//! The implementation for the Java class variables ``start``, ``current`` and ``line`` in 
//! the **The Scanner Class** in 
//! [https://craftinginterpreters.com/scanning.html](https://craftinginterpreters.com/scanning.html). 
//! 
//! This implementation supports UTF8. The field ``byte_count`` keeps track of the 
//! accumulative total bytes up to the character at the ``current``th index.
//! 
//! I have discussed this implementation in detail in this post [Rust: Working with UTF-8 Text](https://behainguyen.wordpress.com/2025/06/09/rust-working-with-utf-8-text/).

/// See the Java class variables ``start``, ``current`` and ``line`` in 
/// the **The Scanner Class** in 
/// [https://craftinginterpreters.com/scanning.html](https://craftinginterpreters.com/scanning.html). 
pub struct ScannerIndex {
    start: usize,
    current: usize,
    /// An extra field, not present in the original Java implementation. This field is to 
    /// support UTF8. It keeps track of the accumulative total bytes up to the character at 
    /// the ``current``th index.
    byte_count: usize,
    line: usize,
}

impl ScannerIndex {
    pub const fn new() -> Self {
        ScannerIndex { 
            start: 0,
            current: 0,
            byte_count: 0,
            line: 1,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn byte_count(&self) -> usize {
        self.byte_count        
    }

    pub fn line(&self) -> usize {
        self.line
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("ScannerIndex -- start: {}, current: {}, byte_count: {}, line: {}", 
                  self.start, self.current, self.byte_count, self.line);
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.start = 0;
        self.current = 0;
        self.byte_count = 0;
        self.line = 1;
    }

    pub fn set_start(&mut self, val: usize) {
        self.start = val;
    }

    pub fn inc_lexeme_indexes(&mut self, val: usize) {
        self.current += 1;
        self.byte_count += val;
    }

    pub fn inc_line(&mut self) {
        self.line += 1;
    }
}
