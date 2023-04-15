use crate::cpu::IntReg;

/// An enum representing the mode of the timer.
enum ClockMode {
    C16 = 1,
    C64 = 2,
    C256 = 3,
    C1024 = 0,
}

/// The `TAC` IO register of the timer, allowing control over the timer.
pub struct Tac {
    byte: u8,
}

impl Tac {
    /// Initializes a new `TAC` IO register.
    fn new() -> Self {
        Self { byte: 0xf8 }
    }
    /// Reads the value of the `TAC` register.
    pub fn byte(&self) -> u8 {
        self.byte
    }
    /// Writes `val` to the `TAC` register.
    pub fn set_byte(&mut self, val: u8) {
        self.byte = val
    }
    /// Reads the current timer mode selected.
    fn mode(&self) -> ClockMode {
        match self.byte & 3 {
            0 => ClockMode::C1024,
            1 => ClockMode::C16,
            2 => ClockMode::C64,
            3 => ClockMode::C256,
            _ => unreachable!(),
        }
    }
    /// Returns whether the timer is currently enabled.
    fn enabled(&self) -> bool {
        (self.byte >> 2) & 1 != 0
    }
}

/// Emulates the Game Boy timer.
pub struct Timer {
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: Tac,
}

impl Timer {
    /// Initializes a new timer.
    pub fn new() -> Self {
        Self {
            div: 0x1800,
            tima: 0x00,
            tma: 0x00,
            tac: Tac::new(),
        }
    }

    /// Emulates a machine cycle of the timer, according to the current clock mode.
    /// May request the TIMER interrupt.
    pub fn step(&mut self, ints: &mut IntReg) {
        if self.tac.enabled() {
            let bit = match self.tac.mode() {
                ClockMode::C16 => 3,
                ClockMode::C64 => 5,
                ClockMode::C256 => 7,
                ClockMode::C1024 => 9,
            };
            if (self.div & (1 << bit)) != 0 && (self.div.wrapping_add(4) & (1 << bit) == 0) {
                if self.tima == 0xff {
                    ints.irq_timer();
                    self.tima = self.tma;
                } else {
                    self.tima += 1;
                }
            }
        }

        self.div = self.div.wrapping_add(4);
    }
}
