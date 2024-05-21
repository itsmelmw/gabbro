use std::io::{stdout, Write};

use super::Serial;

/// Implementation of the Game Boy serial controller that prints the transfered byte as a character to `stdout`.
/// This is useful to retrieve Blargg's CPU test output when no LCD is attached.
pub struct BlarggSerial;
impl Serial for BlarggSerial {
    fn transfer(&mut self, val: u8) {
        print!("{}", val as char);
        let _ = stdout().flush();
    }
}
