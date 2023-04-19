use crate::cpu::ImeState;

use super::{helpers, InstrSet, Instruction, ParamType, BITWISE_INSTRS};

use super::mnemonics::{Mnemonic, Opd, Param::*, Ptr::*};

pub const BASE_INSTRS: InstrSet = [
    // 0x00
    Instruction {
        mnemonic: Mnemonic("NOP", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x01
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("BC"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.regs.set_bc(val);
        },
    },
    // 0x02
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("BC")), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.bc(), cpu.regs.a());
        },
    },
    // 0x03
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("BC"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_bc(cpu.regs.bc().wrapping_add(1));
        },
    },
    // 0x04
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("B"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x05
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("B"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.b());
            cpu.regs.set_b(res);
        },
    },
    // 0x06
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("B"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_b(val);
        },
    },
    // 0x07
    Instruction {
        mnemonic: Mnemonic("RLCA", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::rlc(cpu, cpu.regs.a());
            cpu.regs.flags().set_z(false);
            cpu.regs.set_a(res);
        },
    },
    // 0x08
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Param(U16)), Opd::Fixed("SP")),
        param_type: ParamType::Word,
        cycles: 5,
        brcycles: None,
        operation: |cpu| {
            let addr = cpu.fetch_word();
            cpu.write_word(addr, cpu.regs.sp());
        },
    },
    // 0x09
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("HL"), Opd::Fixed("BC")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::add_hl(cpu, cpu.regs.bc());
        },
    },
    // 0x0a
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Ptr(Fixed("BC"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.bc());
            cpu.regs.set_a(val);
        },
    },
    // 0x0b
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("BC"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_bc(cpu.regs.bc().wrapping_sub(1));
        },
    },
    // 0x0c
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("C"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x0d
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("C"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.c());
            cpu.regs.set_c(res);
        },
    },
    // 0x0e
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("C"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_c(val);
        },
    },
    // 0x0f
    Instruction {
        mnemonic: Mnemonic("RRCA", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::rrc(cpu, cpu.regs.a());
            cpu.regs.flags().set_z(false);
            cpu.regs.set_a(res);
        },
    },
    // 0x10
    Instruction {
        mnemonic: Mnemonic("STOP", Opd::None, Opd::None),
        param_type: ParamType::Byte,
        cycles: 1,
        brcycles: None,
        operation: |_| {
            log::warn!("STOP instruction called, but it is not implemented properly yet.");
            // TODO
        },
    },
    // 0x11
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("DE"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.regs.set_de(val);
        },
    },
    // 0x12
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("DE")), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.de(), cpu.regs.a());
        },
    },
    // 0x13
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("DE"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_de(cpu.regs.de().wrapping_add(1));
        },
    },
    // 0x14
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("D"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x15
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("D"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.d());
            cpu.regs.set_d(res);
        },
    },
    // 0x16
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("D"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_d(val);
        },
    },
    // 0x17
    Instruction {
        mnemonic: Mnemonic("RLA", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::rl(cpu, cpu.regs.a());
            cpu.regs.flags().set_z(false);
            cpu.regs.set_a(res);
        },
    },
    // 0x18
    Instruction {
        mnemonic: Mnemonic("JR", Opd::Param(I8), Opd::None),
        param_type: ParamType::Byte,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte() as i8;
            cpu.cycle();
            helpers::jr(cpu, val);
        },
    },
    // 0x19
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("HL"), Opd::Fixed("DE")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::add_hl(cpu, cpu.regs.de());
        },
    },
    // 0x1a
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Ptr(Fixed("DE"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.de());
            cpu.regs.set_a(val);
        },
    },
    // 0x1b
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("DE"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_de(cpu.regs.de().wrapping_sub(1));
        },
    },
    // 0x1c
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("E"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x1d
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("E"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.e());
            cpu.regs.set_e(res);
        },
    },
    // 0x1e
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("E"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_e(val);
        },
    },
    // 0x1f
    Instruction {
        mnemonic: Mnemonic("RRA", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::rr(cpu, cpu.regs.a());
            cpu.regs.flags().set_z(false);
            cpu.regs.set_a(res);
        },
    },
    // 0x20
    Instruction {
        mnemonic: Mnemonic("JR", Opd::Fixed("NZ"), Opd::Param(I8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: Some(1),
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
        mnemonic: Mnemonic("LD", Opd::Fixed("HL"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.regs.set_hl(val);
        },
    },
    // 0x22
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("HL+")), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let hl = cpu.regs.inc_hl();
            cpu.write_byte(hl, cpu.regs.a());
        },
    },
    // 0x23
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("HL"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_hl(cpu.regs.hl().wrapping_add(1));
        },
    },
    // 0x24
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("H"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x25
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("H"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.h());
            cpu.regs.set_h(res);
        },
    },
    // 0x26
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("H"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_h(val);
        },
    },
    // 0x27
    Instruction {
        mnemonic: Mnemonic("DAA", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::daa(cpu);
        },
    },
    // 0x28
    Instruction {
        mnemonic: Mnemonic("JR", Opd::Fixed("Z"), Opd::Param(I8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: Some(1),
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
        mnemonic: Mnemonic("ADD", Opd::Fixed("HL"), Opd::Fixed("HL")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::add_hl(cpu, cpu.regs.hl());
        },
    },
    // 0x2a
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Ptr(Fixed("HL+"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let hl = cpu.regs.inc_hl();
            let val = cpu.read_byte(hl);
            cpu.regs.set_a(val);
        },
    },
    // 0x2b
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("HL"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_hl(cpu.regs.hl().wrapping_sub(1));
        },
    },
    // 0x2c
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("L"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x2d
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("L"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.l());
            cpu.regs.set_l(res);
        },
    },
    // 0x2e
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("L"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_l(val);
        },
    },
    // 0x2f
    Instruction {
        mnemonic: Mnemonic("CPL", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cpl(cpu);
        },
    },
    // 0x30
    Instruction {
        mnemonic: Mnemonic("JR", Opd::Fixed("NC"), Opd::Param(I8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: Some(1),
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
        mnemonic: Mnemonic("LD", Opd::Fixed("SP"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.regs.set_sp(val);
        },
    },
    // 0x32
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("HL-")), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let hl = cpu.regs.dec_hl();
            cpu.write_byte(hl, cpu.regs.a());
        },
    },
    // 0x33
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("SP"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_sp(cpu.regs.sp().wrapping_add(1));
        },
    },
    // 0x34
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Ptr(Fixed("HL")), Opd::None),
        param_type: ParamType::None,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::inc(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x35
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Ptr(Fixed("HL")), Opd::None),
        param_type: ParamType::None,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            let res = helpers::dec(cpu, val);
            cpu.write_byte(cpu.regs.hl(), res);
        },
    },
    // 0x36
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("HL")), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.write_byte(cpu.regs.hl(), val);
        },
    },
    // 0x37
    Instruction {
        mnemonic: Mnemonic("SCF", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::scf(cpu);
        },
    },
    // 0x38
    Instruction {
        mnemonic: Mnemonic("JR", Opd::Fixed("C"), Opd::Param(I8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: Some(1),
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
        mnemonic: Mnemonic("ADD", Opd::Fixed("HL"), Opd::Fixed("SP")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::add_hl(cpu, cpu.regs.sp());
        },
    },
    // 0x3a
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Ptr(Fixed("HL-"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let hl = cpu.regs.dec_hl();
            let val = cpu.read_byte(hl);
            cpu.regs.set_a(val);
        },
    },
    // 0x3b
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("SP"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_sp(cpu.regs.sp().wrapping_sub(1));
        },
    },
    // 0x3c
    Instruction {
        mnemonic: Mnemonic("INC", Opd::Fixed("A"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::inc(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x3d
    Instruction {
        mnemonic: Mnemonic("DEC", Opd::Fixed("A"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            let res = helpers::dec(cpu, cpu.regs.a());
            cpu.regs.set_a(res);
        },
    },
    // 0x3e
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            cpu.regs.set_a(val);
        },
    },
    // 0x3f
    Instruction {
        mnemonic: Mnemonic("CCF", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::ccf(cpu);
        },
    },
    // 0x40
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("B"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x41
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("B"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.c());
        },
    },
    // 0x42
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("B"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.d());
        },
    },
    // 0x43
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("B"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.e());
        },
    },
    // 0x44
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("B"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.h());
        },
    },
    // 0x45
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("B"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.l());
        },
    },
    // 0x46
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("B"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_b(val);
        },
    },
    // 0x47
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("B"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.a());
        },
    },
    // 0x48
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("C"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.b());
        },
    },
    // 0x49
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("C"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x4a
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("C"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.d());
        },
    },
    // 0x4b
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("C"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.e());
        },
    },
    // 0x4c
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("C"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.h());
        },
    },
    // 0x4d
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("C"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.l());
        },
    },
    // 0x4e
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("C"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_c(val);
        },
    },
    // 0x4f
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("C"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.a());
        },
    },
    // 0x50
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("D"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.b());
        },
    },
    // 0x51
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("D"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.c());
        },
    },
    // 0x52
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("D"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x53
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("D"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.e());
        },
    },
    // 0x54
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("D"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.h());
        },
    },
    // 0x55
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("D"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.l());
        },
    },
    // 0x56
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("D"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_d(val);
        },
    },
    // 0x57
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("D"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.a());
        },
    },
    // 0x58
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("E"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.b());
        },
    },
    // 0x59
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("E"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.c());
        },
    },
    // 0x5a
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("E"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.d());
        },
    },
    // 0x5b
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("E"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x5c
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("E"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.h());
        },
    },
    // 0x5d
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("E"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.l());
        },
    },
    // 0x5e
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("E"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_e(val);
        },
    },
    // 0x5f
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("E"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.a());
        },
    },
    // 0x60
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("H"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.b());
        },
    },
    // 0x61
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("H"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.c());
        },
    },
    // 0x62
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("H"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.d());
        },
    },
    // 0x63
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("H"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.e());
        },
    },
    // 0x64
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("H"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x65
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("H"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.l());
        },
    },
    // 0x66
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("H"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_h(val);
        },
    },
    // 0x67
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("H"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.a());
        },
    },
    // 0x68
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("L"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.b());
        },
    },
    // 0x69
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("L"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.c());
        },
    },
    // 0x6a
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("L"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.d());
        },
    },
    // 0x6b
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("L"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.e());
        },
    },
    // 0x6c
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("L"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.h());
        },
    },
    // 0x6d
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("L"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x6e
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("L"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_l(val);
        },
    },
    // 0x6f
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("L"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.a());
        },
    },
    // 0x70
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("HL")), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.b());
        },
    },
    // 0x71
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("HL")), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.c());
        },
    },
    // 0x72
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("HL")), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.d());
        },
    },
    // 0x73
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("HL")), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.e());
        },
    },
    // 0x74
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("HL")), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.h());
        },
    },
    // 0x75
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("HL")), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.l());
        },
    },
    // 0x76
    Instruction {
        mnemonic: Mnemonic("HALT", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            log::debug!("CPU: Halted");
            cpu.halted = true;
        },
    },
    // 0x77
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Fixed("HL")), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.a());
        },
    },
    // 0x78
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.b());
        },
    },
    // 0x79
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.c());
        },
    },
    // 0x7a
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.d());
        },
    },
    // 0x7b
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.e());
        },
    },
    // 0x7c
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.h());
        },
    },
    // 0x7d
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.l());
        },
    },
    // 0x7e
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            cpu.regs.set_a(val);
        },
    },
    // 0x7f
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x80
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("A"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.b());
        },
    },
    // 0x81
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("A"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.c());
        },
    },
    // 0x82
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("A"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.d());
        },
    },
    // 0x83
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("A"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.e());
        },
    },
    // 0x84
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("A"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.h());
        },
    },
    // 0x85
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("A"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.l());
        },
    },
    // 0x86
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("A"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::add(cpu, val);
        },
    },
    // 0x87
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("A"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.a());
        },
    },
    // 0x88
    Instruction {
        mnemonic: Mnemonic("ADC", Opd::Fixed("A"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.b());
        },
    },
    // 0x89
    Instruction {
        mnemonic: Mnemonic("ADC", Opd::Fixed("A"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.c());
        },
    },
    // 0x8a
    Instruction {
        mnemonic: Mnemonic("ADC", Opd::Fixed("A"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.d());
        },
    },
    // 0x8b
    Instruction {
        mnemonic: Mnemonic("ADC", Opd::Fixed("A"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.e());
        },
    },
    // 0x8c
    Instruction {
        mnemonic: Mnemonic("ADC", Opd::Fixed("A"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.h());
        },
    },
    // 0x8d
    Instruction {
        mnemonic: Mnemonic("ADC", Opd::Fixed("A"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.l());
        },
    },
    // 0x8e
    Instruction {
        mnemonic: Mnemonic("ADC", Opd::Fixed("A"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::adc(cpu, val);
        },
    },
    // 0x8f
    Instruction {
        mnemonic: Mnemonic("ADC", Opd::Fixed("A"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.a());
        },
    },
    // 0x90
    Instruction {
        mnemonic: Mnemonic("SUB", Opd::Fixed("A"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.b());
        },
    },
    // 0x91
    Instruction {
        mnemonic: Mnemonic("SUB", Opd::Fixed("A"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.c());
        },
    },
    // 0x92
    Instruction {
        mnemonic: Mnemonic("SUB", Opd::Fixed("A"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.d());
        },
    },
    // 0x93
    Instruction {
        mnemonic: Mnemonic("SUB", Opd::Fixed("A"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.e());
        },
    },
    // 0x94
    Instruction {
        mnemonic: Mnemonic("SUB", Opd::Fixed("A"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.h());
        },
    },
    // 0x95
    Instruction {
        mnemonic: Mnemonic("SUB", Opd::Fixed("A"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.l());
        },
    },
    // 0x96
    Instruction {
        mnemonic: Mnemonic("SUB", Opd::Fixed("A"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::sub(cpu, val);
        },
    },
    // 0x97
    Instruction {
        mnemonic: Mnemonic("SUB", Opd::Fixed("A"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.a());
        },
    },
    // 0x98
    Instruction {
        mnemonic: Mnemonic("SBC", Opd::Fixed("A"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.b());
        },
    },
    // 0x99
    Instruction {
        mnemonic: Mnemonic("SBC", Opd::Fixed("A"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.c());
        },
    },
    // 0x9a
    Instruction {
        mnemonic: Mnemonic("SBC", Opd::Fixed("A"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.d());
        },
    },
    // 0x9b
    Instruction {
        mnemonic: Mnemonic("SBC", Opd::Fixed("A"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.e());
        },
    },
    // 0x9c
    Instruction {
        mnemonic: Mnemonic("SBC", Opd::Fixed("A"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.h());
        },
    },
    // 0x9d
    Instruction {
        mnemonic: Mnemonic("SBC", Opd::Fixed("A"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.l());
        },
    },
    // 0x9e
    Instruction {
        mnemonic: Mnemonic("SBC", Opd::Fixed("A"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::sbc(cpu, val);
        },
    },
    // 0x9f
    Instruction {
        mnemonic: Mnemonic("SBC", Opd::Fixed("A"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.a());
        },
    },
    // 0xa0
    Instruction {
        mnemonic: Mnemonic("AND", Opd::Fixed("A"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.b());
        },
    },
    // 0xa1
    Instruction {
        mnemonic: Mnemonic("AND", Opd::Fixed("A"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.c());
        },
    },
    // 0xa2
    Instruction {
        mnemonic: Mnemonic("AND", Opd::Fixed("A"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.d());
        },
    },
    // 0xa3
    Instruction {
        mnemonic: Mnemonic("AND", Opd::Fixed("A"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.e());
        },
    },
    // 0xa4
    Instruction {
        mnemonic: Mnemonic("AND", Opd::Fixed("A"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.h());
        },
    },
    // 0xa5
    Instruction {
        mnemonic: Mnemonic("AND", Opd::Fixed("A"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.l());
        },
    },
    // 0xa6
    Instruction {
        mnemonic: Mnemonic("AND", Opd::Fixed("A"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::and(cpu, val);
        },
    },
    // 0xa7
    Instruction {
        mnemonic: Mnemonic("AND", Opd::Fixed("A"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.a());
        },
    },
    // 0xa8
    Instruction {
        mnemonic: Mnemonic("XOR", Opd::Fixed("A"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.b());
        },
    },
    // 0xa9
    Instruction {
        mnemonic: Mnemonic("XOR", Opd::Fixed("A"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.c());
        },
    },
    // 0xaa
    Instruction {
        mnemonic: Mnemonic("XOR", Opd::Fixed("A"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.d());
        },
    },
    // 0xab
    Instruction {
        mnemonic: Mnemonic("XOR", Opd::Fixed("A"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.e());
        },
    },
    // 0xac
    Instruction {
        mnemonic: Mnemonic("XOR", Opd::Fixed("A"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.h());
        },
    },
    // 0xad
    Instruction {
        mnemonic: Mnemonic("XOR", Opd::Fixed("A"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.l());
        },
    },
    // 0xae
    Instruction {
        mnemonic: Mnemonic("XOR", Opd::Fixed("A"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::xor(cpu, val);
        },
    },
    // 0xaf
    Instruction {
        mnemonic: Mnemonic("XOR", Opd::Fixed("A"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.a());
        },
    },
    // 0xb0
    Instruction {
        mnemonic: Mnemonic("OR", Opd::Fixed("A"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.b());
        },
    },
    // 0xb1
    Instruction {
        mnemonic: Mnemonic("OR", Opd::Fixed("A"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.c());
        },
    },
    // 0xb2
    Instruction {
        mnemonic: Mnemonic("OR", Opd::Fixed("A"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.d());
        },
    },
    // 0xb3
    Instruction {
        mnemonic: Mnemonic("OR", Opd::Fixed("A"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.e());
        },
    },
    // 0xb4
    Instruction {
        mnemonic: Mnemonic("OR", Opd::Fixed("A"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.h());
        },
    },
    // 0xb5
    Instruction {
        mnemonic: Mnemonic("OR", Opd::Fixed("A"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.l());
        },
    },
    // 0xb6
    Instruction {
        mnemonic: Mnemonic("OR", Opd::Fixed("A"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::or(cpu, val);
        },
    },
    // 0xb7
    Instruction {
        mnemonic: Mnemonic("OR", Opd::Fixed("A"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.a());
        },
    },
    // 0xb8
    Instruction {
        mnemonic: Mnemonic("CP", Opd::Fixed("A"), Opd::Fixed("B")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.b());
        },
    },
    // 0xb9
    Instruction {
        mnemonic: Mnemonic("CP", Opd::Fixed("A"), Opd::Fixed("C")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.c());
        },
    },
    // 0xba
    Instruction {
        mnemonic: Mnemonic("CP", Opd::Fixed("A"), Opd::Fixed("D")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.d());
        },
    },
    // 0xbb
    Instruction {
        mnemonic: Mnemonic("CP", Opd::Fixed("A"), Opd::Fixed("E")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.e());
        },
    },
    // 0xbc
    Instruction {
        mnemonic: Mnemonic("CP", Opd::Fixed("A"), Opd::Fixed("H")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.h());
        },
    },
    // 0xbd
    Instruction {
        mnemonic: Mnemonic("CP", Opd::Fixed("A"), Opd::Fixed("L")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.l());
        },
    },
    // 0xbe
    Instruction {
        mnemonic: Mnemonic("CP", Opd::Fixed("A"), Opd::Ptr(Fixed("HL"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(cpu.regs.hl());
            helpers::cp(cpu, val);
        },
    },
    // 0xbf
    Instruction {
        mnemonic: Mnemonic("CP", Opd::Fixed("A"), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.a());
        },
    },
    // 0xc0
    Instruction {
        mnemonic: Mnemonic("RET", Opd::Fixed("NZ"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: Some(3),
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
        mnemonic: Mnemonic("POP", Opd::Fixed("BC"), Opd::None),
        param_type: ParamType::None,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.stack_pop();
            cpu.regs.set_bc(val);
        },
    },
    // 0xc2
    Instruction {
        mnemonic: Mnemonic("JP", Opd::Fixed("NZ"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: Some(1),
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
        mnemonic: Mnemonic("JP", Opd::Param(U16), Opd::None),
        param_type: ParamType::Word,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.cycle();
            helpers::jp(cpu, val);
        },
    },
    // 0xc4
    Instruction {
        mnemonic: Mnemonic("CALL", Opd::Fixed("NZ"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: Some(3),
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
        mnemonic: Mnemonic("PUSH", Opd::Fixed("BC"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.stack_push(cpu.regs.bc());
        },
    },
    // 0xc6
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("A"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::add(cpu, val);
        },
    },
    // 0xc7
    Instruction {
        mnemonic: Mnemonic("RST", Opd::Fixed("00H"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0000);
        },
    },
    // 0xc8
    Instruction {
        mnemonic: Mnemonic("RET", Opd::Fixed("Z"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: Some(3),
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
        mnemonic: Mnemonic("RET", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            helpers::ret(cpu);
            cpu.cycle();
        },
    },
    // 0xca
    Instruction {
        mnemonic: Mnemonic("JP", Opd::Fixed("Z"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: Some(1),
        operation: |cpu| {
            let val = cpu.fetch_word();
            if cpu.regs.flags().z() {
                cpu.cycle();
                helpers::jp(cpu, val);
            }
        },
    },
    // 0xcb
    Instruction {
        mnemonic: Mnemonic("PREFIX", Opd::Fixed("CB"), Opd::None),
        param_type: ParamType::None,
        cycles: 0,
        brcycles: None,
        operation: |cpu| cpu.execute(BITWISE_INSTRS),
    },
    // 0xcc
    Instruction {
        mnemonic: Mnemonic("CALL", Opd::Fixed("Z"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: Some(3),
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
        mnemonic: Mnemonic("CALL", Opd::Param(U16), Opd::None),
        param_type: ParamType::Word,
        cycles: 6,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_word();
            cpu.cycle();
            helpers::call(cpu, val);
        },
    },
    // 0xce
    Instruction {
        mnemonic: Mnemonic("ADC", Opd::Fixed("A"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::adc(cpu, val);
        },
    },
    // 0xcf
    Instruction {
        mnemonic: Mnemonic("RST", Opd::Fixed("08H"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0008);
        },
    },
    // 0xd0
    Instruction {
        mnemonic: Mnemonic("RET", Opd::Fixed("NC"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: Some(3),
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
        mnemonic: Mnemonic("POP", Opd::Fixed("DE"), Opd::None),
        param_type: ParamType::None,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.stack_pop();
            cpu.regs.set_de(val);
        },
    },
    // 0xd2
    Instruction {
        mnemonic: Mnemonic("JP", Opd::Fixed("NC"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: Some(1),
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
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xd4
    Instruction {
        mnemonic: Mnemonic("CALL", Opd::Fixed("NC"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: Some(3),
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
        mnemonic: Mnemonic("PUSH", Opd::Fixed("DE"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.stack_push(cpu.regs.de());
        },
    },
    // 0xd6
    Instruction {
        mnemonic: Mnemonic("SUB", Opd::Fixed("A"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::sub(cpu, val);
        },
    },
    // 0xd7
    Instruction {
        mnemonic: Mnemonic("RST", Opd::Fixed("10H"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0010);
        },
    },
    // 0xd8
    Instruction {
        mnemonic: Mnemonic("RET", Opd::Fixed("C"), Opd::None),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: Some(3),
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
        mnemonic: Mnemonic("RETI", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            helpers::ret(cpu);
            cpu.cycle();
            cpu.ime = ImeState::Enabling;
        },
    },
    // 0xda
    Instruction {
        mnemonic: Mnemonic("JP", Opd::Fixed("C"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: Some(1),
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
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xdc
    Instruction {
        mnemonic: Mnemonic("CALL", Opd::Fixed("C"), Opd::Param(U16)),
        param_type: ParamType::Word,
        cycles: 3,
        brcycles: Some(3),
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
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xde
    Instruction {
        mnemonic: Mnemonic("SBC", Opd::Fixed("A"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::sbc(cpu, val);
        },
    },
    // 0xdf
    Instruction {
        mnemonic: Mnemonic("RST", Opd::Fixed("18H"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0018);
        },
    },
    // 0xe0
    Instruction {
        mnemonic: Mnemonic("LDH", Opd::Ptr(Param(U8)), Opd::Fixed("A")),
        param_type: ParamType::Byte,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let addr = 0xff00 + cpu.fetch_byte() as u16;
            cpu.write_byte(addr, cpu.regs.a());
        },
    },
    // 0xe1
    Instruction {
        mnemonic: Mnemonic("POP", Opd::Fixed("HL"), Opd::None),
        param_type: ParamType::None,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.stack_pop();
            cpu.regs.set_hl(val);
        },
    },
    // 0xe2
    Instruction {
        mnemonic: Mnemonic("LDH", Opd::Ptr(Fixed("C")), Opd::Fixed("A")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(0xff00 + cpu.regs.c() as u16, cpu.regs.a());
        },
    },
    // 0xe3
    Instruction {
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xe4
    Instruction {
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xe5
    Instruction {
        mnemonic: Mnemonic("PUSH", Opd::Fixed("HL"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.stack_push(cpu.regs.hl());
        },
    },
    // 0xe6
    Instruction {
        mnemonic: Mnemonic("AND", Opd::Fixed("A"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::and(cpu, val);
        },
    },
    // 0xe7
    Instruction {
        mnemonic: Mnemonic("RST", Opd::Fixed("20H"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0020);
        },
    },
    // 0xe8
    Instruction {
        mnemonic: Mnemonic("ADD", Opd::Fixed("SP"), Opd::Param(I8)),
        param_type: ParamType::Byte,
        cycles: 4,
        brcycles: None,
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
        mnemonic: Mnemonic("JP", Opd::Fixed("HL"), Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::jp(cpu, cpu.regs.hl());
        },
    },
    // 0xea
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Ptr(Param(U16)), Opd::Fixed("A")),
        param_type: ParamType::Word,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            let addr = cpu.fetch_word();
            cpu.write_byte(addr, cpu.regs.a());
        },
    },
    // 0xeb
    Instruction {
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xec
    Instruction {
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xed
    Instruction {
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xee
    Instruction {
        mnemonic: Mnemonic("XOR", Opd::Fixed("A"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::xor(cpu, val);
        },
    },
    // 0xef
    Instruction {
        mnemonic: Mnemonic("RST", Opd::Fixed("28H"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0028);
        },
    },
    // 0xf0
    Instruction {
        mnemonic: Mnemonic("LDH", Opd::Fixed("A"), Opd::Ptr(Param(U8))),
        param_type: ParamType::Byte,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let addr = 0xff00 + cpu.fetch_byte() as u16;
            let val = cpu.read_byte(addr);
            cpu.regs.set_a(val);
        },
    },
    // 0xf1
    Instruction {
        mnemonic: Mnemonic("POP", Opd::Fixed("AF"), Opd::None),
        param_type: ParamType::None,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.stack_pop();
            cpu.regs.set_af(val);
        },
    },
    // 0xf2
    Instruction {
        mnemonic: Mnemonic("LDH", Opd::Fixed("A"), Opd::Ptr(Fixed("C"))),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.read_byte(0xff00 + cpu.regs.c() as u16);
            cpu.regs.set_a(val);
        },
    },
    // 0xf3
    Instruction {
        mnemonic: Mnemonic("DI", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.ime = ImeState::Disabled;
        },
    },
    // 0xf4
    Instruction {
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xf5
    Instruction {
        mnemonic: Mnemonic("PUSH", Opd::Fixed("AF"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.stack_push(cpu.regs.af());
        },
    },
    // 0xf6
    Instruction {
        mnemonic: Mnemonic("OR", Opd::Fixed("A"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::or(cpu, val);
        },
    },
    // 0xf7
    Instruction {
        mnemonic: Mnemonic("RST", Opd::Fixed("30H"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0030);
        },
    },
    // 0xf8
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("HL"), Opd::Param(SPI8)),
        param_type: ParamType::Byte,
        cycles: 3,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte() as i8;
            let hl = helpers::add_sp(cpu, val);
            cpu.cycle();
            cpu.regs.set_hl(hl);
        },
    },
    // 0xf9
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("SP"), Opd::Fixed("HL")),
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            cpu.regs.set_sp(cpu.regs.hl());
        },
    },
    // 0xfa
    Instruction {
        mnemonic: Mnemonic("LD", Opd::Fixed("A"), Opd::Ptr(Param(U16))),
        param_type: ParamType::Word,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            let addr = cpu.fetch_word();
            let val = cpu.read_byte(addr);
            cpu.regs.set_a(val);
        },
    },
    // 0xfb
    Instruction {
        mnemonic: Mnemonic("EI", Opd::None, Opd::None),
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            log::debug!("CPU: Enabling IME");
            cpu.ime = ImeState::Enabling;
        },
    },
    // 0xfc
    Instruction {
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xfd
    Instruction {
        mnemonic: Mnemonic("INVALID", Opd::None, Opd::None),
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xfe
    Instruction {
        mnemonic: Mnemonic("CP", Opd::Fixed("A"), Opd::Param(U8)),
        param_type: ParamType::Byte,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            let val = cpu.fetch_byte();
            helpers::cp(cpu, val);
        },
    },
    // 0xff
    Instruction {
        mnemonic: Mnemonic("RST", Opd::Fixed("38H"), Opd::None),
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0038);
        },
    },
];
