/// Type used for when a peripheral cannot return any errors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoError;

/// An enum representing the color of a pixel on the Game Boy LCD.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LcdColor {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

/// A trait with functions that the Game Boy PPU calls when updating the LCD.
pub trait Lcd {
    /// Type for errors that can occur while handling the LCD callbacks.
    type Error;
    /// Gets called when a new pixel is pushed to the LCD.
    fn push_pixel(&mut self, _color: LcdColor) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Gets called when all pixels of a frame have been pushed to the LCD.
    fn frame_ready(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Lcd for () {
    type Error = NoError;
}

/// A trait with a function the Game Boy APU calls when pushing new audio samples.
pub trait Speaker {
    /// Type for errors that can occur while handling the audio callbacks.
    type Error;
    /// Gets called when a new audio sample is pushed to the speaker.
    fn push_sample(&mut self, _left: f32, _right: f32) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Speaker for () {
    type Error = NoError;
}

/// Represents the current state of the pressed buttons of the Game Boy.
/// If a button value is `true`, it is pressed. When it is `false`, it is released.
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub trait Joypad {
    /// Type for errors that can occur while retrieving the current button state.
    type Error;
    /// Should return the current state of the buttons.
    /// If a button is pressed, its value should be `true`. If it is released, it should be `false`.
    fn get_button_state(&mut self) -> Result<ButtonState, Self::Error> {
        Ok(ButtonState::default())
    }
}

impl Joypad for () {
    type Error = NoError;
}

/// A temporary simple implementation of a serial interface.
/// Serial transfer is currently not implemented properly.
/// This currently only exists to use for Blargg's Game Boy CPU test ROMs.
pub trait Cable {
    /// A function called when a serial transfer should take place.
    /// Serial transfer is currently not implemented properly.
    /// This currently only exists to use for Blargg's Game Boy CPU test ROMs.
    fn transfer(&mut self, _val: u8) {}
}

impl Cable for () {}
