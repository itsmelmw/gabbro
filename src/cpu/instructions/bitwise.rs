use super::{helpers, InstrSet, Instruction, ParamType};

use super::mnemonic::{Imm, Mnemonic, Reg8};

pub const BITWISE_PREFIX: u8 = 0xcb;

pub const BITWISE_INSTRS: InstrSet = [
    // 0x00
    Instruction {
        mnemonic: Mnemonic::RlcReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rlc(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x01
    Instruction {
        mnemonic: Mnemonic::RlcReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rlc(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x02
    Instruction {
        mnemonic: Mnemonic::RlcReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rlc(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x03
    Instruction {
        mnemonic: Mnemonic::RlcReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rlc(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x04
    Instruction {
        mnemonic: Mnemonic::RlcReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rlc(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x05
    Instruction {
        mnemonic: Mnemonic::RlcReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rlc(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x06
    Instruction {
        mnemonic: Mnemonic::RlcHlAddr,
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::rlc(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x07
    Instruction {
        mnemonic: Mnemonic::RlcReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rlc(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x08
    Instruction {
        mnemonic: Mnemonic::RrcReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rrc(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x09
    Instruction {
        mnemonic: Mnemonic::RrcReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rrc(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x0a
    Instruction {
        mnemonic: Mnemonic::RrcReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rrc(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x0b
    Instruction {
        mnemonic: Mnemonic::RrcReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rrc(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x0c
    Instruction {
        mnemonic: Mnemonic::RrcReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rrc(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x0d
    Instruction {
        mnemonic: Mnemonic::RrcReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rrc(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x0e
    Instruction {
        mnemonic: Mnemonic::RrcHlAddr,
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::rrc(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x0f
    Instruction {
        mnemonic: Mnemonic::RrcReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rrc(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x10
    Instruction {
        mnemonic: Mnemonic::RlReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rl(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x11
    Instruction {
        mnemonic: Mnemonic::RlReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rl(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x12
    Instruction {
        mnemonic: Mnemonic::RlReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rl(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x13
    Instruction {
        mnemonic: Mnemonic::RlReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rl(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x14
    Instruction {
        mnemonic: Mnemonic::RlReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rl(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x15
    Instruction {
        mnemonic: Mnemonic::RlReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rl(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x16
    Instruction {
        mnemonic: Mnemonic::RlHlAddr,
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::rl(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x17
    Instruction {
        mnemonic: Mnemonic::RlReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rl(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x18
    Instruction {
        mnemonic: Mnemonic::RrReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rr(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x19
    Instruction {
        mnemonic: Mnemonic::RrReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rr(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x1a
    Instruction {
        mnemonic: Mnemonic::RrReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rr(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x1b
    Instruction {
        mnemonic: Mnemonic::RrReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rr(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x1c
    Instruction {
        mnemonic: Mnemonic::RrReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rr(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x1d
    Instruction {
        mnemonic: Mnemonic::RrReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rr(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x1e
    Instruction {
        mnemonic: Mnemonic::RrHlAddr,
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::rr(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x1f
    Instruction {
        mnemonic: Mnemonic::RrReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rr(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x20
    Instruction {
        mnemonic: Mnemonic::SlaReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sla(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x21
    Instruction {
        mnemonic: Mnemonic::SlaReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sla(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x22
    Instruction {
        mnemonic: Mnemonic::SlaReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sla(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x23
    Instruction {
        mnemonic: Mnemonic::SlaReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sla(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x24
    Instruction {
        mnemonic: Mnemonic::SlaReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sla(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x25
    Instruction {
        mnemonic: Mnemonic::SlaReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sla(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x26
    Instruction {
        mnemonic: Mnemonic::SlaHlAddr,
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::sla(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x27
    Instruction {
        mnemonic: Mnemonic::SlaReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sla(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x28
    Instruction {
        mnemonic: Mnemonic::SraReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sra(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x29
    Instruction {
        mnemonic: Mnemonic::SraReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sra(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x2a
    Instruction {
        mnemonic: Mnemonic::SraReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sra(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x2b
    Instruction {
        mnemonic: Mnemonic::SraReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sra(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x2c
    Instruction {
        mnemonic: Mnemonic::SraReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sra(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x2d
    Instruction {
        mnemonic: Mnemonic::SraReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sra(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x2e
    Instruction {
        mnemonic: Mnemonic::SraHlAddr,
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::sra(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x2f
    Instruction {
        mnemonic: Mnemonic::SraReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::sra(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x30
    Instruction {
        mnemonic: Mnemonic::SwapReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::swap(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x31
    Instruction {
        mnemonic: Mnemonic::SwapReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::swap(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x32
    Instruction {
        mnemonic: Mnemonic::SwapReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::swap(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x33
    Instruction {
        mnemonic: Mnemonic::SwapReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::swap(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x34
    Instruction {
        mnemonic: Mnemonic::SwapReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::swap(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x35
    Instruction {
        mnemonic: Mnemonic::SwapReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::swap(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x36
    Instruction {
        mnemonic: Mnemonic::SwapHlAddr,
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::swap(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x37
    Instruction {
        mnemonic: Mnemonic::SwapReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::swap(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x38
    Instruction {
        mnemonic: Mnemonic::SrlReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::srl(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x39
    Instruction {
        mnemonic: Mnemonic::SrlReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::srl(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x3a
    Instruction {
        mnemonic: Mnemonic::SrlReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::srl(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x3b
    Instruction {
        mnemonic: Mnemonic::SrlReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::srl(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x3c
    Instruction {
        mnemonic: Mnemonic::SrlReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::srl(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x3d
    Instruction {
        mnemonic: Mnemonic::SrlReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::srl(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x3e
    Instruction {
        mnemonic: Mnemonic::SrlHlAddr,
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::srl(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x3f
    Instruction {
        mnemonic: Mnemonic::SrlReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::srl(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x40
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 0, cpu.regs.b());
        },
    },
    // 0x41
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 0, cpu.regs.c());
        },
    },
    // 0x42
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 0, cpu.regs.d());
        },
    },
    // 0x43
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 0, cpu.regs.e());
        },
    },
    // 0x44
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 0, cpu.regs.h());
        },
    },
    // 0x45
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 0, cpu.regs.l());
        },
    },
    // 0x46
    Instruction {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(0)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::bit(cpu, 0, val);
        },
    },
    // 0x47
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(0), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 0, cpu.regs.a());
        },
    },
    // 0x48
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 1, cpu.regs.b());
        },
    },
    // 0x49
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 1, cpu.regs.c());
        },
    },
    // 0x4a
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 1, cpu.regs.d());
        },
    },
    // 0x4b
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 1, cpu.regs.e());
        },
    },
    // 0x4c
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 1, cpu.regs.h());
        },
    },
    // 0x4d
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 1, cpu.regs.l());
        },
    },
    // 0x4e
    Instruction {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(1)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::bit(cpu, 1, val);
        },
    },
    // 0x4f
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(1), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 1, cpu.regs.a());
        },
    },
    // 0x50
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 2, cpu.regs.b());
        },
    },
    // 0x51
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 2, cpu.regs.c());
        },
    },
    // 0x52
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 2, cpu.regs.d());
        },
    },
    // 0x53
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 2, cpu.regs.e());
        },
    },
    // 0x54
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 2, cpu.regs.h());
        },
    },
    // 0x55
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 2, cpu.regs.l());
        },
    },
    // 0x56
    Instruction {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(2)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::bit(cpu, 2, val);
        },
    },
    // 0x57
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(2), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 2, cpu.regs.a());
        },
    },
    // 0x58
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 3, cpu.regs.b());
        },
    },
    // 0x59
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 3, cpu.regs.c());
        },
    },
    // 0x5a
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 3, cpu.regs.d());
        },
    },
    // 0x5b
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 3, cpu.regs.e());
        },
    },
    // 0x5c
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 3, cpu.regs.h());
        },
    },
    // 0x5d
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 3, cpu.regs.l());
        },
    },
    // 0x5e
    Instruction {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(3)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::bit(cpu, 3, val);
        },
    },
    // 0x5f
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(3), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 3, cpu.regs.a());
        },
    },
    // 0x60
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 4, cpu.regs.b());
        },
    },
    // 0x61
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 4, cpu.regs.c());
        },
    },
    // 0x62
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 4, cpu.regs.d());
        },
    },
    // 0x63
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 4, cpu.regs.e());
        },
    },
    // 0x64
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 4, cpu.regs.h());
        },
    },
    // 0x65
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 4, cpu.regs.l());
        },
    },
    // 0x66
    Instruction {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(4)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::bit(cpu, 4, val);
        },
    },
    // 0x67
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(4), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 4, cpu.regs.a());
        },
    },
    // 0x68
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 5, cpu.regs.b());
        },
    },
    // 0x69
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 5, cpu.regs.c());
        },
    },
    // 0x6a
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 5, cpu.regs.d());
        },
    },
    // 0x6b
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 5, cpu.regs.e());
        },
    },
    // 0x6c
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 5, cpu.regs.h());
        },
    },
    // 0x6d
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 5, cpu.regs.l());
        },
    },
    // 0x6e
    Instruction {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(5)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::bit(cpu, 5, val);
        },
    },
    // 0x6f
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(5), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 5, cpu.regs.a());
        },
    },
    // 0x70
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 6, cpu.regs.b());
        },
    },
    // 0x71
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 6, cpu.regs.c());
        },
    },
    // 0x72
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 6, cpu.regs.d());
        },
    },
    // 0x73
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 6, cpu.regs.e());
        },
    },
    // 0x74
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 6, cpu.regs.h());
        },
    },
    // 0x75
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 6, cpu.regs.l());
        },
    },
    // 0x76
    Instruction {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(6)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::bit(cpu, 6, val);
        },
    },
    // 0x77
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(6), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 6, cpu.regs.a());
        },
    },
    // 0x78
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 7, cpu.regs.b());
        },
    },
    // 0x79
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 7, cpu.regs.c());
        },
    },
    // 0x7a
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 7, cpu.regs.d());
        },
    },
    // 0x7b
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 7, cpu.regs.e());
        },
    },
    // 0x7c
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 7, cpu.regs.h());
        },
    },
    // 0x7d
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 7, cpu.regs.l());
        },
    },
    // 0x7e
    Instruction {
        mnemonic: Mnemonic::BitHlAddr(Imm::Known(7)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::bit(cpu, 7, val);
        },
    },
    // 0x7f
    Instruction {
        mnemonic: Mnemonic::BitReg8(Imm::Known(7), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::bit(cpu, 7, cpu.regs.a());
        },
    },
    // 0x80
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::res(0, cpu.regs.b()));
        },
    },
    // 0x81
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::res(0, cpu.regs.c()));
        },
    },
    // 0x82
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::res(0, cpu.regs.d()));
        },
    },
    // 0x83
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::res(0, cpu.regs.e()));
        },
    },
    // 0x84
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::res(0, cpu.regs.h()));
        },
    },
    // 0x85
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::res(0, cpu.regs.l()));
        },
    },
    // 0x86
    Instruction {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(0)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::res(0, val));
        },
    },
    // 0x87
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(0), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::res(0, cpu.regs.a()));
        },
    },
    // 0x88
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::res(1, cpu.regs.b()));
        },
    },
    // 0x89
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::res(1, cpu.regs.c()));
        },
    },
    // 0x8a
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::res(1, cpu.regs.d()));
        },
    },
    // 0x8b
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::res(1, cpu.regs.e()));
        },
    },
    // 0x8c
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::res(1, cpu.regs.h()));
        },
    },
    // 0x8d
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::res(1, cpu.regs.l()));
        },
    },
    // 0x8e
    Instruction {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(1)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::res(1, val));
        },
    },
    // 0x8f
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(1), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::res(1, cpu.regs.a()));
        },
    },
    // 0x90
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::res(2, cpu.regs.b()));
        },
    },
    // 0x91
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::res(2, cpu.regs.c()));
        },
    },
    // 0x92
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::res(2, cpu.regs.d()));
        },
    },
    // 0x93
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::res(2, cpu.regs.e()));
        },
    },
    // 0x94
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::res(2, cpu.regs.h()));
        },
    },
    // 0x95
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::res(2, cpu.regs.l()));
        },
    },
    // 0x96
    Instruction {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(2)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::res(2, val));
        },
    },
    // 0x97
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(2), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::res(2, cpu.regs.a()));
        },
    },
    // 0x98
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::res(3, cpu.regs.b()));
        },
    },
    // 0x99
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::res(3, cpu.regs.c()));
        },
    },
    // 0x9a
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::res(3, cpu.regs.d()));
        },
    },
    // 0x9b
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::res(3, cpu.regs.e()));
        },
    },
    // 0x9c
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::res(3, cpu.regs.h()));
        },
    },
    // 0x9d
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::res(3, cpu.regs.l()));
        },
    },
    // 0x9e
    Instruction {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(3)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::res(3, val));
        },
    },
    // 0x9f
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(3), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::res(3, cpu.regs.a()));
        },
    },
    // 0xa0
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::res(4, cpu.regs.b()));
        },
    },
    // 0xa1
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::res(4, cpu.regs.c()));
        },
    },
    // 0xa2
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::res(4, cpu.regs.d()));
        },
    },
    // 0xa3
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::res(4, cpu.regs.e()));
        },
    },
    // 0xa4
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::res(4, cpu.regs.h()));
        },
    },
    // 0xa5
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::res(4, cpu.regs.l()));
        },
    },
    // 0xa6
    Instruction {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(4)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::res(4, val));
        },
    },
    // 0xa7
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(4), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::res(4, cpu.regs.a()));
        },
    },
    // 0xa8
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::res(5, cpu.regs.b()));
        },
    },
    // 0xa9
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::res(5, cpu.regs.c()));
        },
    },
    // 0xaa
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::res(5, cpu.regs.d()));
        },
    },
    // 0xab
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::res(5, cpu.regs.e()));
        },
    },
    // 0xac
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::res(5, cpu.regs.h()));
        },
    },
    // 0xad
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::res(5, cpu.regs.l()));
        },
    },
    // 0xae
    Instruction {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(5)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::res(5, val));
        },
    },
    // 0xaf
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(5), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::res(5, cpu.regs.a()));
        },
    },
    // 0xb0
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::res(6, cpu.regs.b()));
        },
    },
    // 0xb1
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::res(6, cpu.regs.c()));
        },
    },
    // 0xb2
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::res(6, cpu.regs.d()));
        },
    },
    // 0xb3
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::res(6, cpu.regs.e()));
        },
    },
    // 0xb4
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::res(6, cpu.regs.h()));
        },
    },
    // 0xb5
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::res(6, cpu.regs.l()));
        },
    },
    // 0xb6
    Instruction {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(6)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::res(6, val));
        },
    },
    // 0xb7
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(6), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::res(6, cpu.regs.a()));
        },
    },
    // 0xb8
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::res(7, cpu.regs.b()));
        },
    },
    // 0xb9
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::res(7, cpu.regs.c()));
        },
    },
    // 0xba
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::res(7, cpu.regs.d()));
        },
    },
    // 0xbb
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::res(7, cpu.regs.e()));
        },
    },
    // 0xbc
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::res(7, cpu.regs.h()));
        },
    },
    // 0xbd
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::res(7, cpu.regs.l()));
        },
    },
    // 0xbe
    Instruction {
        mnemonic: Mnemonic::ResHlAddr(Imm::Known(7)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::res(7, val));
        },
    },
    // 0xbf
    Instruction {
        mnemonic: Mnemonic::ResReg8(Imm::Known(7), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::res(7, cpu.regs.a()));
        },
    },
    // 0xc0
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::set(0, cpu.regs.b()));
        },
    },
    // 0xc1
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::set(0, cpu.regs.c()));
        },
    },
    // 0xc2
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::set(0, cpu.regs.d()));
        },
    },
    // 0xc3
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::set(0, cpu.regs.e()));
        },
    },
    // 0xc4
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::set(0, cpu.regs.h()));
        },
    },
    // 0xc5
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::set(0, cpu.regs.l()));
        },
    },
    // 0xc6
    Instruction {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(0)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::set(0, val));
        },
    },
    // 0xc7
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(0), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::set(0, cpu.regs.a()));
        },
    },
    // 0xc8
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::set(1, cpu.regs.b()));
        },
    },
    // 0xc9
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::set(1, cpu.regs.c()));
        },
    },
    // 0xca
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::set(1, cpu.regs.d()));
        },
    },
    // 0xcb
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::set(1, cpu.regs.e()));
        },
    },
    // 0xcc
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::set(1, cpu.regs.h()));
        },
    },
    // 0xcd
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::set(1, cpu.regs.l()));
        },
    },
    // 0xce
    Instruction {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(1)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::set(1, val));
        },
    },
    // 0xcf
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(1), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::set(1, cpu.regs.a()));
        },
    },
    // 0xd0
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::set(2, cpu.regs.b()));
        },
    },
    // 0xd1
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::set(2, cpu.regs.c()));
        },
    },
    // 0xd2
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::set(2, cpu.regs.d()));
        },
    },
    // 0xd3
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::set(2, cpu.regs.e()));
        },
    },
    // 0xd4
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::set(2, cpu.regs.h()));
        },
    },
    // 0xd5
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::set(2, cpu.regs.l()));
        },
    },
    // 0xd6
    Instruction {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(2)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::set(2, val));
        },
    },
    // 0xd7
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(2), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::set(2, cpu.regs.a()));
        },
    },
    // 0xd8
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::set(3, cpu.regs.b()));
        },
    },
    // 0xd9
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::set(3, cpu.regs.c()));
        },
    },
    // 0xda
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::set(3, cpu.regs.d()));
        },
    },
    // 0xdb
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::set(3, cpu.regs.e()));
        },
    },
    // 0xdc
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::set(3, cpu.regs.h()));
        },
    },
    // 0xdd
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::set(3, cpu.regs.l()));
        },
    },
    // 0xde
    Instruction {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(3)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::set(3, val));
        },
    },
    // 0xdf
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(3), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::set(3, cpu.regs.a()));
        },
    },
    // 0xe0
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::set(4, cpu.regs.b()));
        },
    },
    // 0xe1
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::set(4, cpu.regs.c()));
        },
    },
    // 0xe2
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::set(4, cpu.regs.d()));
        },
    },
    // 0xe3
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::set(4, cpu.regs.e()));
        },
    },
    // 0xe4
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::set(4, cpu.regs.h()));
        },
    },
    // 0xe5
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::set(4, cpu.regs.l()));
        },
    },
    // 0xe6
    Instruction {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(4)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::set(4, val));
        },
    },
    // 0xe7
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(4), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::set(4, cpu.regs.a()));
        },
    },
    // 0xe8
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::set(5, cpu.regs.b()));
        },
    },
    // 0xe9
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::set(5, cpu.regs.c()));
        },
    },
    // 0xea
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::set(5, cpu.regs.d()));
        },
    },
    // 0xeb
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::set(5, cpu.regs.e()));
        },
    },
    // 0xec
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::set(5, cpu.regs.h()));
        },
    },
    // 0xed
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::set(5, cpu.regs.l()));
        },
    },
    // 0xee
    Instruction {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(5)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::set(5, val));
        },
    },
    // 0xef
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(5), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::set(5, cpu.regs.a()));
        },
    },
    // 0xf0
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::set(6, cpu.regs.b()));
        },
    },
    // 0xf1
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::set(6, cpu.regs.c()));
        },
    },
    // 0xf2
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::set(6, cpu.regs.d()));
        },
    },
    // 0xf3
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::set(6, cpu.regs.e()));
        },
    },
    // 0xf4
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::set(6, cpu.regs.h()));
        },
    },
    // 0xf5
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::set(6, cpu.regs.l()));
        },
    },
    // 0xf6
    Instruction {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(6)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::set(6, val));
        },
    },
    // 0xf7
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(6), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::set(6, cpu.regs.a()));
        },
    },
    // 0xf8
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(helpers::set(7, cpu.regs.b()));
        },
    },
    // 0xf9
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(helpers::set(7, cpu.regs.c()));
        },
    },
    // 0xfa
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(helpers::set(7, cpu.regs.d()));
        },
    },
    // 0xfb
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(helpers::set(7, cpu.regs.e()));
        },
    },
    // 0xfc
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(helpers::set(7, cpu.regs.h()));
        },
    },
    // 0xfd
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(helpers::set(7, cpu.regs.l()));
        },
    },
    // 0xfe
    Instruction {
        mnemonic: Mnemonic::SetHlAddr(Imm::Known(7)),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.write_byte(cpu.regs.hl(), helpers::set(7, val));
        },
    },
    // 0xff
    Instruction {
        mnemonic: Mnemonic::SetReg8(Imm::Known(7), Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(helpers::set(7, cpu.regs.a()));
        },
    },
];
