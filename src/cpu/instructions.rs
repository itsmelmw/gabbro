mod base;
mod bitwise;
mod helpers;
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
    pub mnemonic: &'static str,
    pub param_type: ParamType,
    pub cycles: usize,
    pub brcycles: Option<usize>,
    pub operation: fn(&mut Cpu),
}

/// An instruction set containing 256 instructions.
pub type InstrSet = [Instruction; 256];
