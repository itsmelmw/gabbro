use crate::cpu::ImeState;

use super::{helpers, InstrSet, Instruction, ParamType, BITWISE_INSTRS};

pub const BASE_INSTRS: InstrSet = [
    // 0x00
    Instruction {
        mnemonic: "NOP",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x01
    Instruction {
        mnemonic: "LD BC,u16",
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
        mnemonic: "LD (BC),A",
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.bc(), cpu.regs.a());
        },
    },
    // 0x03
    Instruction {
        mnemonic: "INC BC",
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
        mnemonic: "INC B",
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
        mnemonic: "DEC B",
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
        mnemonic: "LD B,u8",
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
        mnemonic: "RLCA",
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
        mnemonic: "LD (u16),SP",
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
        mnemonic: "ADD HL,BC",
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
        mnemonic: "LD A,(BC)",
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
        mnemonic: "DEC BC",
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
        mnemonic: "INC C",
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
        mnemonic: "DEC C",
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
        mnemonic: "LD C,u8",
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
        mnemonic: "RRCA",
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
        mnemonic: "STOP",
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
        mnemonic: "LD DE,u16",
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
        mnemonic: "LD (DE),A",
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.de(), cpu.regs.a());
        },
    },
    // 0x13
    Instruction {
        mnemonic: "INC DE",
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
        mnemonic: "INC D",
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
        mnemonic: "DEC D",
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
        mnemonic: "LD D,u8",
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
        mnemonic: "RLA",
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
        mnemonic: "JR i8",
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
        mnemonic: "ADD HL,DE",
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
        mnemonic: "LD A,(DE)",
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
        mnemonic: "DEC DE",
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
        mnemonic: "INC E",
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
        mnemonic: "DEC E",
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
        mnemonic: "LD E,u8",
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
        mnemonic: "RRA",
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
        mnemonic: "JR NZ,i8",
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
        mnemonic: "LD HL,u16",
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
        mnemonic: "LD (HL+),A",
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
        mnemonic: "INC HL",
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
        mnemonic: "INC H",
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
        mnemonic: "DEC H",
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
        mnemonic: "LD H,u8",
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
        mnemonic: "DAA",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::daa(cpu);
        },
    },
    // 0x28
    Instruction {
        mnemonic: "JR Z,i8",
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
        mnemonic: "ADD HL,HL",
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
        mnemonic: "LD A,(HL+)",
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
        mnemonic: "DEC HL",
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
        mnemonic: "INC L",
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
        mnemonic: "DEC L",
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
        mnemonic: "LD L,u8",
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
        mnemonic: "CPL",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cpl(cpu);
        },
    },
    // 0x30
    Instruction {
        mnemonic: "JR NC,i8",
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
        mnemonic: "LD SP,u16",
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
        mnemonic: "LD (HL-),A",
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
        mnemonic: "INC SP",
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
        mnemonic: "INC (HL)",
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
        mnemonic: "DEC (HL)",
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
        mnemonic: "LD (HL),u8",
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
        mnemonic: "SCF",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::scf(cpu);
        },
    },
    // 0x38
    Instruction {
        mnemonic: "JR C,i8",
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
        mnemonic: "ADD HL,SP",
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
        mnemonic: "LD A,(HL-)",
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
        mnemonic: "DEC SP",
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
        mnemonic: "INC A",
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
        mnemonic: "DEC A",
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
        mnemonic: "LD A,u8",
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
        mnemonic: "CCF",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::ccf(cpu);
        },
    },
    // 0x40
    Instruction {
        mnemonic: "LD B,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x41
    Instruction {
        mnemonic: "LD B,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.c());
        },
    },
    // 0x42
    Instruction {
        mnemonic: "LD B,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.d());
        },
    },
    // 0x43
    Instruction {
        mnemonic: "LD B,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.e());
        },
    },
    // 0x44
    Instruction {
        mnemonic: "LD B,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.h());
        },
    },
    // 0x45
    Instruction {
        mnemonic: "LD B,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.l());
        },
    },
    // 0x46
    Instruction {
        mnemonic: "LD B,(HL)",
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
        mnemonic: "LD B,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_b(cpu.regs.a());
        },
    },
    // 0x48
    Instruction {
        mnemonic: "LD C,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.b());
        },
    },
    // 0x49
    Instruction {
        mnemonic: "LD C,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x4a
    Instruction {
        mnemonic: "LD C,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.d());
        },
    },
    // 0x4b
    Instruction {
        mnemonic: "LD C,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.e());
        },
    },
    // 0x4c
    Instruction {
        mnemonic: "LD C,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.h());
        },
    },
    // 0x4d
    Instruction {
        mnemonic: "LD C,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.l());
        },
    },
    // 0x4e
    Instruction {
        mnemonic: "LD C,(HL)",
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
        mnemonic: "LD C,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_c(cpu.regs.a());
        },
    },
    // 0x50
    Instruction {
        mnemonic: "LD D,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.b());
        },
    },
    // 0x51
    Instruction {
        mnemonic: "LD D,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.c());
        },
    },
    // 0x52
    Instruction {
        mnemonic: "LD D,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x53
    Instruction {
        mnemonic: "LD D,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.e());
        },
    },
    // 0x54
    Instruction {
        mnemonic: "LD D,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.h());
        },
    },
    // 0x55
    Instruction {
        mnemonic: "LD D,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.l());
        },
    },
    // 0x56
    Instruction {
        mnemonic: "LD D,(HL)",
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
        mnemonic: "LD D,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_d(cpu.regs.a());
        },
    },
    // 0x58
    Instruction {
        mnemonic: "LD E,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.b());
        },
    },
    // 0x59
    Instruction {
        mnemonic: "LD E,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.c());
        },
    },
    // 0x5a
    Instruction {
        mnemonic: "LD E,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.d());
        },
    },
    // 0x5b
    Instruction {
        mnemonic: "LD E,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x5c
    Instruction {
        mnemonic: "LD E,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.h());
        },
    },
    // 0x5d
    Instruction {
        mnemonic: "LD E,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.l());
        },
    },
    // 0x5e
    Instruction {
        mnemonic: "LD E,(HL)",
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
        mnemonic: "LD E,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_e(cpu.regs.a());
        },
    },
    // 0x60
    Instruction {
        mnemonic: "LD H,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.b());
        },
    },
    // 0x61
    Instruction {
        mnemonic: "LD H,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.c());
        },
    },
    // 0x62
    Instruction {
        mnemonic: "LD H,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.d());
        },
    },
    // 0x63
    Instruction {
        mnemonic: "LD H,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.e());
        },
    },
    // 0x64
    Instruction {
        mnemonic: "LD H,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x65
    Instruction {
        mnemonic: "LD H,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.l());
        },
    },
    // 0x66
    Instruction {
        mnemonic: "LD H,(HL)",
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
        mnemonic: "LD H,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_h(cpu.regs.a());
        },
    },
    // 0x68
    Instruction {
        mnemonic: "LD L,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.b());
        },
    },
    // 0x69
    Instruction {
        mnemonic: "LD L,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.c());
        },
    },
    // 0x6a
    Instruction {
        mnemonic: "LD L,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.d());
        },
    },
    // 0x6b
    Instruction {
        mnemonic: "LD L,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.e());
        },
    },
    // 0x6c
    Instruction {
        mnemonic: "LD L,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.h());
        },
    },
    // 0x6d
    Instruction {
        mnemonic: "LD L,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x6e
    Instruction {
        mnemonic: "LD L,(HL)",
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
        mnemonic: "LD L,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_l(cpu.regs.a());
        },
    },
    // 0x70
    Instruction {
        mnemonic: "LD (HL),B",
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.b());
        },
    },
    // 0x71
    Instruction {
        mnemonic: "LD (HL),C",
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.c());
        },
    },
    // 0x72
    Instruction {
        mnemonic: "LD (HL),D",
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.d());
        },
    },
    // 0x73
    Instruction {
        mnemonic: "LD (HL),E",
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.e());
        },
    },
    // 0x74
    Instruction {
        mnemonic: "LD (HL),H",
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.h());
        },
    },
    // 0x75
    Instruction {
        mnemonic: "LD (HL),L",
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.l());
        },
    },
    // 0x76
    Instruction {
        mnemonic: "HALT",
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
        mnemonic: "LD (HL),A",
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(cpu.regs.hl(), cpu.regs.a());
        },
    },
    // 0x78
    Instruction {
        mnemonic: "LD A,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.b());
        },
    },
    // 0x79
    Instruction {
        mnemonic: "LD A,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.c());
        },
    },
    // 0x7a
    Instruction {
        mnemonic: "LD A,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.d());
        },
    },
    // 0x7b
    Instruction {
        mnemonic: "LD A,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.e());
        },
    },
    // 0x7c
    Instruction {
        mnemonic: "LD A,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.h());
        },
    },
    // 0x7d
    Instruction {
        mnemonic: "LD A,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.regs.set_a(cpu.regs.l());
        },
    },
    // 0x7e
    Instruction {
        mnemonic: "LD A,(HL)",
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
        mnemonic: "LD A,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |_| {},
    },
    // 0x80
    Instruction {
        mnemonic: "ADD A,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.b());
        },
    },
    // 0x81
    Instruction {
        mnemonic: "ADD A,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.c());
        },
    },
    // 0x82
    Instruction {
        mnemonic: "ADD A,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.d());
        },
    },
    // 0x83
    Instruction {
        mnemonic: "ADD A,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.e());
        },
    },
    // 0x84
    Instruction {
        mnemonic: "ADD A,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.h());
        },
    },
    // 0x85
    Instruction {
        mnemonic: "ADD A,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.l());
        },
    },
    // 0x86
    Instruction {
        mnemonic: "ADD A,(HL)",
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
        mnemonic: "ADD A,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::add(cpu, cpu.regs.a());
        },
    },
    // 0x88
    Instruction {
        mnemonic: "ADC A,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.b());
        },
    },
    // 0x89
    Instruction {
        mnemonic: "ADC A,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.c());
        },
    },
    // 0x8a
    Instruction {
        mnemonic: "ADC A,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.d());
        },
    },
    // 0x8b
    Instruction {
        mnemonic: "ADC A,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.e());
        },
    },
    // 0x8c
    Instruction {
        mnemonic: "ADC A,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.h());
        },
    },
    // 0x8d
    Instruction {
        mnemonic: "ADC A,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.l());
        },
    },
    // 0x8e
    Instruction {
        mnemonic: "ADC A,(HL)",
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
        mnemonic: "ADC A,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::adc(cpu, cpu.regs.a());
        },
    },
    // 0x90
    Instruction {
        mnemonic: "SUB A,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.b());
        },
    },
    // 0x91
    Instruction {
        mnemonic: "SUB A,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.c());
        },
    },
    // 0x92
    Instruction {
        mnemonic: "SUB A,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.d());
        },
    },
    // 0x93
    Instruction {
        mnemonic: "SUB A,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.e());
        },
    },
    // 0x94
    Instruction {
        mnemonic: "SUB A,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.h());
        },
    },
    // 0x95
    Instruction {
        mnemonic: "SUB A,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.l());
        },
    },
    // 0x96
    Instruction {
        mnemonic: "SUB A,(HL)",
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
        mnemonic: "SUB A,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sub(cpu, cpu.regs.a());
        },
    },
    // 0x98
    Instruction {
        mnemonic: "SBC A,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.b());
        },
    },
    // 0x99
    Instruction {
        mnemonic: "SBC A,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.c());
        },
    },
    // 0x9a
    Instruction {
        mnemonic: "SBC A,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.d());
        },
    },
    // 0x9b
    Instruction {
        mnemonic: "SBC A,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.e());
        },
    },
    // 0x9c
    Instruction {
        mnemonic: "SBC A,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.h());
        },
    },
    // 0x9d
    Instruction {
        mnemonic: "SBC A,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.l());
        },
    },
    // 0x9e
    Instruction {
        mnemonic: "SBC A,(HL)",
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
        mnemonic: "SBC A,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::sbc(cpu, cpu.regs.a());
        },
    },
    // 0xa0
    Instruction {
        mnemonic: "AND A,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.b());
        },
    },
    // 0xa1
    Instruction {
        mnemonic: "AND A,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.c());
        },
    },
    // 0xa2
    Instruction {
        mnemonic: "AND A,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.d());
        },
    },
    // 0xa3
    Instruction {
        mnemonic: "AND A,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.e());
        },
    },
    // 0xa4
    Instruction {
        mnemonic: "AND A,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.h());
        },
    },
    // 0xa5
    Instruction {
        mnemonic: "AND A,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.l());
        },
    },
    // 0xa6
    Instruction {
        mnemonic: "AND A,(HL)",
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
        mnemonic: "AND A,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::and(cpu, cpu.regs.a());
        },
    },
    // 0xa8
    Instruction {
        mnemonic: "XOR A,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.b());
        },
    },
    // 0xa9
    Instruction {
        mnemonic: "XOR A,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.c());
        },
    },
    // 0xaa
    Instruction {
        mnemonic: "XOR A,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.d());
        },
    },
    // 0xab
    Instruction {
        mnemonic: "XOR A,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.e());
        },
    },
    // 0xac
    Instruction {
        mnemonic: "XOR A,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.h());
        },
    },
    // 0xad
    Instruction {
        mnemonic: "XOR A,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.l());
        },
    },
    // 0xae
    Instruction {
        mnemonic: "XOR A,(HL)",
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
        mnemonic: "XOR A,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::xor(cpu, cpu.regs.a());
        },
    },
    // 0xb0
    Instruction {
        mnemonic: "OR A,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.b());
        },
    },
    // 0xb1
    Instruction {
        mnemonic: "OR A,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.c());
        },
    },
    // 0xb2
    Instruction {
        mnemonic: "OR A,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.d());
        },
    },
    // 0xb3
    Instruction {
        mnemonic: "OR A,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.e());
        },
    },
    // 0xb4
    Instruction {
        mnemonic: "OR A,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.h());
        },
    },
    // 0xb5
    Instruction {
        mnemonic: "OR A,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.l());
        },
    },
    // 0xb6
    Instruction {
        mnemonic: "OR A,(HL)",
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
        mnemonic: "OR A,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::or(cpu, cpu.regs.a());
        },
    },
    // 0xb8
    Instruction {
        mnemonic: "CP A,B",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.b());
        },
    },
    // 0xb9
    Instruction {
        mnemonic: "CP A,C",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.c());
        },
    },
    // 0xba
    Instruction {
        mnemonic: "CP A,D",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.d());
        },
    },
    // 0xbb
    Instruction {
        mnemonic: "CP A,E",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.e());
        },
    },
    // 0xbc
    Instruction {
        mnemonic: "CP A,H",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.h());
        },
    },
    // 0xbd
    Instruction {
        mnemonic: "CP A,L",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.l());
        },
    },
    // 0xbe
    Instruction {
        mnemonic: "CP A,(HL)",
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
        mnemonic: "CP A,A",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::cp(cpu, cpu.regs.a());
        },
    },
    // 0xc0
    Instruction {
        mnemonic: "RET NZ",
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
        mnemonic: "POP BC",
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
        mnemonic: "JP NZ,u16",
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
        mnemonic: "JP u16",
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
        mnemonic: "CALL NZ,u16",
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
        mnemonic: "PUSH BC",
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
        mnemonic: "ADD A,u8",
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
        mnemonic: "RST 00H",
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
        mnemonic: "RET Z",
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
        mnemonic: "RET",
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
        mnemonic: "JP Z,u16",
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
        mnemonic: "PREFIX CB",
        param_type: ParamType::None,
        cycles: 0,
        brcycles: None,
        operation: |cpu| cpu.execute(BITWISE_INSTRS),
    },
    // 0xcc
    Instruction {
        mnemonic: "CALL Z,u16",
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
        mnemonic: "CALL u16",
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
        mnemonic: "ADC A,u8",
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
        mnemonic: "RST 08H",
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
        mnemonic: "RET NC",
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
        mnemonic: "POP DE",
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
        mnemonic: "JP NC,u16",
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
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xd4
    Instruction {
        mnemonic: "CALL NC,u16",
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
        mnemonic: "PUSH DE",
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
        mnemonic: "SUB A,u8",
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
        mnemonic: "RST 10H",
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
        mnemonic: "RET C",
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
        mnemonic: "RETI",
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
        mnemonic: "JP C,u16",
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
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xdc
    Instruction {
        mnemonic: "CALL C,u16",
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
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xde
    Instruction {
        mnemonic: "SBC A,u8",
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
        mnemonic: "RST 18H",
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
        mnemonic: "LDH (u8),A",
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
        mnemonic: "POP HL",
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
        mnemonic: "LDH (C),A",
        param_type: ParamType::None,
        cycles: 2,
        brcycles: None,
        operation: |cpu| {
            cpu.write_byte(0xff00 + cpu.regs.c() as u16, cpu.regs.a());
        },
    },
    // 0xe3
    Instruction {
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xe4
    Instruction {
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xe5
    Instruction {
        mnemonic: "PUSH HL",
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
        mnemonic: "AND A,u8",
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
        mnemonic: "RST 20H",
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
        mnemonic: "ADD SP,i8",
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
        mnemonic: "JP HL",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            helpers::jp(cpu, cpu.regs.hl());
        },
    },
    // 0xea
    Instruction {
        mnemonic: "LD (u16),A",
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
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xec
    Instruction {
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xed
    Instruction {
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xee
    Instruction {
        mnemonic: "XOR A,u8",
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
        mnemonic: "RST 28H",
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
        mnemonic: "LDH A,(u8)",
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
        mnemonic: "POP AF",
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
        mnemonic: "LDH A,(C)",
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
        mnemonic: "DI",
        param_type: ParamType::None,
        cycles: 1,
        brcycles: None,
        operation: |cpu| {
            cpu.ime = ImeState::Disabled;
        },
    },
    // 0xf4
    Instruction {
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xf5
    Instruction {
        mnemonic: "PUSH AF",
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
        mnemonic: "OR A,u8",
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
        mnemonic: "RST 30H",
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
        mnemonic: "LD HL,SP+i8",
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
        mnemonic: "LD SP,HL",
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
        mnemonic: "LD A,(u16)",
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
        mnemonic: "EI",
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
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xfd
    Instruction {
        mnemonic: "",
        param_type: ParamType::Word,
        cycles: 0,
        brcycles: None,
        operation: |_| {
            helpers::invalid();
        },
    },
    // 0xfe
    Instruction {
        mnemonic: "CP A,u8",
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
        mnemonic: "RST 38H",
        param_type: ParamType::None,
        cycles: 4,
        brcycles: None,
        operation: |cpu| {
            cpu.cycle();
            helpers::call(cpu, 0x0038);
        },
    },
];
