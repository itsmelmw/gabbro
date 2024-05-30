use crate::{cpu::interrupts::IntReg, peripherals::Cable};

/// A temporary simple implementation of the serial controller.
/// Made only to be used for Blargg's Game Boy CPU tests.
pub struct SerialController<C>
where
    C: Cable,
{
    cable: C,
    pub sb: u8,
    pub sc: u8,
}

impl<C> SerialController<C>
where
    C: Cable,
{
    pub fn new(cable: C) -> Self {
        Self {
            cable,
            sb: 0x00,
            sc: 0x7e,
        }
    }

    pub fn step(&mut self, _ints: &mut IntReg) {
        if (self.sc >> 7) & 1 != 0 {
            self.sc &= !(1 << 7);
            self.cable.transfer(self.sb);
        }
    }
}
