mod base;
mod bitwise;
mod disasm;
mod helpers;

use self::disasm::{Disasm, Mnemonic};
use super::Cpu;
pub use base::BASE_INSTRS;
pub use bitwise::BITWISE_INSTRS;

/// The size of the parameter expected by an instruction.
pub enum ParamType {
    None,
    Byte,
    Word,
}

/// Stores information about a Game Boy instruction.
pub struct Instruction {
    mnemonic: Mnemonic,
    param_type: ParamType,
    cycles: usize,
    branch_cycles: Option<usize>,
    operation: fn(&mut Cpu),
}

/// An instruction set containing 256 instructions.
pub type InstrSet = [Instruction; 256];

impl Instruction {
    pub fn length(&self) -> usize {
        match self.param_type {
            ParamType::None => 1,
            ParamType::Byte => 2,
            ParamType::Word => 3,
        }
    }
    pub fn machine_cycles(&self) -> usize {
        self.cycles
    }
    pub fn clock_cycles(&self) -> usize {
        self.cycles * 4
    }
    pub fn machine_branch_cycles(&self) -> Option<usize> {
        self.branch_cycles
    }
    pub fn clock_branch_cycles(&self) -> Option<usize> {
        self.branch_cycles.map(|cycles| cycles * 4)
    }
    pub fn execute(&self, cpu: &mut Cpu) {
        (self.operation)(cpu)
    }
    pub fn disasm_static(&self) -> String {
        self.mnemonic.disasm_static()
    }
    pub fn disasm(&self, bytes: &[u8]) -> String {
        self.mnemonic.disasm(bytes)
    }
}
