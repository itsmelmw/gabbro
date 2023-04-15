/// Represents an interrupt IO register. An 8-bit value consisting of 5 interrupt flags.
pub struct IntReg {
    byte: u8,
}

impl IntReg {
    /// Initializes a new interrupt register with a given value `val`.
    fn new(val: u8) -> Self {
        Self { byte: val }
    }
    /// Reads the value of the interrupt register.
    pub fn byte(&self) -> u8 {
        self.byte
    }
    /// Writes `val` to the interrupt register.
    pub fn set_byte(&mut self, val: u8) {
        self.byte = val
    }
    /// Sets the VBLANK interrupt flag.
    pub fn irq_vblank(&mut self) {
        self.byte |= 1 << 0
    }
    /// Sets the LCDSTAT interrupt flag.
    pub fn irq_lcdstat(&mut self) {
        self.byte |= 1 << 1
    }
    /// Sets the TIMER interrupt flag.
    pub fn irq_timer(&mut self) {
        self.byte |= 1 << 2
    }
    /// Sets the SERIAL interrupt flag.
    pub fn _irq_serial(&mut self) {
        // TODO
        self.byte |= 1 << 3
    }
    /// Sets the JOYPAD interrupt flag.
    pub fn irq_joypad(&mut self) {
        self.byte |= 1 << 4
    }
    /// Returns whether the VBLANK interrupt was requested.
    fn vblank(&self) -> bool {
        self.byte & 1 != 0
    }
    /// Returns whether the LCDSTAT interrupt was requested.
    fn lcdstat(&self) -> bool {
        (self.byte >> 1) & 1 != 0
    }
    /// Returns whether the TIMER interrupt was requested.
    fn timer(&self) -> bool {
        (self.byte >> 2) & 1 != 0
    }
    /// Returns whether the SERIAL interrupt is requested.
    fn serial(&self) -> bool {
        (self.byte >> 3) & 1 != 0
    }
    /// Returns whether the JOYPAD interrupt is requested.
    fn joypad(&self) -> bool {
        (self.byte >> 4) & 1 != 0
    }
    /// Resets the VBLANK interrupt flag.
    fn reset_vblank(&mut self) {
        self.byte &= !(1 << 0)
    }
    /// Resets the LCDSTAT interrupt flag.
    fn reset_lcdstat(&mut self) {
        self.byte &= !(1 << 1)
    }
    /// Resets the TIMER interrupt flag.
    fn reset_timer(&mut self) {
        self.byte &= !(1 << 2)
    }
    /// Resets the SERIAL interrupt flag.
    fn reset_serial(&mut self) {
        self.byte &= !(1 << 3)
    }
    /// Resets the JOYPAD interrupt flag.
    fn reset_joypad(&mut self) {
        self.byte &= !(1 << 4)
    }
}

/// Stores the `IF` register (`flags`) and the `IE` register (`enable`).
pub struct InterruptControl {
    /// The `IF` register.
    pub flags: IntReg,
    /// The `IE` register.
    pub enable: IntReg,
}

impl InterruptControl {
    /// Initializes the `IF` and `IE` registers.
    pub fn new() -> Self {
        Self {
            flags: IntReg::new(0xe1),
            enable: IntReg::new(0x00),
        }
    }

    /// Checks whether an interrupt was requested.
    /// If so, it resets the corresponding flag and returns the address we should jump to.
    pub fn step(&mut self) -> Option<u16> {
        if self.flags.vblank() && self.enable.vblank() {
            log::debug!("CPU: VBLANK interrupt triggered");
            self.flags.reset_vblank();
            return Some(0x0040);
        } else if self.flags.lcdstat() && self.enable.lcdstat() {
            log::debug!("CPU: LCDSTAT interrupt triggered");
            self.flags.reset_lcdstat();
            return Some(0x0048);
        } else if self.flags.timer() && self.enable.timer() {
            log::debug!("CPU: TIMER interrupt triggered");
            self.flags.reset_timer();
            return Some(0x0050);
        } else if self.flags.serial() && self.enable.serial() {
            log::debug!("CPU: SERIAL interrupt triggered");
            self.flags.reset_serial();
            return Some(0x0058);
        } else if self.flags.joypad() && self.enable.joypad() {
            log::debug!("CPU: JOYPAD interrupt triggered");
            self.flags.reset_joypad();
            return Some(0x0060);
        }
        None
    }

    /// Checks whether an interrupt request is pending.
    pub fn pending(&self) -> bool {
        self.flags.byte() & self.enable.byte() != 0
    }
}
