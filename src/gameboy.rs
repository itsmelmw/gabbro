use crate::{
    cpu::Cpu,
    peripherals::{Cable, Joypad, Lcd, Speaker},
};

#[cfg(feature = "debug")]
use crate::cpu::{
    instructions::{
        bitwise::BITWISE_PREFIX,
        debug::{
            base::BASE_INSTR_INFO, bitwise::BITWISE_INSTR_INFO, HasImmediate, Mnemonic, ParamType,
        },
    },
    registers::Regs,
};

/// Represents an emulated Game Boy.
pub struct Gameboy<L = (), S = (), J = (), C = ()>
where
    L: Lcd,
    S: Speaker,
    J: Joypad,
    C: Cable,
{
    cpu: Cpu<L, S, J, C>,
}

impl Gameboy {
    /// Creates a [`GameboyBuilder`], allowing peripherals for different input and output devices to be attached.
    pub fn builder(rom: Vec<u8>) -> GameboyBuilder {
        GameboyBuilder::new(rom)
    }
}

impl<L, S, J, C> Gameboy<L, S, J, C>
where
    L: Lcd,
    S: Speaker,
    J: Joypad,
    C: Cable,
{
    /// Runs the Game Boy emulator in an infinite loop.
    pub fn run(&mut self) {
        loop {
            self.cpu.step();
        }
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
                *BITWISE_INSTR_INFO[opcode as usize].mnemonic()
            }
            _ => {
                // Length can vary for other instructions.
                let instr = &BASE_INSTR_INFO[opcode as usize];
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

/// A builder for a [`Gameboy`], allowing peripherals for different input and output devices to be attached.
pub struct GameboyBuilder<L = (), S = (), J = (), C = ()>
where
    L: Lcd,
    S: Speaker,
    J: Joypad,
    C: Cable,
{
    rom: Vec<u8>,
    lcd: L,
    speaker: S,
    joypad: J,
    cable: C,
}

impl GameboyBuilder {
    /// Initializes a new builder for a [`Gameboy`].
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            lcd: (),
            speaker: (),
            joypad: (),
            cable: (),
        }
    }
}

impl<S, J, C> GameboyBuilder<(), S, J, C>
where
    S: Speaker,
    J: Joypad,
    C: Cable,
{
    /// Used to attach a [`Lcd`], which defines how pixels pushed to the LCD should be handled.
    pub fn lcd<L>(self, lcd: L) -> GameboyBuilder<L, S, J, C>
    where
        L: Lcd,
    {
        GameboyBuilder {
            rom: self.rom,
            lcd,
            speaker: self.speaker,
            joypad: self.joypad,
            cable: self.cable,
        }
    }
}

impl<L, J, C> GameboyBuilder<L, (), J, C>
where
    L: Lcd,
    J: Joypad,
    C: Cable,
{
    /// Used to attach a [`Speaker`], which defines how audio samples should be processed.
    pub fn speaker<S>(self, speaker: S) -> GameboyBuilder<L, S, J, C>
    where
        S: Speaker,
    {
        GameboyBuilder {
            rom: self.rom,
            lcd: self.lcd,
            speaker,
            joypad: self.joypad,
            cable: self.cable,
        }
    }
}

impl<L, S, C> GameboyBuilder<L, S, (), C>
where
    L: Lcd,
    S: Speaker,
    C: Cable,
{
    /// Used to attach a [`Joypad`], which defines when buttons are considered pressed or released.
    pub fn joypad<J>(self, joypad: J) -> GameboyBuilder<L, S, J, C>
    where
        J: Joypad,
    {
        GameboyBuilder {
            rom: self.rom,
            lcd: self.lcd,
            speaker: self.speaker,
            joypad,
            cable: self.cable,
        }
    }
}

impl<L, S, J> GameboyBuilder<L, S, J, ()>
where
    L: Lcd,
    S: Speaker,
    J: Joypad,
{
    /// Used to attach a [`Cable`], which defines how a serial transfer should be handled.
    pub fn cable<C>(self, cable: C) -> GameboyBuilder<L, S, J, C>
    where
        C: Cable,
    {
        GameboyBuilder {
            rom: self.rom,
            lcd: self.lcd,
            speaker: self.speaker,
            joypad: self.joypad,
            cable,
        }
    }
}

impl<L, S, J, C> GameboyBuilder<L, S, J, C>
where
    L: Lcd,
    S: Speaker,
    J: Joypad,
    C: Cable,
{
    /// Builds a new [`Gameboy`].
    pub fn build(self) -> Gameboy<L, S, J, C> {
        Gameboy {
            cpu: Cpu::new(self.rom, self.lcd, self.speaker, self.joypad, self.cable),
        }
    }
}
