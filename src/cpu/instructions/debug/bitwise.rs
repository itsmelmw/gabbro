use crate::cpu::instructions::debug::{Cycles, Imm, InstrInfo, Mnemonic, ParamType, Reg8};

pub const BITWISE_INSTR_INFO: [InstrInfo; 256] = [
    // 0x00
    InstrInfo {
        mnemonic: Mnemonic::RlcReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x01
    InstrInfo {
        mnemonic: Mnemonic::RlcReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x02
    InstrInfo {
        mnemonic: Mnemonic::RlcReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x03
    InstrInfo {
        mnemonic: Mnemonic::RlcReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x04
    InstrInfo {
        mnemonic: Mnemonic::RlcReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x05
    InstrInfo {
        mnemonic: Mnemonic::RlcReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x06
    InstrInfo {
        mnemonic: Mnemonic::RlcHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x07
    InstrInfo {
        mnemonic: Mnemonic::RlcReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x08
    InstrInfo {
        mnemonic: Mnemonic::RrcReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x09
    InstrInfo {
        mnemonic: Mnemonic::RrcReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x0a
    InstrInfo {
        mnemonic: Mnemonic::RrcReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x0b
    InstrInfo {
        mnemonic: Mnemonic::RrcReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x0c
    InstrInfo {
        mnemonic: Mnemonic::RrcReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x0d
    InstrInfo {
        mnemonic: Mnemonic::RrcReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x0e
    InstrInfo {
        mnemonic: Mnemonic::RrcHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x0f
    InstrInfo {
        mnemonic: Mnemonic::RrcReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x10
    InstrInfo {
        mnemonic: Mnemonic::RlReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x11
    InstrInfo {
        mnemonic: Mnemonic::RlReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x12
    InstrInfo {
        mnemonic: Mnemonic::RlReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x13
    InstrInfo {
        mnemonic: Mnemonic::RlReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x14
    InstrInfo {
        mnemonic: Mnemonic::RlReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x15
    InstrInfo {
        mnemonic: Mnemonic::RlReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x16
    InstrInfo {
        mnemonic: Mnemonic::RlHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x17
    InstrInfo {
        mnemonic: Mnemonic::RlReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x18
    InstrInfo {
        mnemonic: Mnemonic::RrReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x19
    InstrInfo {
        mnemonic: Mnemonic::RrReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x1a
    InstrInfo {
        mnemonic: Mnemonic::RrReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x1b
    InstrInfo {
        mnemonic: Mnemonic::RrReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x1c
    InstrInfo {
        mnemonic: Mnemonic::RrReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x1d
    InstrInfo {
        mnemonic: Mnemonic::RrReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x1e
    InstrInfo {
        mnemonic: Mnemonic::RrHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x1f
    InstrInfo {
        mnemonic: Mnemonic::RrReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x20
    InstrInfo {
        mnemonic: Mnemonic::SlaReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x21
    InstrInfo {
        mnemonic: Mnemonic::SlaReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x22
    InstrInfo {
        mnemonic: Mnemonic::SlaReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x23
    InstrInfo {
        mnemonic: Mnemonic::SlaReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x24
    InstrInfo {
        mnemonic: Mnemonic::SlaReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x25
    InstrInfo {
        mnemonic: Mnemonic::SlaReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x26
    InstrInfo {
        mnemonic: Mnemonic::SlaHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x27
    InstrInfo {
        mnemonic: Mnemonic::SlaReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x28
    InstrInfo {
        mnemonic: Mnemonic::SraReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x29
    InstrInfo {
        mnemonic: Mnemonic::SraReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x2a
    InstrInfo {
        mnemonic: Mnemonic::SraReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x2b
    InstrInfo {
        mnemonic: Mnemonic::SraReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x2c
    InstrInfo {
        mnemonic: Mnemonic::SraReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x2d
    InstrInfo {
        mnemonic: Mnemonic::SraReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x2e
    InstrInfo {
        mnemonic: Mnemonic::SraHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x2f
    InstrInfo {
        mnemonic: Mnemonic::SraReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x30
    InstrInfo {
        mnemonic: Mnemonic::SwapReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x31
    InstrInfo {
        mnemonic: Mnemonic::SwapReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x32
    InstrInfo {
        mnemonic: Mnemonic::SwapReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x33
    InstrInfo {
        mnemonic: Mnemonic::SwapReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x34
    InstrInfo {
        mnemonic: Mnemonic::SwapReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x35
    InstrInfo {
        mnemonic: Mnemonic::SwapReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x36
    InstrInfo {
        mnemonic: Mnemonic::SwapHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x37
    InstrInfo {
        mnemonic: Mnemonic::SwapReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x38
    InstrInfo {
        mnemonic: Mnemonic::SrlReg8(Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x39
    InstrInfo {
        mnemonic: Mnemonic::SrlReg8(Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x3a
    InstrInfo {
        mnemonic: Mnemonic::SrlReg8(Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x3b
    InstrInfo {
        mnemonic: Mnemonic::SrlReg8(Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x3c
    InstrInfo {
        mnemonic: Mnemonic::SrlReg8(Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x3d
    InstrInfo {
        mnemonic: Mnemonic::SrlReg8(Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x3e
    InstrInfo {
        mnemonic: Mnemonic::SrlHlAddr,
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x3f
    InstrInfo {
        mnemonic: Mnemonic::SrlReg8(Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x40
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x41
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x42
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x43
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x44
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x45
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x46
    InstrInfo {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(0)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x47
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x48
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x49
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x4a
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x4b
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x4c
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x4d
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x4e
    InstrInfo {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(1)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x4f
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x50
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x51
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x52
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x53
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x54
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x55
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x56
    InstrInfo {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(2)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x57
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x58
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x59
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x5a
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x5b
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x5c
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x5d
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x5e
    InstrInfo {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(3)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x5f
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x60
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x61
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x62
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x63
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x64
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x65
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x66
    InstrInfo {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(4)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x67
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x68
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x69
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x6a
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x6b
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x6c
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x6d
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x6e
    InstrInfo {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(5)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x6f
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x70
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x71
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x72
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x73
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x74
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x75
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x76
    InstrInfo {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(6)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x77
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x78
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x79
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x7a
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x7b
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x7c
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x7d
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x7e
    InstrInfo {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(7)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(2),
    },
    // 0x7f
    InstrInfo {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x80
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x81
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x82
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x83
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x84
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x85
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x86
    InstrInfo {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(0)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x87
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x88
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x89
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x8a
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x8b
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x8c
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x8d
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x8e
    InstrInfo {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(1)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x8f
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x90
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x91
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x92
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x93
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x94
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x95
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x96
    InstrInfo {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(2)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x97
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x98
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x99
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x9a
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x9b
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x9c
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x9d
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0x9e
    InstrInfo {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(3)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0x9f
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa0
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa1
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa2
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa3
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa4
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa5
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa6
    InstrInfo {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(4)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xa7
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa8
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xa9
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xaa
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xab
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xac
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xad
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xae
    InstrInfo {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(5)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xaf
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb0
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb1
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb2
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb3
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb4
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb5
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb6
    InstrInfo {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(6)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xb7
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb8
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xb9
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xba
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xbb
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xbc
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xbd
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xbe
    InstrInfo {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(7)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xbf
    InstrInfo {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xc0
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xc1
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xc2
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xc3
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xc4
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xc5
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xc6
    InstrInfo {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(0)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xc7
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xc8
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xc9
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xca
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xcb
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xcc
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xcd
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xce
    InstrInfo {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(1)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xcf
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xd0
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xd1
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xd2
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xd3
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xd4
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xd5
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xd6
    InstrInfo {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(2)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xd7
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xd8
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xd9
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xda
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xdb
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xdc
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xdd
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xde
    InstrInfo {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(3)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xdf
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xe0
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xe1
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xe2
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xe3
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xe4
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xe5
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xe6
    InstrInfo {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(4)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xe7
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xe8
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xe9
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xea
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xeb
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xec
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xed
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xee
    InstrInfo {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(5)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xef
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xf0
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xf1
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xf2
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xf3
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xf4
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xf5
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xf6
    InstrInfo {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(6)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xf7
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xf8
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::B),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xf9
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::C),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xfa
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::D),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xfb
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::E),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xfc
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::H),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xfd
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::L),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
    // 0xfe
    InstrInfo {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(7)),
        param_type: ParamType::None,
        cycles: Cycles::Normal(3),
    },
    // 0xff
    InstrInfo {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::A),
        param_type: ParamType::None,
        cycles: Cycles::Normal(1),
    },
];
