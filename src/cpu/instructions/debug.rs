pub mod base;
pub mod bitwise;

use std::fmt::{Display, Formatter, Result};

/// The size of the parameter expected by an instruction.
#[derive(Clone, Copy)]
pub enum ParamType {
    None,
    Byte,
    Word,
}

#[derive(Clone, Copy)]
pub enum Cycles {
    Normal(usize),
    Branch(usize, usize),
}

/// Stores information about a Game Boy instruction.
pub struct InstrInfo {
    mnemonic: Mnemonic,
    param_type: ParamType,
    #[allow(dead_code)]
    cycles: Cycles,
}

impl InstrInfo {
    pub fn mnemonic(&self) -> &Mnemonic {
        &self.mnemonic
    }
    pub fn param_type(&self) -> ParamType {
        self.param_type
    }
    #[allow(dead_code)]
    pub fn cycles(&self) -> &Cycles {
        &self.cycles
    }
}

pub trait HasImmediate {
    /// Fills in `Imm::Unknown` values with `Imm::Known(T)` based on
    /// the bytes in `bytes`. `bytes` should contain the bytes
    /// immediately following the instruction opcode, and should have
    /// the length of 0, 1 or 2 depending on the instruction length.
    fn with_immediate(self, bytes: &[u8]) -> Self;
}

// Currently only used for disassembly, but might be interesting
// to use for generalizing instruction operations. Will mean
// rewriting all instructions and part of the cpu though.
/// The visual representation of an instruction.
#[derive(Clone, Copy)]
pub enum Mnemonic {
    // Misc
    /// `NOP`
    Nop,
    /// `STOP`
    Stop,
    /// `HALT`
    Halt,
    /// `DI`
    Di,
    /// `EI`
    Ei,

    // 8-bit Loads
    /// `LD r,r`
    LdReg8Reg8(Reg8, Reg8),
    /// `LD r,(aa)`
    LdReg8Addr(Reg8, Addr),
    /// `LD (aa),r`
    LdAddrReg8(Addr, Reg8),
    /// `LD r,u8`
    LdReg8Imm(Reg8, Imm<u8>),
    /// `LD (HL),u8`
    LdHlAddrImm(Imm<u8>),

    // 16-bit Loads
    /// `LD rr,u16`
    LdReg16Imm(Reg16, Imm<u16>),
    /// `LD (u16),SP`
    LdImmAddrSp(Imm<u16>),
    /// `LD SP,HL`
    LdSpHl,
    /// `PUSH rr`
    Push(Reg16),
    /// `POP rr`
    Pop(Reg16),

    // Branch instructions
    /// `JR i8`
    Jr(Imm<i8>),
    /// `JR c,i8`
    JrCond(Cond, Imm<i8>),
    /// `JP u16`
    Jp(Imm<u16>),
    /// `JP c,u16`
    JpCond(Cond, Imm<u16>),
    /// `JP HL`
    JpHl,
    /// `CALL u16`
    Call(Imm<u16>),
    /// `CALL c,u16`
    CallCond(Cond, Imm<u16>),
    /// `RET`
    Ret,
    /// `RET c`
    RetCond(Cond),
    /// `RETI`
    Reti,
    /// `RST u8`
    Rst(Imm<u8>),

    // 8-bit Arithmetic
    /// `INC r`
    IncReg8(Reg8),
    /// `INC (HL)`
    IncHlAddr,
    /// `DEC r`
    DecReg8(Reg8),
    /// `DEC (HL),
    DecHlAddr,
    /// `ADD A,r`
    AddAReg8(Reg8),
    /// `ADD A,(HL)`
    AddAHlAddr,
    /// `ADD A,u8`
    AddAImm(Imm<u8>),
    /// `ADC A,r`
    AdcAReg8(Reg8),
    /// `ADC A,(HL)`
    AdcAHlAddr,
    /// `ADC A,u8`
    AdcAImm(Imm<u8>),
    /// `SUB A,r`
    SubAReg8(Reg8),
    /// `SUB A,(HL)`
    SubAHlAddr,
    /// `SUB A,u8`
    SubAImm(Imm<u8>),
    /// `SBC A,r`
    SbcAReg8(Reg8),
    /// `SBC A,(HL)`
    SbcAHlAddr,
    /// `SBC A,u8`
    SbcAImm(Imm<u8>),
    /// `AND A,r`
    AndAReg8(Reg8),
    /// `AND A,(HL)`
    AndAHlAddr,
    /// `AND A,u8`
    AndAImm(Imm<u8>),
    /// `XOR A,r`
    XorAReg8(Reg8),
    /// `XOR A,(HL)`
    XorAHlAddr,
    /// `XOR A,u8`
    XorAImm(Imm<u8>),
    /// `OR A,r`
    OrAReg8(Reg8),
    /// `OR A,(HL)`
    OrAHlAddr,
    /// `OR A,u8`
    OrAImm(Imm<u8>),
    /// `CP A,r`
    CpAReg8(Reg8),
    /// `CP A,(HL)`
    CpAHlAddr,
    /// `CP A,u8`
    CpAImm(Imm<u8>),
    /// `DAA`
    Daa,
    /// `CPL`
    Cpl,
    /// `SCF`
    Scf,
    /// `CCF`
    Ccf,

