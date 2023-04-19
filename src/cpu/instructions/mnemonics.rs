// For now, the main goal of this is to separate parameters
// (u8, u16 and i8) from the rest of an instruction for disassembly.
// However, it is set up in a way so that it can be extended
// to also separate registers, branch conditions etc,
// which might be useful for debugging as well.

use crate::cpu::Cpu;

/// The visual representation of an instruction.
pub struct Mnemonic(pub &'static str, pub Opd, pub Opd);

/// Represents an operand inside of a mnemonic.
pub enum Opd {
    /// No operand. Used when a mnemonic has 0 or 1 operand.
    None,
    Ptr(Ptr),
    Param(Param),
    /// Represents anything not (yet) implemented as an enum.
    Fixed(&'static str),
}

/// Represents brackets inside of a mnemonic, meaning that
/// whatever is inside the brackets should be interpreted
/// as a memory address, and we want to use the value it points to.
pub enum Ptr {
    Param(Param),
    Fixed(&'static str),
}

/// Represents a parameter of an instruction inside a mnemonic.
pub enum Param {
    U8,
    I8,
    U16,
    /// `SP+i8`, which only occurs in instruction 0xf8.
    SPI8,
}
