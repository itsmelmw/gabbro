use crate::cpu::instructions::debug::{
    Addr, Cond, Cycles, Imm, InstrInfo, Mnemonic, ParamType, Reg16, Reg8,
};

pub const BASE_INSTR_INFO: [InstrInfo; 256] = [
    // 0x00
    InstrInfo {
        mnemonic: Mnemonic::Nop,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x01
    InstrInfo {
        mnemonic: Mnemonic::LdReg16Imm(Reg16::BC, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Normal(3),
    },
    // 0x02
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::BC), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x03
    InstrInfo {
        mnemonic: Mnemonic::IncReg16(Reg16::BC),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x04
    InstrInfo {
        mnemonic: Mnemonic::IncReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x05
    InstrInfo {
        mnemonic: Mnemonic::DecReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x06
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::B, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0x07
    InstrInfo {
        mnemonic: Mnemonic::Rlca,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x08
    InstrInfo {
        mnemonic: Mnemonic::LdImmAddrSp(Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Normal(5),
    },
    // 0x09
    InstrInfo {
        mnemonic: Mnemonic::AddHlReg16(Reg16::BC),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x0a
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg16(Reg16::BC)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x0b
    InstrInfo {
        mnemonic: Mnemonic::DecReg16(Reg16::BC),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x0c
    InstrInfo {
        mnemonic: Mnemonic::IncReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x0d
    InstrInfo {
        mnemonic: Mnemonic::DecReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x0e
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::C, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0x0f
    InstrInfo {
        mnemonic: Mnemonic::Rrca,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x10
    InstrInfo {
        mnemonic: Mnemonic::Stop,
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(1),
    },
    // 0x11
    InstrInfo {
        mnemonic: Mnemonic::LdReg16Imm(Reg16::DE, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Normal(3),
    },
    // 0x12
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::DE), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x13
    InstrInfo {
        mnemonic: Mnemonic::IncReg16(Reg16::DE),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x14
    InstrInfo {
        mnemonic: Mnemonic::IncReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x15
    InstrInfo {
        mnemonic: Mnemonic::DecReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x16
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::D, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0x17
    InstrInfo {
        mnemonic: Mnemonic::Rla,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x18
    InstrInfo {
        mnemonic: Mnemonic::Jr(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(3),
    },
    // 0x19
    InstrInfo {
        mnemonic: Mnemonic::AddHlReg16(Reg16::DE),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x1a
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg16(Reg16::DE)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x1b
    InstrInfo {
        mnemonic: Mnemonic::DecReg16(Reg16::DE),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x1c
    InstrInfo {
        mnemonic: Mnemonic::IncReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x1d
    InstrInfo {
        mnemonic: Mnemonic::DecReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x1e
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::E, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0x1f
    InstrInfo {
        mnemonic: Mnemonic::Rra,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x20
    InstrInfo {
        mnemonic: Mnemonic::JrCond(Cond::NZ, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Branch(2, 3),
    },
    // 0x21
    InstrInfo {
        mnemonic: Mnemonic::LdReg16Imm(Reg16::HL, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Normal(3),
    },
    // 0x22
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16Inc(Reg16::HL), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x23
    InstrInfo {
        mnemonic: Mnemonic::IncReg16(Reg16::HL),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x24
    InstrInfo {
        mnemonic: Mnemonic::IncReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x25
    InstrInfo {
        mnemonic: Mnemonic::DecReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x26
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::H, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0x27
    InstrInfo {
        mnemonic: Mnemonic::Daa,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x28
    InstrInfo {
        mnemonic: Mnemonic::JrCond(Cond::Z, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Branch(2, 3),
    },
    // 0x29
    InstrInfo {
        mnemonic: Mnemonic::AddHlReg16(Reg16::HL),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x2a
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg16Inc(Reg16::HL)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x2b
    InstrInfo {
        mnemonic: Mnemonic::DecReg16(Reg16::HL),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x2c
    InstrInfo {
        mnemonic: Mnemonic::IncReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x2d
    InstrInfo {
        mnemonic: Mnemonic::DecReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x2e
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::L, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0x2f
    InstrInfo {
        mnemonic: Mnemonic::Cpl,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x30
    InstrInfo {
        mnemonic: Mnemonic::JrCond(Cond::NC, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Branch(2, 3),
    },
    // 0x31
    InstrInfo {
        mnemonic: Mnemonic::LdReg16Imm(Reg16::SP, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Normal(3),
    },
    // 0x32
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16Dec(Reg16::HL), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x33
    InstrInfo {
        mnemonic: Mnemonic::IncReg16(Reg16::SP),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x34
    InstrInfo {
        mnemonic: Mnemonic::IncHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x35
    InstrInfo {
        mnemonic: Mnemonic::DecHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x36
    InstrInfo {
        mnemonic: Mnemonic::LdHlAddrImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(3),
    },
    // 0x37
    InstrInfo {
        mnemonic: Mnemonic::Scf,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x38
    InstrInfo {
        mnemonic: Mnemonic::JrCond(Cond::C, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Branch(2, 3),
    },
    // 0x39
    InstrInfo {
        mnemonic: Mnemonic::AddHlReg16(Reg16::SP),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x3a
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg16Dec(Reg16::HL)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x3b
    InstrInfo {
        mnemonic: Mnemonic::DecReg16(Reg16::SP),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x3c
    InstrInfo {
        mnemonic: Mnemonic::IncReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x3d
    InstrInfo {
        mnemonic: Mnemonic::DecReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x3e
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::A, Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0x3f
    InstrInfo {
        mnemonic: Mnemonic::Ccf,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x40
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x41
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x42
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x43
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x44
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x45
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x46
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::B, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x47
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x48
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x49
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x4a
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x4b
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x4c
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x4d
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x4e
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::C, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x4f
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x50
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x51
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x52
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x53
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x54
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x55
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x56
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::D, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x57
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x58
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x59
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x5a
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x5b
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x5c
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x5d
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x5e
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::E, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x5f
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x60
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x61
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x62
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x63
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x64
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x65
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x66
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::H, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x67
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x68
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x69
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x6a
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x6b
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x6c
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x6d
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x6e
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::L, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x6f
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x70
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x71
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x72
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x73
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x74
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x75
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x76
    InstrInfo {
        mnemonic: Mnemonic::Halt,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x77
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x78
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x79
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x7a
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x7b
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x7c
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x7d
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x7e
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x7f
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x80
    InstrInfo {
        mnemonic: Mnemonic::AddAReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x81
    InstrInfo {
        mnemonic: Mnemonic::AddAReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x82
    InstrInfo {
        mnemonic: Mnemonic::AddAReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x83
    InstrInfo {
        mnemonic: Mnemonic::AddAReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x84
    InstrInfo {
        mnemonic: Mnemonic::AddAReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x85
    InstrInfo {
        mnemonic: Mnemonic::AddAReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x86
    InstrInfo {
        mnemonic: Mnemonic::AddAHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x87
    InstrInfo {
        mnemonic: Mnemonic::AddAReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x88
    InstrInfo {
        mnemonic: Mnemonic::AdcAReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x89
    InstrInfo {
        mnemonic: Mnemonic::AdcAReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x8a
    InstrInfo {
        mnemonic: Mnemonic::AdcAReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x8b
    InstrInfo {
        mnemonic: Mnemonic::AdcAReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x8c
    InstrInfo {
        mnemonic: Mnemonic::AdcAReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x8d
    InstrInfo {
        mnemonic: Mnemonic::AdcAReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x8e
    InstrInfo {
        mnemonic: Mnemonic::AdcAHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x8f
    InstrInfo {
        mnemonic: Mnemonic::AdcAReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x90
    InstrInfo {
        mnemonic: Mnemonic::SubAReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x91
    InstrInfo {
        mnemonic: Mnemonic::SubAReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x92
    InstrInfo {
        mnemonic: Mnemonic::SubAReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x93
    InstrInfo {
        mnemonic: Mnemonic::SubAReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x94
    InstrInfo {
        mnemonic: Mnemonic::SubAReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x95
    InstrInfo {
        mnemonic: Mnemonic::SubAReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x96
    InstrInfo {
        mnemonic: Mnemonic::SubAHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x97
    InstrInfo {
        mnemonic: Mnemonic::SubAReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x98
    InstrInfo {
        mnemonic: Mnemonic::SbcAReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x99
    InstrInfo {
        mnemonic: Mnemonic::SbcAReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x9a
    InstrInfo {
        mnemonic: Mnemonic::SbcAReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x9b
    InstrInfo {
        mnemonic: Mnemonic::SbcAReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x9c
    InstrInfo {
        mnemonic: Mnemonic::SbcAReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x9d
    InstrInfo {
        mnemonic: Mnemonic::SbcAReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x9e
    InstrInfo {
        mnemonic: Mnemonic::SbcAHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x9f
    InstrInfo {
        mnemonic: Mnemonic::SbcAReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa0
    InstrInfo {
        mnemonic: Mnemonic::AndAReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa1
    InstrInfo {
        mnemonic: Mnemonic::AndAReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa2
    InstrInfo {
        mnemonic: Mnemonic::AndAReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa3
    InstrInfo {
        mnemonic: Mnemonic::AndAReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa4
    InstrInfo {
        mnemonic: Mnemonic::AndAReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa5
    InstrInfo {
        mnemonic: Mnemonic::AndAReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa6
    InstrInfo {
        mnemonic: Mnemonic::AndAHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0xa7
    InstrInfo {
        mnemonic: Mnemonic::AndAReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa8
    InstrInfo {
        mnemonic: Mnemonic::XorAReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa9
    InstrInfo {
        mnemonic: Mnemonic::XorAReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xaa
    InstrInfo {
        mnemonic: Mnemonic::XorAReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xab
    InstrInfo {
        mnemonic: Mnemonic::XorAReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xac
    InstrInfo {
        mnemonic: Mnemonic::XorAReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xad
    InstrInfo {
        mnemonic: Mnemonic::XorAReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xae
    InstrInfo {
        mnemonic: Mnemonic::XorAHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0xaf
    InstrInfo {
        mnemonic: Mnemonic::XorAReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb0
    InstrInfo {
        mnemonic: Mnemonic::OrAReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb1
    InstrInfo {
        mnemonic: Mnemonic::OrAReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb2
    InstrInfo {
        mnemonic: Mnemonic::OrAReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb3
    InstrInfo {
        mnemonic: Mnemonic::OrAReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb4
    InstrInfo {
        mnemonic: Mnemonic::OrAReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb5
    InstrInfo {
        mnemonic: Mnemonic::OrAReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb6
    InstrInfo {
        mnemonic: Mnemonic::OrAHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0xb7
    InstrInfo {
        mnemonic: Mnemonic::OrAReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb8
    InstrInfo {
        mnemonic: Mnemonic::CpAReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb9
    InstrInfo {
        mnemonic: Mnemonic::CpAReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xba
    InstrInfo {
        mnemonic: Mnemonic::CpAReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xbb
    InstrInfo {
        mnemonic: Mnemonic::CpAReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xbc
    InstrInfo {
        mnemonic: Mnemonic::CpAReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xbd
    InstrInfo {
        mnemonic: Mnemonic::CpAReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xbe
    InstrInfo {
        mnemonic: Mnemonic::CpAHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0xbf
    InstrInfo {
        mnemonic: Mnemonic::CpAReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xc0
    InstrInfo {
        mnemonic: Mnemonic::RetCond(Cond::NZ),
        param_type: ParamType::None,
        cycles: Cycles::Branch(2, 5),
    },
    // 0xc1
    InstrInfo {
        mnemonic: Mnemonic::Pop(Reg16::BC),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xc2
    InstrInfo {
        mnemonic: Mnemonic::JpCond(Cond::NZ, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Branch(3, 4),
    },
    // 0xc3
    InstrInfo {
        mnemonic: Mnemonic::Jp(Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Normal(4),
    },
    // 0xc4
    InstrInfo {
        mnemonic: Mnemonic::CallCond(Cond::NZ, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Branch(3, 6),
    },
    // 0xc5
    InstrInfo {
        mnemonic: Mnemonic::Push(Reg16::BC),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xc6
    InstrInfo {
        mnemonic: Mnemonic::AddAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0xc7
    InstrInfo {
        mnemonic: Mnemonic::Rst(Imm::Known(0x00)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xc8
    InstrInfo {
        mnemonic: Mnemonic::RetCond(Cond::Z),
        param_type: ParamType::None,
        cycles: Cycles::Branch(2, 5),
    },
    // 0xc9
    InstrInfo {
        mnemonic: Mnemonic::Ret,
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xca
    InstrInfo {
        mnemonic: Mnemonic::JpCond(Cond::Z, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Branch(3, 4),
    },
    // 0xcb Invalid as it is a prefix.
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xcc
    InstrInfo {
        mnemonic: Mnemonic::CallCond(Cond::Z, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Branch(3, 6),
    },
    // 0xcd
    InstrInfo {
        mnemonic: Mnemonic::Call(Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Normal(6),
    },
    // 0xce
    InstrInfo {
        mnemonic: Mnemonic::AdcAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0xcf
    InstrInfo {
        mnemonic: Mnemonic::Rst(Imm::Known(0x08)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xd0
    InstrInfo {
        mnemonic: Mnemonic::RetCond(Cond::NC),
        param_type: ParamType::None,
        cycles: Cycles::Branch(2, 5),
    },
    // 0xd1
    InstrInfo {
        mnemonic: Mnemonic::Pop(Reg16::DE),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xd2
    InstrInfo {
        mnemonic: Mnemonic::JpCond(Cond::NC, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Branch(3, 4),
    },
    // 0xd3
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xd4
    InstrInfo {
        mnemonic: Mnemonic::CallCond(Cond::NC, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Branch(3, 6),
    },
    // 0xd5
    InstrInfo {
        mnemonic: Mnemonic::Push(Reg16::DE),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xd6
    InstrInfo {
        mnemonic: Mnemonic::SubAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0xd7
    InstrInfo {
        mnemonic: Mnemonic::Rst(Imm::Known(0x10)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xd8
    InstrInfo {
        mnemonic: Mnemonic::RetCond(Cond::C),
        param_type: ParamType::None,
        cycles: Cycles::Branch(2, 5),
    },
    // 0xd9
    InstrInfo {
        mnemonic: Mnemonic::Reti,
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xda
    InstrInfo {
        mnemonic: Mnemonic::JpCond(Cond::C, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Branch(3, 4),
    },
    // 0xdb
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xdc
    InstrInfo {
        mnemonic: Mnemonic::CallCond(Cond::C, Imm::Unknown),
        param_type: ParamType::Word,
        cycles: Cycles::Branch(3, 6),
    },
    // 0xdd
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xde
    InstrInfo {
        mnemonic: Mnemonic::SbcAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0xdf
    InstrInfo {
        mnemonic: Mnemonic::Rst(Imm::Known(0x18)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xe0
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Imm8(Imm::Unknown), Reg8::A),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(3),
    },
    // 0xe1
    InstrInfo {
        mnemonic: Mnemonic::Pop(Reg16::HL),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xe2
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg8(Reg8::C), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0xe3
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xe4
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xe5
    InstrInfo {
        mnemonic: Mnemonic::Push(Reg16::HL),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xe6
    InstrInfo {
        mnemonic: Mnemonic::AndAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0xe7
    InstrInfo {
        mnemonic: Mnemonic::Rst(Imm::Known(0x20)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xe8
    InstrInfo {
        mnemonic: Mnemonic::AddSpImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(4),
    },
    // 0xe9
    InstrInfo {
        mnemonic: Mnemonic::JpHl,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xea
    InstrInfo {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Imm16(Imm::Unknown), Reg8::A),
        param_type: ParamType::Word,
        cycles: Cycles::Normal(4),
    },
    // 0xeb
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xec
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xed
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xee
    InstrInfo {
        mnemonic: Mnemonic::XorAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0xef
    InstrInfo {
        mnemonic: Mnemonic::Rst(Imm::Known(0x28)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xf0
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Imm8(Imm::Unknown)),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(3),
    },
    // 0xf1
    InstrInfo {
        mnemonic: Mnemonic::Pop(Reg16::AF),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xf2
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg8(Reg8::C)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0xf3
    InstrInfo {
        mnemonic: Mnemonic::Di,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xf4
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xf5
    InstrInfo {
        mnemonic: Mnemonic::Push(Reg16::AF),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xf6
    InstrInfo {
        mnemonic: Mnemonic::OrAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0xf7
    InstrInfo {
        mnemonic: Mnemonic::Rst(Imm::Known(0x30)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
    // 0xf8
    InstrInfo {
        mnemonic: Mnemonic::LdHlAddSpImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(3),
    },
    // 0xf9
    InstrInfo {
        mnemonic: Mnemonic::LdSpHl,
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0xfa
    InstrInfo {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Imm16(Imm::Unknown)),
        param_type: ParamType::Word,
        cycles: Cycles::Normal(4),
    },
    // 0xfb
    InstrInfo {
        mnemonic: Mnemonic::Ei,
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xfc
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xfd
    InstrInfo {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        cycles: Cycles::Normal(0),
    },
    // 0xfe
    InstrInfo {
        mnemonic: Mnemonic::CpAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        cycles: Cycles::Normal(2),
    },
    // 0xff
    InstrInfo {
        mnemonic: Mnemonic::Rst(Imm::Known(0x38)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(4),
    },
];
