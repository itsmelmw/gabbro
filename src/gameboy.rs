use crate::{
    cpu::Cpu,
    interfaces::{
        core::{NoJoypad, NoLcd, NoSerial},
        GameboyJoypad, GameboyLcd, GameboySerial,
    },
};

/// Represents an emulated Game Boy.
pub struct Gameboy {
    pub cpu: Cpu,
}

impl Gameboy {
    /// Initializes a new Game Boy.
    pub fn new(
        rom: Vec<u8>,
        lcd: Box<dyn GameboyLcd>,
        joypad: Box<dyn GameboyJoypad>,
        serial: Box<dyn GameboySerial>,
    ) -> Self {
        let cpu = Cpu::new(rom, lcd, joypad, serial);

        Self { cpu }
    }

    /// Runs the Game Boy emulator in an infinite loop.
    pub fn run(&mut self) {
        loop {
            self.cpu.step();
        }
    }

    /// Creates a [`GameboyBuilder`], allowing interfaces for different input and output devices to be attached.
    pub fn builder(rom: Vec<u8>) -> GameboyBuilder {
        GameboyBuilder::new(rom)
    }
}

/// A builder for a [`Gameboy`], allowing interfaces for different input and output devices to be attached.
pub struct GameboyBuilder {
    rom: Vec<u8>,
    lcd: Box<dyn GameboyLcd>,
    joypad: Box<dyn GameboyJoypad>,
    serial: Box<dyn GameboySerial>,
}

impl GameboyBuilder {
    /// Initializes a new builder for a [`Gameboy`].
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            lcd: Box::new(NoLcd),
            joypad: Box::new(NoJoypad),
            serial: Box::new(NoSerial),
        }
    }
    /// Used to attach a [`GameboyLcd`], which defines how pixels pushed to the LCD should be handled.
    pub fn attach_lcd(mut self, lcd: Box<dyn GameboyLcd>) -> Self {
        self.lcd = lcd;
        self
    }
    /// Used to attach a [`GameboyJoypad`], which defines when buttons are considered pressed or released.
    pub fn attach_joypad(mut self, joypad: Box<dyn GameboyJoypad>) -> Self {
        self.joypad = joypad;
        self
    }
    /// Used to attach a [`GameboySerial`], which defines how a serial transfer should be handled.
    pub fn attach_serial(mut self, serial: Box<dyn GameboySerial>) -> Self {
        self.serial = serial;
        self
    }
    /// Builds a new [`Gameboy`].
    pub fn build(self) -> Gameboy {
        Gameboy::new(self.rom, self.lcd, self.joypad, self.serial)
    }
}