    // 8-bit Bitwise Arithmetic
    /// `RLCA`
    Rlca,
    /// `RRCA`
    Rrca,
    /// `RLA`
    Rla,
    /// `RRA`
    Rra,
    /// `RLC r`
    RlcReg8(Reg8),
    /// `RLC (HL)`
    RlcHlAddr,
    /// `RRC r`
    RrcReg8(Reg8),
    /// `RRC (HL)`
    RrcHlAddr,
    /// `RL r`
    RlReg8(Reg8),
    /// `RL (HL)`
    RlHlAddr,
    /// `RR r`
    RrReg8(Reg8),
    /// `RR (HL)`
    RrHlAddr,
    /// `SLA r`
    SlaReg8(Reg8),
    /// `SLA (HL)`
    SlaHlAddr,
    /// `SRA r`
    SraReg8(Reg8),
    /// `SRA (HL)`
    SraHlAddr,
    /// `SWAP r`
    SwapReg8(Reg8),
    /// `SWAP (HL)`
    SwapHlAddr,
    /// `SRL r`
    SrlReg8(Reg8),
    /// `SRL (HL)`
    SrlHlAddr,
    /// `BIT b,r`
    BitReg8(Imm<u8>, Reg8),
    /// `BIT b,(HL)`
    BitHlAddr(Imm<u8>),
    /// `RES b,r`
    ResReg8(Imm<u8>, Reg8),
    /// `RES b,(HL)`
    ResHlAddr(Imm<u8>),
    /// `SET b,r`
    SetReg8(Imm<u8>, Reg8),
    /// `SET b,(HL)`
    SetHlAddr(Imm<u8>),

    // 16-bit Arithmetic
    /// `INC rr`
    IncReg16(Reg16),
    /// `DEC rr`
    DecReg16(Reg16),
    /// `ADD HL,rr`
    AddHlReg16(Reg16),
    /// `ADD SP,i8`
    AddSpImm(Imm<i8>),
    /// `LD HL,SP+i8`
    LdHlAddSpImm(Imm<i8>),

    /// Invalid instructions that cause a crash
    Invalid,
}

#[derive(Clone, Copy)]
pub enum Addr {
    /// `(FF00+u8)`
    Imm8(Imm<u8>),
    /// `(u16)`
    Imm16(Imm<u16>),
    /// `(FF00+r)`
    Reg8(Reg8),
    /// `(rr)`
    Reg16(Reg16),
    /// `(rr+)`
    Reg16Inc(Reg16),
    /// `(rr-)`
    Reg16Dec(Reg16),
}

#[derive(Clone, Copy)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Clone, Copy)]
pub enum Cond {
    /// Condition that is satisfied if the Zero flag is set.
    Z,
    /// Condition that is satisfied if the Zero flag is not set.
    NZ,
    /// Condition that is satisfied if the Carry flag is set.
    C,
    /// Condition that is satisfied if the Carry flag is not set.
    NC,
}

