pub mod core;
#[cfg(feature = "sdl2")]
pub mod sdl2;

/// An enum representing the color of a pixel on the Game Boy LCD.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LcdColor {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

/// A trait with functions that the Game Boy PPU calls when updating the LCD.
pub trait GameboyLcd {
    /// Gets called when a new pixel is pushed to the LCD.
    fn push_pixel(&mut self, _color: LcdColor) {}
    /// Gets called when all pixels of a frame have been pushed to the LCD.
    fn frame_ready(&mut self) {}
}

/// Represents the current state of the pressed buttons of the Game Boy.
/// If a button value is `true`, it is pressed. When it is `false`, it is released.
#[derive(Clone, Copy)]
pub struct ButtonState {
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
}

impl ButtonState {
    /// Initializes a new button state with no buttons pressed.
    pub fn new() -> Self {
        Self {
            right: false,
            left: false,
            up: false,
            down: false,
            a: false,
            b: false,
            select: false,
            start: false,
        }
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait the Game Boy uses to retrieve the current button state.
pub trait GameboyJoypad {
    /// Should return the current state of the buttons.
    /// If a button is pressed, its value should be `true`. If it is released, it should be `false`.
    fn get_button_state(&mut self) -> ButtonState {
        ButtonState::new()
    }
}

/// A temporary simple implementation of a serial interface.
/// Serial transfer is currently not implemented properly.
/// This currently only exists to use for Blargg's Game Boy CPU test ROMs.
pub trait GameboySerial {
    /// A function called when a serial transfer should take place.
    /// Serial transfer is currently not implemented properly.
    /// This currently only exists to use for Blargg's Game Boy CPU test ROMs.
    fn transfer(&mut self, _val: u8) {}
}
