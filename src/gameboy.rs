use crate::{
    cpu::{
        Cpu, HasImmediate, Mnemonic, ParamType, Regs, BASE_INSTRS, BITWISE_INSTRS, BITWISE_PREFIX,
    },
    interfaces::{
        core::{NoJoypad, NoLcd, NoSerial},
        GameboyJoypad, GameboyLcd, GameboySerial,
    },
};

/// Represents an emulated Game Boy.
pub struct Gameboy {
    cpu: Cpu,
}

impl Gameboy {
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

    /// Makes the Game Boy emulator execute a single instruction,
    /// however many cycles that may take.
    #[cfg(feature = "debug")]
    pub fn step(&mut self) {
        self.cpu.step();
    }

    /// Read the bytes at `addr` from the currently mapped memory.
    #[cfg(feature = "debug")]
    pub fn read_mem(&self, addr: u16) -> u8 {
        self.cpu.bus().read(addr)
    }

    /// Retrieve the current CPU register values from the CPU.
    #[cfg(feature = "debug")]
    pub fn regs(&self) -> &Regs {
        self.cpu.regs()
    }

    /// Disassembles the instruction at `addr`. Returns both the bytes
    /// corresponding to the instruction, and the mnemonic.
    #[cfg(feature = "debug")]
    pub fn disasm_at(&self, addr: u16) -> (Vec<u8>, Mnemonic) {
        let mut bytes = Vec::with_capacity(3);

        let opcode = self.cpu.bus().read(addr);
        bytes.push(opcode);
        let mnemonic = match opcode {
            BITWISE_PREFIX => {
                // All bitwise instructions have length 2,
                // and no bytes containing immediates.
                let opcode = self.cpu.bus().read(addr + 1);
                bytes.push(opcode);
                *BITWISE_INSTRS[opcode as usize].mnemonic()
            }
            _ => {
                // Length can vary for other instructions.
                let instr = &BASE_INSTRS[opcode as usize];
                let imm = match instr.param_type() {
                    ParamType::None => vec![],
                    ParamType::Byte => {
                        let byte = self.cpu.bus().read(addr + 1);
                        bytes.push(byte);
                        vec![byte]
                    }
                    ParamType::Word => {
                        let byte1 = self.cpu.bus().read(addr + 1);
                        let byte2 = self.cpu.bus().read(addr + 2);
                        bytes.push(byte1);
                        bytes.push(byte2);
                        vec![byte1, byte2]
                    }
                };
                instr.mnemonic().with_immediate(&imm)
            }
        };
        (bytes, mnemonic)
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
        Gameboy {
            cpu: Cpu::new(self.rom, self.lcd, self.joypad, self.serial),
        }
    }
}