#[derive(Clone, Copy)]
pub enum Imm<T> {
    Known(T),
    Unknown,
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Nop => "NOP".to_string(),
                Self::Stop => "STOP".to_string(),
                Self::Halt => "HALT".to_string(),
                Self::Di => "DI".to_string(),
                Self::Ei => "EI".to_string(),
                Self::LdReg8Reg8(r1, r2) => format!("{:04} {},{}", "LD", r1, r2),
                Self::LdReg8Addr(r, a) => format!("{:04} {},{}", "LD", r, a),
                Self::LdAddrReg8(a, r) => format!("{:04} {},{}", "LD", a, r),
                Self::LdReg8Imm(r, i) => format!("{:04} {},{}", "LD", r, i),
                Self::LdHlAddrImm(i) => format!("{:04} {},{}", "LD", Addr::Reg16(Reg16::HL), i),
                Self::LdReg16Imm(r, i) => format!("{:04} {},{}", "LD", r, i),
                Self::LdImmAddrSp(i) => format!("{:04} {},{}", "LD", Addr::Imm16(*i), Reg16::SP),
                Self::LdSpHl => format!("{:04} {},{}", "LD", Reg16::SP, Reg16::HL),
                Self::Push(r) => format!("{:04} {}", "PUSH", r),
                Self::Pop(r) => format!("{:04} {}", "POP", r),
                Self::Jr(i) => format!("{:04} {}", "JR", i),
                Self::JrCond(c, i) => format!("{:04} {},{}", "JR", c, i),
                Self::Jp(i) => format!("{:04} {}", "JP", i),
                Self::JpCond(c, i) => format!("{:04} {},{}", "JP", c, i),
                Self::JpHl => format!("{:04} {}", "JP", Reg16::HL),
                Self::Call(i) => format!("{:04} {}", "CALL", i),
                Self::CallCond(c, i) => format!("{:04} {},{}", "CALL", c, i),
                Self::Ret => "RET".to_string(),
                Self::RetCond(c) => format!("{:04} {}", "RET", c),
                Self::Reti => "RETI".to_string(),
                Self::Rst(i) => format!("{:04} {}h", "RST", i),
                Self::IncReg8(r) => format!("{:04} {}", "INC", r),
                Self::IncHlAddr => format!("{:04} {}", "INC", Addr::Reg16(Reg16::HL)),
                Self::DecReg8(r) => format!("{:04} {}", "DEC", r),
                Self::DecHlAddr => format!("{:04} {}", "DEC", Addr::Reg16(Reg16::HL)),
                Self::AddAReg8(r) => format!("{:04} {},{}", "ADD", Reg8::A, r),
                Self::AddAHlAddr => format!("{:04} {},{}", "ADD", Reg8::A, Addr::Reg16(Reg16::HL)),
                Self::AddAImm(i) => format!("{:04} {},{}", "ADD", Reg8::A, i),
                Self::AdcAReg8(r) => format!("{:04} {},{}", "ADC", Reg8::A, r),
                Self::AdcAHlAddr => format!("{:04} {},{}", "ADC", Reg8::A, Addr::Reg16(Reg16::HL)),
                Self::AdcAImm(i) => format!("{:04} {},{}", "ADC", Reg8::A, i),
                Self::SubAReg8(r) => format!("{:04} {},{}", "SUB", Reg8::A, r),
                Self::SubAHlAddr => format!("{:04} {},{}", "SUB", Reg8::A, Addr::Reg16(Reg16::HL)),
                Self::SubAImm(i) => format!("{:04} {},{}", "SUB", Reg8::A, i),
                Self::SbcAReg8(r) => format!("{:04} {},{}", "SBC", Reg8::A, r),
                Self::SbcAHlAddr => format!("{:04} {},{}", "SBC", Reg8::A, Addr::Reg16(Reg16::HL)),
                Self::SbcAImm(i) => format!("{:04} {},{}", "SBC", Reg8::A, i),
                Self::AndAReg8(r) => format!("{:04} {},{}", "AND", Reg8::A, r),
                Self::AndAHlAddr => format!("{:04} {},{}", "AND", Reg8::A, Addr::Reg16(Reg16::HL)),
                Self::AndAImm(i) => format!("{:04} {},{}", "AND", Reg8::A, i),
                Self::XorAReg8(r) => format!("{:04} {},{}", "XOR", Reg8::A, r),
                Self::XorAHlAddr => format!("{:04} {},{}", "XOR", Reg8::A, Addr::Reg16(Reg16::HL)),
                Self::XorAImm(i) => format!("{:04} {},{}", "XOR", Reg8::A, i),
                Self::OrAReg8(r) => format!("{:04} {},{}", "OR", Reg8::A, r),
                Self::OrAHlAddr => format!("{:04} {},{}", "OR", Reg8::A, Addr::Reg16(Reg16::HL)),
                Self::OrAImm(i) => format!("{:04} {},{}", "OR", Reg8::A, i),
                Self::CpAReg8(r) => format!("{:04} {},{}", "CP", Reg8::A, r),
                Self::CpAHlAddr => format!("{:04} {},{}", "CP", Reg8::A, Addr::Reg16(Reg16::HL)),
                Self::CpAImm(i) => format!("{:04} {},{}", "CP", Reg8::A, i),
                Self::Daa => "DAA".to_string(),
                Self::Cpl => "CPL".to_string(),
                Self::Scf => "SCF".to_string(),
                Self::Ccf => "CCF".to_string(),
                Self::Rlca => "RLCA".to_string(),
                Self::Rrca => "RRCA".to_string(),
                Self::Rla => "RLA".to_string(),
                Self::Rra => "RRA".to_string(),
                Self::RlcReg8(r) => format!("{:04} {}", "RLC", r),
                Self::RlcHlAddr => format!("{:04} {}", "RLC", Addr::Reg16(Reg16::HL)),
                Self::RrcReg8(r) => format!("{:04} {}", "RRC", r),
                Self::RrcHlAddr => format!("{:04} {}", "RRC", Addr::Reg16(Reg16::HL)),
                Self::RlReg8(r) => format!("{:04} {}", "RL", r),
                Self::RlHlAddr => format!("{:04} {}", "RL", Addr::Reg16(Reg16::HL)),
                Self::RrReg8(r) => format!("{:04} {}", "RR", r),
                Self::RrHlAddr => format!("{:04} {}", "RR", Addr::Reg16(Reg16::HL)),
                Self::SlaReg8(r) => format!("{:04} {}", "SLA", r),
                Self::SlaHlAddr => format!("{:04} {}", "SLA", Addr::Reg16(Reg16::HL)),
                Self::SraReg8(r) => format!("{:04} {}", "SRA", r),
                Self::SraHlAddr => format!("{:04} {}", "SRA", Addr::Reg16(Reg16::HL)),
                Self::SwapReg8(r) => format!("{:04} {}", "SWAP", r),
                Self::SwapHlAddr => format!("{:04} {}", "SWAP", Addr::Reg16(Reg16::HL)),
                Self::SrlReg8(r) => format!("{:04} {}", "SRL", r),
                Self::SrlHlAddr => format!("{:04} {}", "SRL", Addr::Reg16(Reg16::HL)),
                Self::BitReg8(i, r) => format!("{:04} {},{}", "BIT", i, r),
                Self::BitHlAddr(i) => format!("{:04} {},{}", "BIT", i, Addr::Reg16(Reg16::HL)),
                Self::ResReg8(i, r) => format!("{:04} {},{}", "RES", i, r),
                Self::ResHlAddr(i) => format!("{:04} {},{}", "RES", i, Addr::Reg16(Reg16::HL)),
                Self::SetReg8(i, r) => format!("{:04} {},{}", "SET", i, r),
                Self::SetHlAddr(i) => format!("{:04} {},{}", "SET", i, Addr::Reg16(Reg16::HL)),
                Self::IncReg16(r) => format!("{:04} {}", "INC", r),
                Self::DecReg16(r) => format!("{:04} {}", "DEC", r),
                Self::AddHlReg16(r) => format!("{:04} {},{}", "ADD", Reg16::HL, r),
                Self::AddSpImm(i) => format!("{:04} {},{}", "ADD", Reg16::SP, i),
                Self::LdHlAddSpImm(i) => format!("{:04} {},{}+{}", "LD", Reg16::HL, Reg16::SP, i),
                Self::Invalid => "INVALID".to_string(),
            }
        )
    }
}

