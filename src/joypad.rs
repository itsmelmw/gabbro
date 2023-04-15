use crate::{cpu::IntReg, interfaces::GameboyJoypad};

/// An enum representing a directional button.
#[repr(u8)]
enum DirBtn {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
}

/// An enum representing an action button.
#[repr(u8)]
enum ActBtn {
    A = 0,
    B = 1,
    Select = 2,
    Start = 3,
}

/// The `P1` IO register. An 8-bit value consisting of 4 bits that represent pressed and released buttons,
/// and 2 bits that toggle whether action and directional buttons are enabled.
pub struct P1 {
    byte: u8,
}

impl P1 {
    /// Initializes a new `P1` register.
    fn new() -> Self {
        Self { byte: 0xcf }
    }
    /// Reads the value of the `P1` register.
    pub fn byte(&self) -> u8 {
        self.byte
    }
    /// Writes `val` to the `P1` register.
    pub fn set_byte(&mut self, val: u8) {
        self.byte = (val & 0x30) | self.buttons();
    }
    /// Reads the flags that represent the button state.
    fn buttons(&self) -> u8 {
        self.byte & 0x0f
    }
    /// Returns whether the directional buttons are enabled.
    fn direction_enabled(&self) -> bool {
        (self.byte >> 4) & 1 == 0
    }
    /// Returns whether the action buttons are enabled.
    fn action_enabled(&self) -> bool {
        (self.byte >> 5) & 1 == 0
    }
    /// Resets the flags that represent the button state.
    fn clear_buttons(&mut self) {
        self.byte |= 0x0f;
    }
    /// Sets the `bit`th flag if `press` is `true`.
    /// `bit` should be a value between 1 and 4.
    fn set_button(&mut self, bit: u8, press: bool) {
        if press {
            self.byte &= !(1 << bit);
        }
    }
}

/// Emulates the Game Boy joypad.
pub struct JoypadController {
    pub p1: P1,
    joypad: Box<dyn GameboyJoypad>,
}

impl JoypadController {
    /// Initializes a new joypad controller.
    pub fn new(joypad: Box<dyn GameboyJoypad>) -> Self {
        Self {
            p1: P1::new(),
            joypad,
        }
    }

    /// Emulates a machine cycle of the joypad.
    /// Updates `P1` according to the pressed buttons given by the joypad interface.
    /// May request the JOYPAD interrupt.
    pub fn step(&mut self, ints: &mut IntReg) {
        let state = self.joypad.get_button_state();
        let old = self.p1.buttons();
        self.p1.clear_buttons();
        if self.p1.action_enabled() {
            self.p1.set_button(ActBtn::A as u8, state.a);
            self.p1.set_button(ActBtn::B as u8, state.b);
            self.p1.set_button(ActBtn::Select as u8, state.select);
            self.p1.set_button(ActBtn::Start as u8, state.start);
        }
        if self.p1.direction_enabled() {
            self.p1.set_button(DirBtn::Right as u8, state.right);
            self.p1.set_button(DirBtn::Left as u8, state.left);
            self.p1.set_button(DirBtn::Up as u8, state.up);
            self.p1.set_button(DirBtn::Down as u8, state.down);
        }
        if old != self.p1.buttons() {
            ints.irq_joypad();
        }
    }
}
