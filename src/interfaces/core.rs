use super::{GameboyJoypad, GameboyLcd, GameboySerial};
use std::io::{stdout, Write};

/// Implementation of the Game Boy LCD that does not handle any pushed pixels.
/// This is the default for if no LCD gets attached.
pub struct NoLcd;
impl GameboyLcd for NoLcd {}

/// Implementation of the Game Boy joypad that always assumes no buttons are pressed.
/// This is the default for if no joypad gets attached.
pub struct NoJoypad;
impl GameboyJoypad for NoJoypad {}

/// Implementation of the Game Boy serial controller that does not handle serial transfers.
/// This is the default for if no serial controller gets attached.
pub struct NoSerial;
impl GameboySerial for NoSerial {}

/// Implementation of the Game Boy serial controller that prints the transfered byte as a character to `stdout`.
/// This is useful to retrieve Blargg's CPU test output when no LCD is attached.
pub struct BlarggSerial;
impl GameboySerial for BlarggSerial {
    fn transfer(&mut self, val: u8) {
        print!("{}", val as char);
        let _ = stdout().flush();
    }
}