impl HasImmediate for Mnemonic {
    fn with_immediate(self, bytes: &[u8]) -> Self {
        match self {
            Self::LdReg8Addr(r, a) => Self::LdReg8Addr(r, a.with_immediate(bytes)),
            Self::LdAddrReg8(a, r) => Self::LdAddrReg8(a.with_immediate(bytes), r),
            Self::LdReg8Imm(r, i) => Self::LdReg8Imm(r, i.with_immediate(bytes)),
            Self::LdHlAddrImm(i) => Self::LdHlAddrImm(i.with_immediate(bytes)),
            Self::LdReg16Imm(r, i) => Self::LdReg16Imm(r, i.with_immediate(bytes)),
            Self::LdImmAddrSp(i) => Self::LdImmAddrSp(i.with_immediate(bytes)),
            Self::Jr(i) => Self::Jr(i.with_immediate(bytes)),
            Self::JrCond(c, i) => Self::JrCond(c, i.with_immediate(bytes)),
            Self::Jp(i) => Self::Jp(i.with_immediate(bytes)),
            Self::JpCond(c, i) => Self::JpCond(c, i.with_immediate(bytes)),
            Self::Call(i) => Self::Call(i.with_immediate(bytes)),
            Self::CallCond(c, i) => Self::CallCond(c, i.with_immediate(bytes)),
            Self::AddAImm(i) => Self::AddAImm(i.with_immediate(bytes)),
            Self::AdcAImm(i) => Self::AdcAImm(i.with_immediate(bytes)),
            Self::SubAImm(i) => Self::SubAImm(i.with_immediate(bytes)),
            Self::SbcAImm(i) => Self::SbcAImm(i.with_immediate(bytes)),
            Self::AndAImm(i) => Self::AndAImm(i.with_immediate(bytes)),
            Self::XorAImm(i) => Self::XorAImm(i.with_immediate(bytes)),
            Self::OrAImm(i) => Self::OrAImm(i.with_immediate(bytes)),
            Self::CpAImm(i) => Self::CpAImm(i.with_immediate(bytes)),
            Self::AddSpImm(i) => Self::AddSpImm(i.with_immediate(bytes)),
            Self::LdHlAddSpImm(i) => Self::LdHlAddSpImm(i.with_immediate(bytes)),
            _ => self,
        }
    }
}

