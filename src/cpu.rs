pub mod instructions;
pub mod interrupts;
pub mod registers;
use crate::{
    bus::Bus,
    cpu::{instructions::bitwise::BITWISE_PREFIX, registers::Regs},
    peripherals::{Cable, Joypad, Lcd, Speaker},
};

/// State of the Interrupt Master Enable (IME).
/// - Disabled: All interrupts are disabled.
/// - Enabling: Interrupts will be enabled next cycle.
/// - Enabled:  Interrupts are enabled according to the IE register.
#[derive(PartialEq)]
enum ImeState {
    Disabled,
    Enabling,
    Enabled,
}

/// Emulates the Game Boy CPU.
pub struct Cpu<L, S, J, C>
where
    L: Lcd,
    S: Speaker,
    J: Joypad,
    C: Cable,
{
    bus: Bus<L, S, J, C>,
    regs: Regs,
    ime: ImeState,
    halted: bool,
}

impl<L, S, J, C> Cpu<L, S, J, C>
where
    L: Lcd,
    S: Speaker,
    J: Joypad,
    C: Cable,
{
    /// Initializes a new CPU.
    pub(crate) fn new(rom: Vec<u8>, lcd: L, speaker: S, joypad: J, cable: C) -> Self {
        Self {
            bus: Bus::new(rom, lcd, speaker, joypad, cable),
            regs: Regs::new(),
            ime: ImeState::Enabled,
            halted: false,
        }
    }

    /// Fetches and executes one instruction, and checks for interrupts.
    pub(crate) fn step(&mut self) {
        if self.ime == ImeState::Enabling {
            self.ime = ImeState::Enabled;
        }

        if self.halted {
            self.cycle();
            if self.bus.interrupts.pending() {
                log::debug!("CPU: Unhalted");
                self.halted = false;
            }
        } else {
            self.execute_next();
        }

        if self.ime == ImeState::Enabled {
            // Handle an interrupt if there is any.
            if let Some(addr) = self.bus.interrupts.step() {
                self.handle_interrupt(addr);
            }
        }
    }

    /// Executes the instruction currently at `(PC)`.
    fn execute_next(&mut self) {
        let opcode = self.fetch_byte();
        match opcode {
            BITWISE_PREFIX => {
                let opcode = self.fetch_byte();
                self.execute_bitwise(opcode);
            }
            _ => self.execute_base(opcode),
        }
    }

    /// Handles an interrupt. Takes 5 machine cycles.
    fn handle_interrupt(&mut self, addr: u16) {
        self.ime = ImeState::Disabled;
        self.cycle();
        self.cycle();
        self.stack_push(self.regs.pc());
        self.cycle();
        self.regs.set_pc(addr);
    }

    /// Emulates a machine cycle for all other parts of the Game Boy.
    /// Called every time the CPU reads/writes a byte from/to memory.
    /// Also called during some instructions if they take an extra internal cycle,
    /// like for branch instructions and 16-bit arithmetic.
    pub(crate) fn cycle(&mut self) {
        self.bus.io_step();
    }

    /// Reads the byte at `addr`. Takes a machine cycle.
    pub(crate) fn read_byte(&mut self, addr: u16) -> u8 {
        self.cycle();
        self.bus.read(addr)
    }

    /// Reads two bytes at `addr` and `addr + 1`. Takes two machine cycles.
    #[allow(dead_code)]
    pub(crate) fn read_word(&mut self, addr: u16) -> u16 {
        (self.read_byte(addr + 1) as u16) | (self.read_byte(addr) as u16) << 8
    }

    /// Writes `val` to `addr`. Takes a machine cycle.
    pub(crate) fn write_byte(&mut self, addr: u16, val: u8) {
        self.cycle();
        self.bus.write(addr, val);
    }

    /// Writes `val` to `addr` and `addr + 1`. Takes two machine cycles.
    pub(crate) fn write_word(&mut self, addr: u16, val: u16) {
        self.write_byte(addr, (val & 0x00ff) as u8);
        self.write_byte(addr + 1, (val >> 8) as u8);
    }

    /// Fetches the byte at `(PC)`, and increments `PC`. Takes a machine cycle.
    pub(crate) fn fetch_byte(&mut self) -> u8 {
        let addr = self.regs.pc();
        self.regs.inc_pc();
        self.read_byte(addr)
    }

    /// Fetches two bytes at `(PC)`, and increments `PC` twice. Takes two machine cycles.
    pub(crate) fn fetch_word(&mut self) -> u16 {
        (self.fetch_byte() as u16) | (self.fetch_byte() as u16) << 8
    }

    /// Pushes `val` to `(SP)` and `(SP - 1)`, and decrements `SP` twice. Takes two machine cycles.
    pub(crate) fn stack_push(&mut self, val: u16) {
        self.stack_push_byte((val >> 8) as u8);
        self.stack_push_byte((val & 0xff) as u8);
    }

    /// Pops two bytes at `(SP)` and `(SP + 1)`, and increments `SP` twice. Takes two machine cycles.
    pub(crate) fn stack_pop(&mut self) -> u16 {
        (self.stack_pop_byte() as u16) | (self.stack_pop_byte() as u16) << 8
    }

    /// Pushes `val` to `(SP)`, and decrements `SP`. Takes a machine cycle.
    fn stack_push_byte(&mut self, val: u8) {
        self.regs.dec_sp();
        let addr = self.regs.sp();
        self.write_byte(addr, val);
    }

    /// Pops a byte at `(SP)`, and increments `SP`. Takes a machine cycle.
    fn stack_pop_byte(&mut self) -> u8 {
        let addr = self.regs.sp();
        self.regs.inc_sp();
        self.read_byte(addr)
    }

    #[cfg(feature = "debug")]
    pub(crate) fn regs(&self) -> &Regs {
        &self.regs
    }

    #[cfg(feature = "debug")]
    pub(crate) fn bus(&self) -> &Bus<L, S, J, C> {
        &self.bus
    }
}
