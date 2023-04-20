pub trait Disasm {
    /// Returns the disassembly of an instruction without filled-in parameters.
    fn disasm_static(&self) -> String;
    /// Returns the disassembly of an instruction using the bytes in memory to fill parameter values.
    /// May panic if the length of `bytes` is not equal to the length of the instruction.
    fn disasm(&self, bytes: &[u8]) -> String;
}

/// The visual representation of an instruction.
pub struct Mnemonic(pub &'static str, pub Option<Opd>, pub Option<Opd>);

/// Represents an operand inside of a mnemonic.
pub enum Opd {
    /// A variable parameter specified by the byte(s) following the opcode.
    Param(Param),
    /// An 8 or 16 bit register.
    Reg(Reg),
    /// A fixed number.
    Num(u16),
    /// A conditional check on a flag.
    Cond(Cond),
    /// A pointer derefence. Use the given value as address to read from.
    Addr(Addr),
    /// Used only for instruction 0xf8, which uses `SP+i8` as an operand.
    Sum(Reg, Param),
}

pub enum Addr {
    FromParam(Param),
    FromReg(Reg),
    FromRegInc(Reg),
    FromRegDec(Reg),
}

pub enum Reg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
}

/// Represents a condition in conditional branch instructions.
pub enum Cond {
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

/// Represents a parameter of an instruction.
pub enum Param {
    U8,
    I8,
    U16,
}

impl Disasm for Mnemonic {
    fn disasm_static(&self) -> String {
        match self {
            Mnemonic(s, None, None) => format!("{}", s),
            Mnemonic(s, Some(opd), None) => format!("{} {}", s, opd.disasm_static()),
            Mnemonic(s, Some(opd1), Some(opd2)) => {
                format!("{} {},{}", s, opd1.disasm_static(), opd2.disasm_static())
            }
            _ => unreachable!(),
        }
    }
    fn disasm(&self, bytes: &[u8]) -> String {
        match self {
            Mnemonic(s, None, None) => format!("{}", s),
            Mnemonic(s, Some(opd), None) => format!("{} {}", s, opd.disasm(bytes)),
            Mnemonic(s, Some(opd1), Some(opd2)) => {
                format!("{} {},{}", s, opd1.disasm(bytes), opd2.disasm(bytes))
            }
            _ => unreachable!(),
        }
    }
}

impl Disasm for Opd {
    fn disasm_static(&self) -> String {
        match self {
            Self::Param(param) => param.disasm_static(),
            Self::Reg(reg) => reg.disasm_static(),
            Self::Num(num) => format!("{:x}", num),
            Self::Cond(cond) => cond.disasm_static(),
            Self::Addr(addr) => addr.disasm_static(),
            Self::Sum(reg, param) => format!("{}+{}", reg.disasm_static(), param.disasm_static()),
        }
    }
    fn disasm(&self, bytes: &[u8]) -> String {
        match self {
            Self::Param(param) => param.disasm(bytes),
            Self::Reg(reg) => reg.disasm(bytes),
            Self::Num(num) => format!("{:x}", num),
            Self::Cond(cond) => cond.disasm(bytes),
            Self::Addr(addr) => addr.disasm(bytes),
            Self::Sum(reg, param) => format!("{}+{}", reg.disasm(bytes), param.disasm(bytes)),
        }
    }
}

impl Disasm for Addr {
    fn disasm_static(&self) -> String {
        match self {
            Self::FromParam(param) => format!("({})", param.disasm_static()),
            Self::FromReg(reg) if !reg.is_16_bit() => format!("(FF00+{})", reg.disasm_static()),
            Self::FromReg(reg) => format!("({})", reg.disasm_static()),
            Self::FromRegInc(reg) => format!("({}+)", reg.disasm_static()),
            Self::FromRegDec(reg) => format!("({}-)", reg.disasm_static()),
        }
    }
    fn disasm(&self, bytes: &[u8]) -> String {
        match self {
            Self::FromParam(param) => format!("({})", param.disasm(bytes)),
            Self::FromReg(reg) => format!("({})", reg.disasm(bytes)),
            Self::FromRegInc(reg) => format!("({}+)", reg.disasm(bytes)),
            Self::FromRegDec(reg) => format!("({}-)", reg.disasm(bytes)),
        }
    }
}

impl Disasm for Reg {
    fn disasm_static(&self) -> String {
        match self {
            Self::A => String::from("A"),
            Self::B => String::from("B"),
            Self::C => String::from("C"),
            Self::D => String::from("D"),
            Self::E => String::from("E"),
            Self::H => String::from("H"),
            Self::L => String::from("L"),
            Self::AF => String::from("AF"),
            Self::BC => String::from("BC"),
            Self::DE => String::from("DE"),
            Self::HL => String::from("HL"),
            Self::SP => String::from("SP"),
        }
    }
    fn disasm(&self, _bytes: &[u8]) -> String {
        self.disasm_static()
    }
}

impl Reg {
    pub fn is_16_bit(&self) -> bool {
        match self {
            Self::AF | Self::BC | Self::DE | Self::HL | Self::SP => true,
            _ => false,
        }
    }
}

impl Disasm for Cond {
    fn disasm_static(&self) -> String {
        match self {
            Self::Zero => String::from("Z"),
            Self::NotZero => String::from("NZ"),
            Self::Carry => String::from("C"),
            Self::NotCarry => String::from("NC"),
        }
    }
    fn disasm(&self, _bytes: &[u8]) -> String {
        self.disasm_static()
    }
}

impl Disasm for Param {
    fn disasm_static(&self) -> String {
        match self {
            Self::U8 => String::from("u8"),
            Self::I8 => String::from("i8"),
            Self::U16 => String::from("u16"),
        }
    }
    fn disasm(&self, bytes: &[u8]) -> String {
        match self {
            Self::U8 | Self::I8 => format!("{:#02x}", bytes[1]),
            Self::U16 => format!("{:#04x}", (bytes[2] as u16) | (bytes[1] as u16) << 8),
        }
    }
}
