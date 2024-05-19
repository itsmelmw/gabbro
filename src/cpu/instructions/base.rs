use crate::cpu::ImeState;

use super::{helpers, InstrSet, Instruction, ParamType};

use super::mnemonic::{Addr, Cond, Imm, Mnemonic, Reg16, Reg8};

pub const BASE_INSTRS: InstrSet = [
    // 0x00
    Instruction {
        mnemonic: Mnemonic::Nop,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |_| {},
    },
    // 0x01
    Instruction {
        mnemonic: Mnemonic::LdReg16Imm(Reg16::BC, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.regs.set_bc(val);
        },
    },
    // 0x02
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::BC), Reg8::A),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.bc(), cpu.regs.a());
        },
    },
    // 0x03
    Instruction {
        mnemonic: Mnemonic::IncReg16(Reg16::BC),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_bc(cpu.regs.bc().wrapping_add(1));
        },
    },
    // 0x04
    Instruction {
        mnemonic: Mnemonic::IncReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x05
    Instruction {
        mnemonic: Mnemonic::DecReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x06
    Instruction {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::B, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_b(val);
        },
    },
    // 0x07
    Instruction {
        mnemonic: Mnemonic::Rlca,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rlc(cpu, cpu.regs.a());
            cpu.regs.flags_mut().set_z(false);
            cpu.regs.set_a(res);
        },
    },
    // 0x08
    Instruction {
        mnemonic: Mnemonic::LdImmAddrSp(Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 5,
        _branch_cycles: None,
        operation: |cpu| {
            let addr = cpu.fetch_word();
            cpu.write_word(addr, cpu.regs.sp());
        },
    },
    // 0x09
    Instruction {
        mnemonic: Mnemonic::AddHlReg16(Reg16::BC),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::add_hl(cpu, cpu.regs.bc());
        },
    },
    // 0x0a
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg16(Reg16::BC)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.bc());
            cpu.regs.set_a(val);
        },
    },
    // 0x0b
    Instruction {
        mnemonic: Mnemonic::DecReg16(Reg16::BC),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_bc(cpu.regs.bc().wrapping_sub(1));
        },
    },
    // 0x0c
    Instruction {
        mnemonic: Mnemonic::IncReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x0d
    Instruction {
        mnemonic: Mnemonic::DecReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x0e
    Instruction {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::C, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_c(val);
        },
    },
    // 0x0f
    Instruction {
        mnemonic: Mnemonic::Rrca,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rrc(cpu, cpu.regs.a());
            cpu.regs.flags_mut().set_z(false);
            cpu.regs.set_a(res);
        },
    },
    // 0x10
    Instruction {
        mnemonic: Mnemonic::Stop,
        param_type: ParamType::Byte,
        _cycles: 1,
        _branch_cycles: None,
        operation: |_| {
            log::warn!("STOP instruction called, but it is not implemented properly yet.");
            // TODO
        },
    },
    // 0x11
    Instruction {
        mnemonic: Mnemonic::LdReg16Imm(Reg16::DE, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.regs.set_de(val);
        },
    },
    // 0x12
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::DE), Reg8::A),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.de(), cpu.regs.a());
        },
    },
    // 0x13
    Instruction {
        mnemonic: Mnemonic::IncReg16(Reg16::DE),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_de(cpu.regs.de().wrapping_add(1));
        },
    },
    // 0x14
    Instruction {
        mnemonic: Mnemonic::IncReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x15
    Instruction {
        mnemonic: Mnemonic::DecReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x16
    Instruction {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::D, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_d(val);
        },
    },
    // 0x17
    Instruction {
        mnemonic: Mnemonic::Rla,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rl(cpu, cpu.regs.a());
            cpu.regs.flags_mut().set_z(false);
            cpu.regs.set_a(res);
        },
    },
    // 0x18
    Instruction {
        mnemonic: Mnemonic::Jr(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte() as i8;
            cpu.cycle();
            helpers::jr(cpu, val);
        },
    },
    // 0x19
    Instruction {
        mnemonic: Mnemonic::AddHlReg16(Reg16::DE),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::add_hl(cpu, cpu.regs.de());
        },
    },
    // 0x1a
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg16(Reg16::DE)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.de());
            cpu.regs.set_a(val);
        },
    },
    // 0x1b
    Instruction {
        mnemonic: Mnemonic::DecReg16(Reg16::DE),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_de(cpu.regs.de().wrapping_sub(1));
        },
    },
    // 0x1c
    Instruction {
        mnemonic: Mnemonic::IncReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x1d
    Instruction {
        mnemonic: Mnemonic::DecReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x1e
    Instruction {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::E, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_e(val);
        },
    },
    // 0x1f
    Instruction {
        mnemonic: Mnemonic::Rra,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::rr(cpu, cpu.regs.a());
            cpu.regs.flags_mut().set_z(false);
            cpu.regs.set_a(res);
        },
    },
    // 0x20
    Instruction {
        mnemonic: Mnemonic::JrCond(Cond::NZ, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: Some(1),
        operation: |cpu| {
            let val = cpu.fetch_byte() as i8;
            if !cpu.regs.flags().z() {
                cpu.cycle();
                helpers::jr(cpu, val);
            }
        },
    },
    // 0x21
    Instruction {
        mnemonic: Mnemonic::LdReg16Imm(Reg16::HL, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.regs.set_hl(val);
        },
    },
    // 0x22
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16Inc(Reg16::HL), Reg8::A),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let hl = cpu.regs.hl();
            cpu.write_byte(hl, cpu.regs.a());
            cpu.regs.inc_hl();
        },
    },
    // 0x23
    Instruction {
        mnemonic: Mnemonic::IncReg16(Reg16::HL),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_hl(cpu.regs.hl().wrapping_add(1));
        },
    },
    // 0x24
    Instruction {
        mnemonic: Mnemonic::IncReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x25
    Instruction {
        mnemonic: Mnemonic::DecReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x26
    Instruction {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::H, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_h(val);
        },
    },
    // 0x27
    Instruction {
        mnemonic: Mnemonic::Daa,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::daa(cpu);
        },
    },
    // 0x28
    Instruction {
        mnemonic: Mnemonic::JrCond(Cond::Z, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: Some(1),
        operation: |cpu| {
            let val = cpu.fetch_byte() as i8;
            if cpu.regs.flags().z() {
                cpu.cycle();
                helpers::jr(cpu, val);
            }
        },
    },
    // 0x29
    Instruction {
        mnemonic: Mnemonic::AddHlReg16(Reg16::HL),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::add_hl(cpu, cpu.regs.hl());
        },
    },
    // 0x2a
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg16Inc(Reg16::HL)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let hl = cpu.regs.hl();
            let val = cpu.read_byte(hl);
            cpu.regs.inc_hl();
            cpu.regs.set_a(val);
        },
    },
    // 0x2b
    Instruction {
        mnemonic: Mnemonic::DecReg16(Reg16::HL),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_hl(cpu.regs.hl().wrapping_sub(1));
        },
    },
    // 0x2c
    Instruction {
        mnemonic: Mnemonic::IncReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x2d
    Instruction {
        mnemonic: Mnemonic::DecReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x2e
    Instruction {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::L, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_l(val);
        },
    },
    // 0x2f
    Instruction {
        mnemonic: Mnemonic::Cpl,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::cpl(cpu);
        },
    },
    // 0x30
    Instruction {
        mnemonic: Mnemonic::JrCond(Cond::NC, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: Some(1),
        operation: |cpu| {
            let val = cpu.fetch_byte() as i8;
            if !cpu.regs.flags().c() {
                cpu.cycle();
                helpers::jr(cpu, val);
            }
        },
    },
    // 0x31
    Instruction {
        mnemonic: Mnemonic::LdReg16Imm(Reg16::SP, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.regs.set_sp(val);
        },
    },
    // 0x32
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16Dec(Reg16::HL), Reg8::A),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let hl = cpu.regs.hl();
            cpu.write_byte(hl, cpu.regs.a());
            cpu.regs.dec_hl();
        },
    },
    // 0x33
    Instruction {
        mnemonic: Mnemonic::IncReg16(Reg16::SP),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_sp(cpu.regs.sp().wrapping_add(1));
        },
    },
    // 0x34
    Instruction {
        mnemonic: Mnemonic::IncHlAddr,
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::inc(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x35
    Instruction {
        mnemonic: Mnemonic::DecHlAddr,
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::dec(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x36
    Instruction {
        mnemonic: Mnemonic::LdHlAddrImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.write_byte(cpu.regs.hl(), val);
        },
    },
    // 0x37
    Instruction {
        mnemonic: Mnemonic::Scf,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::scf(cpu);
        },
    },
    // 0x38
    Instruction {
        mnemonic: Mnemonic::JrCond(Cond::C, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: Some(1),
        operation: |cpu| {
            let val = cpu.fetch_byte() as i8;
            if cpu.regs.flags().c() {
                cpu.cycle();
                helpers::jr(cpu, val);
            }
        },
    },
    // 0x39
    Instruction {
        mnemonic: Mnemonic::AddHlReg16(Reg16::SP),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::add_hl(cpu, cpu.regs.sp());
        },
    },
    // 0x3a
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg16Dec(Reg16::HL)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let hl = cpu.regs.hl();
            let val = cpu.read_byte(hl);
            cpu.regs.dec_hl();
            cpu.regs.set_a(val);
        },
    },
    // 0x3b
    Instruction {
        mnemonic: Mnemonic::DecReg16(Reg16::SP),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_sp(cpu.regs.sp().wrapping_sub(1));
        },
    },
    // 0x3c
    Instruction {
        mnemonic: Mnemonic::IncReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x3d
    Instruction {
        mnemonic: Mnemonic::DecReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x3e
    Instruction {
        mnemonic: Mnemonic::LdReg8Imm(Reg8::A, Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_a(val);
        },
    },
    // 0x3f
    Instruction {
        mnemonic: Mnemonic::Ccf,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::ccf(cpu);
        },
    },
    // 0x40
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |_| {},
    },
    // 0x41
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.c());
        },
    },
    // 0x42
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.d());
        },
    },
    // 0x43
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.e());
        },
    },
    // 0x44
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.h());
        },
    },
    // 0x45
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.l());
        },
    },
    // 0x46
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::B, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_b(val);
        },
    },
    // 0x47
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::B, Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.a());
        },
    },
    // 0x48
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.b());
        },
    },
    // 0x49
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |_| {},
    },
    // 0x4a
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.d());
        },
    },
    // 0x4b
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.e());
        },
    },
    // 0x4c
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.h());
        },
    },
    // 0x4d
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.l());
        },
    },
    // 0x4e
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::C, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_c(val);
        },
    },
    // 0x4f
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::C, Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.a());
        },
    },
    // 0x50
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.b());
        },
    },
    // 0x51
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.c());
        },
    },
    // 0x52
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |_| {},
    },
    // 0x53
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.e());
        },
    },
    // 0x54
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.h());
        },
    },
    // 0x55
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.l());
        },
    },
    // 0x56
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::D, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_d(val);
        },
    },
    // 0x57
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::D, Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.a());
        },
    },
    // 0x58
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.b());
        },
    },
    // 0x59
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.c());
        },
    },
    // 0x5a
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.d());
        },
    },
    // 0x5b
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |_| {},
    },
    // 0x5c
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.h());
        },
    },
    // 0x5d
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.l());
        },
    },
    // 0x5e
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::E, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_e(val);
        },
    },
    // 0x5f
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::E, Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.a());
        },
    },
    // 0x60
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.b());
        },
    },
    // 0x61
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.c());
        },
    },
    // 0x62
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.d());
        },
    },
    // 0x63
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.e());
        },
    },
    // 0x64
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |_| {},
    },
    // 0x65
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.l());
        },
    },
    // 0x66
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::H, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_h(val);
        },
    },
    // 0x67
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::H, Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.a());
        },
    },
    // 0x68
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.b());
        },
    },
    // 0x69
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.c());
        },
    },
    // 0x6a
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.d());
        },
    },
    // 0x6b
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.e());
        },
    },
    // 0x6c
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.h());
        },
    },
    // 0x6d
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |_| {},
    },
    // 0x6e
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::L, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_l(val);
        },
    },
    // 0x6f
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::L, Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.a());
        },
    },
    // 0x70
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::B),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.b());
        },
    },
    // 0x71
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::C),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.c());
        },
    },
    // 0x72
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::D),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.d());
        },
    },
    // 0x73
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::E),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.e());
        },
    },
    // 0x74
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::H),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.h());
        },
    },
    // 0x75
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::L),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.l());
        },
    },
    // 0x76
    Instruction {
        mnemonic: Mnemonic::Halt,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            log::debug!("CPU: Halted");
            cpu.halted = true;
        },
    },
    // 0x77
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg16(Reg16::HL), Reg8::A),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.a());
        },
    },
    // 0x78
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.b());
        },
    },
    // 0x79
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.c());
        },
    },
    // 0x7a
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.d());
        },
    },
    // 0x7b
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.e());
        },
    },
    // 0x7c
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.h());
        },
    },
    // 0x7d
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.l());
        },
    },
    // 0x7e
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg16(Reg16::HL)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_a(val);
        },
    },
    // 0x7f
    Instruction {
        mnemonic: Mnemonic::LdReg8Reg8(Reg8::A, Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |_| {},
    },
    // 0x80
    Instruction {
        mnemonic: Mnemonic::AddAReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.b());
        },
    },
    // 0x81
    Instruction {
        mnemonic: Mnemonic::AddAReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.c());
        },
    },
    // 0x82
    Instruction {
        mnemonic: Mnemonic::AddAReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.d());
        },
    },
    // 0x83
    Instruction {
        mnemonic: Mnemonic::AddAReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.e());
        },
    },
    // 0x84
    Instruction {
        mnemonic: Mnemonic::AddAReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.h());
        },
    },
    // 0x85
    Instruction {
        mnemonic: Mnemonic::AddAReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.l());
        },
    },
    // 0x86
    Instruction {
        mnemonic: Mnemonic::AddAHlAddr,
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::add(cpu, val);
        },
    },
    // 0x87
    Instruction {
        mnemonic: Mnemonic::AddAReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.a());
        },
    },
    // 0x88
    Instruction {
        mnemonic: Mnemonic::AdcAReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.b());
        },
    },
    // 0x89
    Instruction {
        mnemonic: Mnemonic::AdcAReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.c());
        },
    },
    // 0x8a
    Instruction {
        mnemonic: Mnemonic::AdcAReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.d());
        },
    },
    // 0x8b
    Instruction {
        mnemonic: Mnemonic::AdcAReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.e());
        },
    },
    // 0x8c
    Instruction {
        mnemonic: Mnemonic::AdcAReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.h());
        },
    },
    // 0x8d
    Instruction {
        mnemonic: Mnemonic::AdcAReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.l());
        },
    },
    // 0x8e
    Instruction {
        mnemonic: Mnemonic::AdcAHlAddr,
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::adc(cpu, val);
        },
    },
    // 0x8f
    Instruction {
        mnemonic: Mnemonic::AdcAReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.a());
        },
    },
    // 0x90
    Instruction {
        mnemonic: Mnemonic::SubAReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.b());
        },
    },
    // 0x91
    Instruction {
        mnemonic: Mnemonic::SubAReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.c());
        },
    },
    // 0x92
    Instruction {
        mnemonic: Mnemonic::SubAReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.d());
        },
    },
    // 0x93
    Instruction {
        mnemonic: Mnemonic::SubAReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.e());
        },
    },
    // 0x94
    Instruction {
        mnemonic: Mnemonic::SubAReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.h());
        },
    },
    // 0x95
    Instruction {
        mnemonic: Mnemonic::SubAReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.l());
        },
    },
    // 0x96
    Instruction {
        mnemonic: Mnemonic::SubAHlAddr,
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::sub(cpu, val);
        },
    },
    // 0x97
    Instruction {
        mnemonic: Mnemonic::SubAReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.a());
        },
    },
    // 0x98
    Instruction {
        mnemonic: Mnemonic::SbcAReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.b());
        },
    },
    // 0x99
    Instruction {
        mnemonic: Mnemonic::SbcAReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.c());
        },
    },
    // 0x9a
    Instruction {
        mnemonic: Mnemonic::SbcAReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.d());
        },
    },
    // 0x9b
    Instruction {
        mnemonic: Mnemonic::SbcAReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.e());
        },
    },
    // 0x9c
    Instruction {
        mnemonic: Mnemonic::SbcAReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.h());
        },
    },
    // 0x9d
    Instruction {
        mnemonic: Mnemonic::SbcAReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.l());
        },
    },
    // 0x9e
    Instruction {
        mnemonic: Mnemonic::SbcAHlAddr,
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::sbc(cpu, val);
        },
    },
    // 0x9f
    Instruction {
        mnemonic: Mnemonic::SbcAReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.a());
        },
    },
    // 0xa0
    Instruction {
        mnemonic: Mnemonic::AndAReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.b());
        },
    },
    // 0xa1
    Instruction {
        mnemonic: Mnemonic::AndAReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.c());
        },
    },
    // 0xa2
    Instruction {
        mnemonic: Mnemonic::AndAReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.d());
        },
    },
    // 0xa3
    Instruction {
        mnemonic: Mnemonic::AndAReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.e());
        },
    },
    // 0xa4
    Instruction {
        mnemonic: Mnemonic::AndAReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.h());
        },
    },
    // 0xa5
    Instruction {
        mnemonic: Mnemonic::AndAReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.l());
        },
    },
    // 0xa6
    Instruction {
        mnemonic: Mnemonic::AndAHlAddr,
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::and(cpu, val);
        },
    },
    // 0xa7
    Instruction {
        mnemonic: Mnemonic::AndAReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.a());
        },
    },
    // 0xa8
    Instruction {
        mnemonic: Mnemonic::XorAReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.b());
        },
    },
    // 0xa9
    Instruction {
        mnemonic: Mnemonic::XorAReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.c());
        },
    },
    // 0xaa
    Instruction {
        mnemonic: Mnemonic::XorAReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.d());
        },
    },
    // 0xab
    Instruction {
        mnemonic: Mnemonic::XorAReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.e());
        },
    },
    // 0xac
    Instruction {
        mnemonic: Mnemonic::XorAReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.h());
        },
    },
    // 0xad
    Instruction {
        mnemonic: Mnemonic::XorAReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.l());
        },
    },
    // 0xae
    Instruction {
        mnemonic: Mnemonic::XorAHlAddr,
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::xor(cpu, val);
        },
    },
    // 0xaf
    Instruction {
        mnemonic: Mnemonic::XorAReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.a());
        },
    },
    // 0xb0
    Instruction {
        mnemonic: Mnemonic::OrAReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.b());
        },
    },
    // 0xb1
    Instruction {
        mnemonic: Mnemonic::OrAReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.c());
        },
    },
    // 0xb2
    Instruction {
        mnemonic: Mnemonic::OrAReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.d());
        },
    },
    // 0xb3
    Instruction {
        mnemonic: Mnemonic::OrAReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.e());
        },
    },
    // 0xb4
    Instruction {
        mnemonic: Mnemonic::OrAReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.h());
        },
    },
    // 0xb5
    Instruction {
        mnemonic: Mnemonic::OrAReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.l());
        },
    },
    // 0xb6
    Instruction {
        mnemonic: Mnemonic::OrAHlAddr,
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::or(cpu, val);
        },
    },
    // 0xb7
    Instruction {
        mnemonic: Mnemonic::OrAReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.a());
        },
    },
    // 0xb8
    Instruction {
        mnemonic: Mnemonic::CpAReg8(Reg8::B),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.b());
        },
    },
    // 0xb9
    Instruction {
        mnemonic: Mnemonic::CpAReg8(Reg8::C),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.c());
        },
    },
    // 0xba
    Instruction {
        mnemonic: Mnemonic::CpAReg8(Reg8::D),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.d());
        },
    },
    // 0xbb
    Instruction {
        mnemonic: Mnemonic::CpAReg8(Reg8::E),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.e());
        },
    },
    // 0xbc
    Instruction {
        mnemonic: Mnemonic::CpAReg8(Reg8::H),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.h());
        },
    },
    // 0xbd
    Instruction {
        mnemonic: Mnemonic::CpAReg8(Reg8::L),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.l());
        },
    },
    // 0xbe
    Instruction {
        mnemonic: Mnemonic::CpAHlAddr,
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::cp(cpu, val);
        },
    },
    // 0xbf
    Instruction {
        mnemonic: Mnemonic::CpAReg8(Reg8::A),
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.a());
        },
    },
    // 0xc0
    Instruction {
        mnemonic: Mnemonic::RetCond(Cond::NZ),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: Some(3),
        operation: |cpu| {
            cpu.cycle();
            if !cpu.regs.flags().z() {
                helpers::ret(cpu);
                cpu.cycle();
            }
        },
    },
    // 0xc1
    Instruction {
        mnemonic: Mnemonic::Pop(Reg16::BC), // Mnemonic::Pop(Reg16::BC),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.stack_pop();
            cpu.regs.set_bc(val);
        },
    },
    // 0xc2
    Instruction {
        mnemonic: Mnemonic::JpCond(Cond::NZ, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: Some(1),
        operation: |cpu| {
            let val = cpu.fetch_word();
            if !cpu.regs.flags().z() {
                cpu.cycle();
                helpers::jp(cpu, val);
            }
        },
    },
    // 0xc3
    Instruction {
        mnemonic: Mnemonic::Jp(Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.cycle();
            helpers::jp(cpu, val);
        },
    },
    // 0xc4
    Instruction {
        mnemonic: Mnemonic::CallCond(Cond::NZ, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: Some(3),
        operation: |cpu| {
            let val = cpu.fetch_word();
            if !cpu.regs.flags().z() {
                cpu.cycle();
                helpers::call(cpu, val);
            }
        },
    },
    // 0xc5
    Instruction {
        mnemonic: Mnemonic::Push(Reg16::BC), // Mnemonic::Push(Reg16::BC),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.stack_push(cpu.regs.bc());
        },
    },
    // 0xc6
    Instruction {
        mnemonic: Mnemonic::AddAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::add(cpu, val);
        },
    },
    // 0xc7
    Instruction {
        mnemonic: Mnemonic::Rst(Imm::Known(0x00)),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0000);
        },
    },
    // 0xc8
    Instruction {
        mnemonic: Mnemonic::RetCond(Cond::Z),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: Some(3),
        operation: |cpu| {
            cpu.cycle();
            if cpu.regs.flags().z() {
                helpers::ret(cpu);
                cpu.cycle();
            }
        },
    },
    // 0xc9
    Instruction {
        mnemonic: Mnemonic::Ret,
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::ret(cpu);
            cpu.cycle();
        },
    },
    // 0xca
    Instruction {
        mnemonic: Mnemonic::JpCond(Cond::Z, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: Some(1),
        operation: |cpu| {
            let val = cpu.fetch_word();
            if cpu.regs.flags().z() {
                cpu.cycle();
                helpers::jp(cpu, val);
            }
        },
    },
    // 0xcb Invalid as it is a prefix.
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| helpers::invalid(),
    },
    // 0xcc
    Instruction {
        mnemonic: Mnemonic::CallCond(Cond::Z, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: Some(3),
        operation: |cpu| {
            let val = cpu.fetch_word();
            if cpu.regs.flags().z() {
                cpu.cycle();
                helpers::call(cpu, val);
            }
        },
    },
    // 0xcd
    Instruction {
        mnemonic: Mnemonic::Call(Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 6,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.cycle();
            helpers::call(cpu, val);
        },
    },
    // 0xce
    Instruction {
        mnemonic: Mnemonic::AdcAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::adc(cpu, val);
        },
    },
    // 0xcf
    Instruction {
        mnemonic: Mnemonic::Rst(Imm::Known(0x08)),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0008);
        },
    },
    // 0xd0
    Instruction {
        mnemonic: Mnemonic::RetCond(Cond::NC),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: Some(3),
        operation: |cpu| {
            cpu.cycle();
            if !cpu.regs.flags().c() {
                helpers::ret(cpu);
                cpu.cycle();
            }
        },
    },
    // 0xd1
    Instruction {
        mnemonic: Mnemonic::Pop(Reg16::DE), // Mnemonic::Pop(Reg16::DE),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.stack_pop();
            cpu.regs.set_de(val);
        },
    },
    // 0xd2
    Instruction {
        mnemonic: Mnemonic::JpCond(Cond::NC, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: Some(1),
        operation: |cpu| {
            let val = cpu.fetch_word();
            if !cpu.regs.flags().c() {
                cpu.cycle();
                helpers::jp(cpu, val);
            }
        },
    },
    // 0xd3
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xd4
    Instruction {
        mnemonic: Mnemonic::CallCond(Cond::NC, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: Some(3),
        operation: |cpu| {
            let val = cpu.fetch_word();
            if !cpu.regs.flags().c() {
                cpu.cycle();
                helpers::call(cpu, val);
            }
        },
    },
    // 0xd5
    Instruction {
        mnemonic: Mnemonic::Push(Reg16::DE),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.stack_push(cpu.regs.de());
        },
    },
    // 0xd6
    Instruction {
        mnemonic: Mnemonic::SubAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::sub(cpu, val);
        },
    },
    // 0xd7
    Instruction {
        mnemonic: Mnemonic::Rst(Imm::Known(0x10)),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0010);
        },
    },
    // 0xd8
    Instruction {
        mnemonic: Mnemonic::RetCond(Cond::C),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: Some(3),
        operation: |cpu| {
            cpu.cycle();
            if cpu.regs.flags().c() {
                helpers::ret(cpu);
                cpu.cycle();
            }
        },
    },
    // 0xd9
    Instruction {
        mnemonic: Mnemonic::Reti,
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::ret(cpu);
            cpu.cycle();
            cpu.ime = ImeState::Enabling;
        },
    },
    // 0xda
    Instruction {
        mnemonic: Mnemonic::JpCond(Cond::C, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: Some(1),
        operation: |cpu| {
            let val = cpu.fetch_word();
            if cpu.regs.flags().c() {
                cpu.cycle();
                helpers::jp(cpu, val);
            }
        },
    },
    // 0xdb
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xdc
    Instruction {
        mnemonic: Mnemonic::CallCond(Cond::C, Imm::Unknown),
        param_type: ParamType::Word,
        _cycles: 3,
        _branch_cycles: Some(3),
        operation: |cpu| {
            let val = cpu.fetch_word();
            if cpu.regs.flags().c() {
                cpu.cycle();
                helpers::call(cpu, val);
            }
        },
    },
    // 0xdd
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xde
    Instruction {
        mnemonic: Mnemonic::SbcAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::sbc(cpu, val);
        },
    },
    // 0xdf
    Instruction {
        mnemonic: Mnemonic::Rst(Imm::Known(0x18)),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0018);
        },
    },
    // 0xe0
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Imm8(Imm::Unknown), Reg8::A),
        param_type: ParamType::Byte,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let addr = 0xff00 + cpu.fetch_byte() as u16;
            cpu.write_byte(addr, cpu.regs.a());
        },
    },
    // 0xe1
    Instruction {
        mnemonic: Mnemonic::Pop(Reg16::HL),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.stack_pop();
            cpu.regs.set_hl(val);
        },
    },
    // 0xe2
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Reg8(Reg8::C), Reg8::A),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.write_byte(0xff00 + cpu.regs.c() as u16, cpu.regs.a());
        },
    },
    // 0xe3
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xe4
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xe5
    Instruction {
        mnemonic: Mnemonic::Push(Reg16::HL),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.stack_push(cpu.regs.hl());
        },
    },
    // 0xe6
    Instruction {
        mnemonic: Mnemonic::AndAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::and(cpu, val);
        },
    },
    // 0xe7
    Instruction {
        mnemonic: Mnemonic::Rst(Imm::Known(0x20)),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0020);
        },
    },
    // 0xe8
    Instruction {
        mnemonic: Mnemonic::AddSpImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte() as i8;
            let sp = helpers::add_sp(cpu, val);
            cpu.cycle();
            cpu.cycle();
            cpu.regs.set_sp(sp);
        },
    },
    // 0xe9
    Instruction {
        mnemonic: Mnemonic::JpHl,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            helpers::jp(cpu, cpu.regs.hl());
        },
    },
    // 0xea
    Instruction {
        mnemonic: Mnemonic::LdAddrReg8(Addr::Imm16(Imm::Unknown), Reg8::A),
        param_type: ParamType::Word,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            let addr = cpu.fetch_word();
            cpu.write_byte(addr, cpu.regs.a());
        },
    },
    // 0xeb
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xec
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xed
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xee
    Instruction {
        mnemonic: Mnemonic::XorAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::xor(cpu, val);
        },
    },
    // 0xef
    Instruction {
        mnemonic: Mnemonic::Rst(Imm::Known(0x28)),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0028);
        },
    },
    // 0xf0
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Imm8(Imm::Unknown)),
        param_type: ParamType::Byte,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let addr = 0xff00 + cpu.fetch_byte() as u16;
            let val = cpu.read_byte(addr);
            cpu.regs.set_a(val);
        },
    },
    // 0xf1
    Instruction {
        mnemonic: Mnemonic::Pop(Reg16::AF),
        param_type: ParamType::None,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.stack_pop();
            cpu.regs.set_af(val);
        },
    },
    // 0xf2
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Reg8(Reg8::C)),
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(0xff00 + cpu.regs.c() as u16);
            cpu.regs.set_a(val);
        },
    },
    // 0xf3
    Instruction {
        mnemonic: Mnemonic::Di,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.ime = ImeState::Disabled;
        },
    },
    // 0xf4
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xf5
    Instruction {
        mnemonic: Mnemonic::Push(Reg16::AF),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.stack_push(cpu.regs.af());
        },
    },
    // 0xf6
    Instruction {
        mnemonic: Mnemonic::OrAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::or(cpu, val);
        },
    },
    // 0xf7
    Instruction {
        mnemonic: Mnemonic::Rst(Imm::Known(0x30)),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0030);
        },
    },
    // 0xf8
    Instruction {
        mnemonic: Mnemonic::LdHlAddSpImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 3,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte() as i8;
            let hl = helpers::add_sp(cpu, val);
            cpu.cycle();
            cpu.regs.set_hl(hl);
        },
    },
    // 0xf9
    Instruction {
        mnemonic: Mnemonic::LdSpHl,
        param_type: ParamType::None,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_sp(cpu.regs.hl());
        },
    },
    // 0xfa
    Instruction {
        mnemonic: Mnemonic::LdReg8Addr(Reg8::A, Addr::Imm16(Imm::Unknown)),
        param_type: ParamType::Word,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            let addr = cpu.fetch_word();
            let val = cpu.read_byte(addr);
            cpu.regs.set_a(val);
        },
    },
    // 0xfb
    Instruction {
        mnemonic: Mnemonic::Ei,
        param_type: ParamType::None,
        _cycles: 1,
        _branch_cycles: None,
        operation: |cpu| {
            log::debug!("CPU: Enabling IME");
            cpu.ime = ImeState::Enabling;
        },
    },
    // 0xfc
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xfd
    Instruction {
        mnemonic: Mnemonic::Invalid,
        param_type: ParamType::None,
        _cycles: 0,
        _branch_cycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xfe
    Instruction {
        mnemonic: Mnemonic::CpAImm(Imm::Unknown),
        param_type: ParamType::Byte,
        _cycles: 2,
        _branch_cycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::cp(cpu, val);
        },
    },
    // 0xff
    Instruction {
        mnemonic: Mnemonic::Rst(Imm::Known(0x38)),
        param_type: ParamType::None,
        _cycles: 4,
        _branch_cycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0038);
        },
    },
];
