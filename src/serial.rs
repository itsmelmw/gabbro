use crate::{cpu::IntReg, interfaces::GameboySerial};

/// A temporary simple implementation of the serial controller.
/// Made only to be used for Blargg's Game Boy CPU tests.
pub struct SerialController {
    serial: Box<dyn GameboySerial>,
    pub sb: u8,
    pub sc: u8,
}

impl SerialController {
    pub fn new(serial: Box<dyn GameboySerial>) -> Self {
        Self {
            serial,
            sb: 0x00,
            sc: 0x7e,
        }
    }

    pub fn step(&mut self, _ints: &mut IntReg) {
        if (self.sc >> 7) & 1 != 0 {
            self.sc &= !(1 << 7);
            self.serial.transfer(self.sb);
        }
    }
}
