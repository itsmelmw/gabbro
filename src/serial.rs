use crate::{cpu::interrupts::IntReg, peripherals::Serial};

/// A temporary simple implementation of the serial controller.
/// Made only to be used for Blargg's Game Boy CPU tests.
pub struct SerialController<S>
where
    S: Serial,
{
    serial: S,
    pub sb: u8,
    pub sc: u8,
}

impl<S> SerialController<S>
where
    S: Serial,
{
    pub fn new(serial: S) -> Self {
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