impl Display for Addr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({})",
            match self {
                Self::Imm8(i) => format!("ff00+{}", i),
                Self::Imm16(i) => i.to_string(),
                Self::Reg8(r) => format!("ff00+{}", r),
                Self::Reg16(r) => r.to_string(),
                Self::Reg16Inc(r) => format!("{}+", r),
                Self::Reg16Dec(r) => format!("{}-", r),
            }
        )
    }
}

impl HasImmediate for Addr {
    fn with_immediate(self, bytes: &[u8]) -> Self {
        match self {
            Self::Imm8(i) => Self::Imm8(i.with_immediate(bytes)),
            Self::Imm16(i) => Self::Imm16(i.with_immediate(bytes)),
            _ => self,
        }
    }
}

impl Display for Reg8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::A => "A",
                Self::B => "B",
                Self::C => "C",
                Self::D => "D",
                Self::E => "E",
                Self::H => "H",
                Self::L => "L",
            }
        )
    }
}

impl Display for Reg16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::AF => "AF",
                Self::BC => "BC",
                Self::DE => "DE",
                Self::HL => "HL",
                Self::SP => "SP",
            }
        )
    }
}

impl Display for Cond {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Z => "Z",
                Self::NZ => "NZ",
                Self::C => "C",
                Self::NC => "NC",
            }
        )
    }
}

impl Display for Imm<u16> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Known(i) => format!("{:04x}", i),
                Self::Unknown => "u16".to_string(),
            }
        )
    }
}

impl HasImmediate for Imm<u16> {
    fn with_immediate(self, bytes: &[u8]) -> Self {
        Self::Known((bytes[0] as u16) | (bytes[1] as u16) << 8)
    }
}

impl Display for Imm<i8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Known(i) => format!("{:02x}", i),
                Self::Unknown => "i8".to_string(),
            }
        )
    }
}

impl HasImmediate for Imm<i8> {
    fn with_immediate(self, bytes: &[u8]) -> Self {
        Self::Known(bytes[0] as i8)
    }
}

impl Display for Imm<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::Known(i) => format!("{:02x}", i),
                Self::Unknown => "u8".to_string(),
            }
        )
    }
}

impl HasImmediate for Imm<u8> {
    fn with_immediate(self, bytes: &[u8]) -> Self {
        Self::Known(bytes[0])
    }
}
