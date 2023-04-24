mod base;
mod bitwise;
mod helpers;
mod mnemonic;

pub use self::mnemonic::{HasImmediate, Mnemonic};
use super::Cpu;
pub use base::BASE_INSTRS;
pub use bitwise::{BITWISE_INSTRS, BITWISE_PREFIX};

/// The size of the parameter expected by an instruction.
#[derive(Clone, Copy)]
pub enum ParamType {
    None,
    Byte,
    Word,
}

/// Stores information about a Game Boy instruction.
pub struct Instruction {
    mnemonic: Mnemonic,
    param_type: ParamType,
    _cycles: usize,
    _branch_cycles: Option<usize>,
    operation: fn(&mut Cpu),
}

/// An instruction set containing 256 instructions.
pub type InstrSet = [Instruction; 256];

impl Instruction {
    pub fn mnemonic(&self) -> &Mnemonic {
        &self.mnemonic
    }
    pub fn param_type(&self) -> ParamType {
        self.param_type
    }
    pub(super) fn execute(&self, cpu: &mut Cpu) {
        (self.operation)(cpu)
    }
}
