use crate::{
    cpu::{instructions::helpers, Cpu, ImeState},
    peripherals::{Cable, Joypad, Lcd, Speaker},
};

impl<L, S, J, C> Cpu<L, S, J, C>
where
    L: Lcd,
    S: Speaker,
    J: Joypad,
    C: Cable,
{
    pub(in crate::cpu) fn execute_base(&mut self, opcode: u8) {
        match opcode {
            0x0 => {}
            0x1 => {
                let val = self.fetch_word();
                self.regs.set_bc(val);
            }
            0x2 => {
                self.write_byte(self.regs.bc(), self.regs.a());
            }
            0x3 => {
                self.cycle();
                self.regs.set_bc(self.regs.bc().wrapping_add(1));
            }
            0x4 => {
                let res = helpers::inc(self, self.regs.b());
                self.regs.set_b(res);
            }
            0x5 => {
                let res = helpers::dec(self, self.regs.b());
                self.regs.set_b(res);
            }
            0x6 => {
                let val = self.fetch_byte();
                self.regs.set_b(val);
            }
            0x7 => {
                let res = helpers::rlc(self, self.regs.a());
                self.regs.flags_mut().set_z(false);
                self.regs.set_a(res);
            }
            0x8 => {
                let addr = self.fetch_word();
                self.write_word(addr, self.regs.sp());
            }
            0x9 => {
                self.cycle();
                helpers::add_hl(self, self.regs.bc());
            }
            0xa => {
                let val = self.read_byte(self.regs.bc());
                self.regs.set_a(val);
            }
            0xb => {
                self.cycle();
                self.regs.set_bc(self.regs.bc().wrapping_sub(1));
            }
            0xc => {
                let res = helpers::inc(self, self.regs.c());
                self.regs.set_c(res);
            }
            0xd => {
                let res = helpers::dec(self, self.regs.c());
                self.regs.set_c(res);
            }
            0xe => {
                let val = self.fetch_byte();
                self.regs.set_c(val);
            }
            0xf => {
                let res = helpers::rrc(self, self.regs.a());
                self.regs.flags_mut().set_z(false);
                self.regs.set_a(res);
            }
            0x10 => {
                log::warn!("STOP instruction called, but it is not implemented properly yet.");
            }
            0x11 => {
                let val = self.fetch_word();
                self.regs.set_de(val);
            }
            0x12 => {
                self.write_byte(self.regs.de(), self.regs.a());
            }
            0x13 => {
                self.cycle();
                self.regs.set_de(self.regs.de().wrapping_add(1));
            }
            0x14 => {
                let res = helpers::inc(self, self.regs.d());
                self.regs.set_d(res);
            }
            0x15 => {
                let res = helpers::dec(self, self.regs.d());
                self.regs.set_d(res);
            }
            0x16 => {
                let val = self.fetch_byte();
                self.regs.set_d(val);
            }
            0x17 => {
                let res = helpers::rl(self, self.regs.a());
                self.regs.flags_mut().set_z(false);
                self.regs.set_a(res);
            }
            0x18 => {
                let val = self.fetch_byte() as i8;
                self.cycle();
                helpers::jr(self, val);
            }
            0x19 => {
                self.cycle();
                helpers::add_hl(self, self.regs.de());
            }
            0x1a => {
                let val = self.read_byte(self.regs.de());
                self.regs.set_a(val);
            }
            0x1b => {
                self.cycle();
                self.regs.set_de(self.regs.de().wrapping_sub(1));
            }
            0x1c => {
                let res = helpers::inc(self, self.regs.e());
                self.regs.set_e(res);
            }
            0x1d => {
                let res = helpers::dec(self, self.regs.e());
                self.regs.set_e(res);
            }
            0x1e => {
                let val = self.fetch_byte();
                self.regs.set_e(val);
            }
            0x1f => {
                let res = helpers::rr(self, self.regs.a());
                self.regs.flags_mut().set_z(false);
                self.regs.set_a(res);
            }
            0x20 => {
                let val = self.fetch_byte() as i8;
                if !self.regs.flags().z() {
                    self.cycle();
                    helpers::jr(self, val);
                }
            }
            0x21 => {
                let val = self.fetch_word();
                self.regs.set_hl(val);
            }
            0x22 => {
                let hl = self.regs.hl();
                self.write_byte(hl, self.regs.a());
                self.regs.inc_hl();
            }
            0x23 => {
                self.cycle();
                self.regs.set_hl(self.regs.hl().wrapping_add(1));
            }
            0x24 => {
                let res = helpers::inc(self, self.regs.h());
                self.regs.set_h(res);
            }
            0x25 => {
                let res = helpers::dec(self, self.regs.h());
                self.regs.set_h(res);
            }
            0x26 => {
                let val = self.fetch_byte();
                self.regs.set_h(val);
            }
            0x27 => {
                helpers::daa(self);
            }
            0x28 => {
                let val = self.fetch_byte() as i8;
                if self.regs.flags().z() {
                    self.cycle();
                    helpers::jr(self, val);
                }
            }
            0x29 => {
                self.cycle();
                helpers::add_hl(self, self.regs.hl());
            }
            0x2a => {
                let hl = self.regs.hl();
                let val = self.read_byte(hl);
                self.regs.inc_hl();
                self.regs.set_a(val);
            }
            0x2b => {
                self.cycle();
                self.regs.set_hl(self.regs.hl().wrapping_sub(1));
            }
            0x2c => {
                let res = helpers::inc(self, self.regs.l());
                self.regs.set_l(res);
            }
            0x2d => {
                let res = helpers::dec(self, self.regs.l());
                self.regs.set_l(res);
            }
            0x2e => {
                let val = self.fetch_byte();
                self.regs.set_l(val);
            }
            0x2f => {
                helpers::cpl(self);
            }
            0x30 => {
                let val = self.fetch_byte() as i8;
                if !self.regs.flags().c() {
                    self.cycle();
                    helpers::jr(self, val);
                }
            }
            0x31 => {
                let val = self.fetch_word();
                self.regs.set_sp(val);
            }
            0x32 => {
                let hl = self.regs.hl();
                self.write_byte(hl, self.regs.a());
                self.regs.dec_hl();
            }
            0x33 => {
                self.cycle();
                self.regs.set_sp(self.regs.sp().wrapping_add(1));
            }
            0x34 => {
                let val = self.read_byte(self.regs.hl());
                let res = helpers::inc(self, val);
                self.write_byte(self.regs.hl(), res);
            }
            0x35 => {
                let val = self.read_byte(self.regs.hl());
                let res = helpers::dec(self, val);
                self.write_byte(self.regs.hl(), res);
            }
            0x36 => {
                let val = self.fetch_byte();
                self.write_byte(self.regs.hl(), val);
            }
            0x37 => {
                helpers::scf(self);
            }
            0x38 => {
                let val = self.fetch_byte() as i8;
                if self.regs.flags().c() {
                    self.cycle();
                    helpers::jr(self, val);
                }
            }
            0x39 => {
                self.cycle();
                helpers::add_hl(self, self.regs.sp());
            }
            0x3a => {
                let hl = self.regs.hl();
                let val = self.read_byte(hl);
                self.regs.dec_hl();
                self.regs.set_a(val);
            }
            0x3b => {
                self.cycle();
                self.regs.set_sp(self.regs.sp().wrapping_sub(1));
            }
            0x3c => {
                let res = helpers::inc(self, self.regs.a());
                self.regs.set_a(res);
            }
            0x3d => {
                let res = helpers::dec(self, self.regs.a());
                self.regs.set_a(res);
            }
            0x3e => {
                let val = self.fetch_byte();
                self.regs.set_a(val);
            }
            0x3f => {
                helpers::ccf(self);
            }
            0x40 => {}
            0x41 => {
                self.regs.set_b(self.regs.c());
            }
            0x42 => {
                self.regs.set_b(self.regs.d());
            }
            0x43 => {
                self.regs.set_b(self.regs.e());
            }
            0x44 => {
                self.regs.set_b(self.regs.h());
            }
            0x45 => {
                self.regs.set_b(self.regs.l());
            }
            0x46 => {
                let val = self.read_byte(self.regs.hl());
                self.regs.set_b(val);
            }
            0x47 => {
                self.regs.set_b(self.regs.a());
            }
            0x48 => {
                self.regs.set_c(self.regs.b());
            }
            0x49 => {}
            0x4a => {
                self.regs.set_c(self.regs.d());
            }
            0x4b => {
                self.regs.set_c(self.regs.e());
            }
            0x4c => {
                self.regs.set_c(self.regs.h());
            }
            0x4d => {
                self.regs.set_c(self.regs.l());
            }
            0x4e => {
                let val = self.read_byte(self.regs.hl());
                self.regs.set_c(val);
            }
            0x4f => {
                self.regs.set_c(self.regs.a());
            }
            0x50 => {
                self.regs.set_d(self.regs.b());
            }
            0x51 => {
                self.regs.set_d(self.regs.c());
            }
            0x52 => {}
            0x53 => {
                self.regs.set_d(self.regs.e());
            }
            0x54 => {
                self.regs.set_d(self.regs.h());
            }
            0x55 => {
                self.regs.set_d(self.regs.l());
            }
            0x56 => {
                let val = self.read_byte(self.regs.hl());
                self.regs.set_d(val);
            }
            0x57 => {
                self.regs.set_d(self.regs.a());
            }
            0x58 => {
                self.regs.set_e(self.regs.b());
            }
            0x59 => {
                self.regs.set_e(self.regs.c());
            }
            0x5a => {
                self.regs.set_e(self.regs.d());
            }
            0x5b => {}
            0x5c => {
                self.regs.set_e(self.regs.h());
            }
            0x5d => {
                self.regs.set_e(self.regs.l());
            }
            0x5e => {
                let val = self.read_byte(self.regs.hl());
                self.regs.set_e(val);
            }
            0x5f => {
                self.regs.set_e(self.regs.a());
            }
            0x60 => {
                self.regs.set_h(self.regs.b());
            }
            0x61 => {
                self.regs.set_h(self.regs.c());
            }
            0x62 => {
                self.regs.set_h(self.regs.d());
            }
            0x63 => {
                self.regs.set_h(self.regs.e());
            }
            0x64 => {}
            0x65 => {
                self.regs.set_h(self.regs.l());
            }
            0x66 => {
                let val = self.read_byte(self.regs.hl());
                self.regs.set_h(val);
            }
            0x67 => {
                self.regs.set_h(self.regs.a());
            }
            0x68 => {
                self.regs.set_l(self.regs.b());
            }
            0x69 => {
                self.regs.set_l(self.regs.c());
            }
            0x6a => {
                self.regs.set_l(self.regs.d());
            }
            0x6b => {
                self.regs.set_l(self.regs.e());
            }
            0x6c => {
                self.regs.set_l(self.regs.h());
            }
            0x6d => {}
            0x6e => {
                let val = self.read_byte(self.regs.hl());
                self.regs.set_l(val);
            }
            0x6f => {
                self.regs.set_l(self.regs.a());
            }
            0x70 => {
                self.write_byte(self.regs.hl(), self.regs.b());
            }
            0x71 => {
                self.write_byte(self.regs.hl(), self.regs.c());
            }
            0x72 => {
                self.write_byte(self.regs.hl(), self.regs.d());
            }
            0x73 => {
                self.write_byte(self.regs.hl(), self.regs.e());
            }
            0x74 => {
                self.write_byte(self.regs.hl(), self.regs.h());
            }
            0x75 => {
                self.write_byte(self.regs.hl(), self.regs.l());
            }
            0x76 => {
                log::debug!("CPU: Halted");
                self.halted = true;
            }
            0x77 => {
                self.write_byte(self.regs.hl(), self.regs.a());
            }
            0x78 => {
                self.regs.set_a(self.regs.b());
            }
            0x79 => {
                self.regs.set_a(self.regs.c());
            }
            0x7a => {
                self.regs.set_a(self.regs.d());
            }
            0x7b => {
                self.regs.set_a(self.regs.e());
            }
            0x7c => {
                self.regs.set_a(self.regs.h());
            }
            0x7d => {
                self.regs.set_a(self.regs.l());
            }
            0x7e => {
                let val = self.read_byte(self.regs.hl());
                self.regs.set_a(val);
            }
            0x7f => {}
            0x80 => {
                helpers::add(self, self.regs.b());
            }
            0x81 => {
                helpers::add(self, self.regs.c());
            }
            0x82 => {
                helpers::add(self, self.regs.d());
            }
            0x83 => {
                helpers::add(self, self.regs.e());
            }
            0x84 => {
                helpers::add(self, self.regs.h());
            }
            0x85 => {
                helpers::add(self, self.regs.l());
            }
            0x86 => {
                let val = self.read_byte(self.regs.hl());
                helpers::add(self, val);
            }
            0x87 => {
                helpers::add(self, self.regs.a());
            }
            0x88 => {
                helpers::adc(self, self.regs.b());
            }
            0x89 => {
                helpers::adc(self, self.regs.c());
            }
            0x8a => {
                helpers::adc(self, self.regs.d());
            }
            0x8b => {
                helpers::adc(self, self.regs.e());
            }
            0x8c => {
                helpers::adc(self, self.regs.h());
            }
            0x8d => {
                helpers::adc(self, self.regs.l());
            }
            0x8e => {
                let val = self.read_byte(self.regs.hl());
                helpers::adc(self, val);
            }
            0x8f => {
                helpers::adc(self, self.regs.a());
            }
            0x90 => {
                helpers::sub(self, self.regs.b());
            }
            0x91 => {
                helpers::sub(self, self.regs.c());
            }
            0x92 => {
                helpers::sub(self, self.regs.d());
            }
            0x93 => {
                helpers::sub(self, self.regs.e());
            }
            0x94 => {
                helpers::sub(self, self.regs.h());
            }
            0x95 => {
                helpers::sub(self, self.regs.l());
            }
            0x96 => {
                let val = self.read_byte(self.regs.hl());
                helpers::sub(self, val);
            }
            0x97 => {
                helpers::sub(self, self.regs.a());
            }
            0x98 => {
                helpers::sbc(self, self.regs.b());
            }
            0x99 => {
                helpers::sbc(self, self.regs.c());
            }
            0x9a => {
                helpers::sbc(self, self.regs.d());
            }
            0x9b => {
                helpers::sbc(self, self.regs.e());
            }
            0x9c => {
                helpers::sbc(self, self.regs.h());
            }
            0x9d => {
                helpers::sbc(self, self.regs.l());
            }
            0x9e => {
                let val = self.read_byte(self.regs.hl());
                helpers::sbc(self, val);
            }
            0x9f => {
                helpers::sbc(self, self.regs.a());
            }
            0xa0 => {
                helpers::and(self, self.regs.b());
            }
            0xa1 => {
                helpers::and(self, self.regs.c());
            }
            0xa2 => {
                helpers::and(self, self.regs.d());
            }
            0xa3 => {
                helpers::and(self, self.regs.e());
            }
            0xa4 => {
                helpers::and(self, self.regs.h());
            }
            0xa5 => {
                helpers::and(self, self.regs.l());
            }
            0xa6 => {
                let val = self.read_byte(self.regs.hl());
                helpers::and(self, val);
            }
            0xa7 => {
                helpers::and(self, self.regs.a());
            }
            0xa8 => {
                helpers::xor(self, self.regs.b());
            }
            0xa9 => {
                helpers::xor(self, self.regs.c());
            }
            0xaa => {
                helpers::xor(self, self.regs.d());
            }
            0xab => {
                helpers::xor(self, self.regs.e());
            }
            0xac => {
                helpers::xor(self, self.regs.h());
            }
            0xad => {
                helpers::xor(self, self.regs.l());
            }
            0xae => {
                let val = self.read_byte(self.regs.hl());
                helpers::xor(self, val);
            }
            0xaf => {
                helpers::xor(self, self.regs.a());
            }
            0xb0 => {
                helpers::or(self, self.regs.b());
            }
            0xb1 => {
                helpers::or(self, self.regs.c());
            }
            0xb2 => {
                helpers::or(self, self.regs.d());
            }
            0xb3 => {
                helpers::or(self, self.regs.e());
            }
            0xb4 => {
                helpers::or(self, self.regs.h());
            }
            0xb5 => {
                helpers::or(self, self.regs.l());
            }
            0xb6 => {
                let val = self.read_byte(self.regs.hl());
                helpers::or(self, val);
            }
            0xb7 => {
                helpers::or(self, self.regs.a());
            }
            0xb8 => {
                helpers::cp(self, self.regs.b());
            }
            0xb9 => {
                helpers::cp(self, self.regs.c());
            }
            0xba => {
                helpers::cp(self, self.regs.d());
            }
            0xbb => {
                helpers::cp(self, self.regs.e());
            }
            0xbc => {
                helpers::cp(self, self.regs.h());
            }
            0xbd => {
                helpers::cp(self, self.regs.l());
            }
            0xbe => {
                let val = self.read_byte(self.regs.hl());
                helpers::cp(self, val);
            }
            0xbf => {
                helpers::cp(self, self.regs.a());
            }
            0xc0 => {
                self.cycle();
                if !self.regs.flags().z() {
                    helpers::ret(self);
                    self.cycle();
                }
            }
            0xc1 => {
                let val = self.stack_pop();
                self.regs.set_bc(val);
            }
            0xc2 => {
                let val = self.fetch_word();
                if !self.regs.flags().z() {
                    self.cycle();
                    helpers::jp(self, val);
                }
            }
            0xc3 => {
                let val = self.fetch_word();
                self.cycle();
                helpers::jp(self, val);
            }
            0xc4 => {
                let val = self.fetch_word();
                if !self.regs.flags().z() {
                    self.cycle();
                    helpers::call(self, val);
                }
            }
            0xc5 => {
                self.cycle();
                self.stack_push(self.regs.bc());
            }
            0xc6 => {
                let val = self.fetch_byte();
                helpers::add(self, val);
            }
            0xc7 => {
                self.cycle();
                helpers::call(self, 0x0000);
            }
            0xc8 => {
                self.cycle();
                if self.regs.flags().z() {
                    helpers::ret(self);
                    self.cycle();
                }
            }
            0xc9 => {
                helpers::ret(self);
                self.cycle();
            }
            0xca => {
                let val = self.fetch_word();
                if self.regs.flags().z() {
                    self.cycle();
                    helpers::jp(self, val);
                }
            }
            0xcb => {
                helpers::invalid();
            }
            0xcc => {
                let val = self.fetch_word();
                if self.regs.flags().z() {
                    self.cycle();
                    helpers::call(self, val);
                }
            }
            0xcd => {
                let val = self.fetch_word();
                self.cycle();
                helpers::call(self, val);
            }
            0xce => {
                let val = self.fetch_byte();
                helpers::adc(self, val);
            }
            0xcf => {
                self.cycle();
                helpers::call(self, 0x0008);
            }
            0xd0 => {
                self.cycle();
                if !self.regs.flags().c() {
                    helpers::ret(self);
                    self.cycle();
                }
            }
            0xd1 => {
                let val = self.stack_pop();
                self.regs.set_de(val);
            }
            0xd2 => {
                let val = self.fetch_word();
                if !self.regs.flags().c() {
                    self.cycle();
                    helpers::jp(self, val);
                }
            }
            0xd3 => {
                helpers::invalid();
            }
            0xd4 => {
                let val = self.fetch_word();
                if !self.regs.flags().c() {
                    self.cycle();
                    helpers::call(self, val);
                }
            }
            0xd5 => {
                self.cycle();
                self.stack_push(self.regs.de());
            }
            0xd6 => {
                let val = self.fetch_byte();
                helpers::sub(self, val);
            }
            0xd7 => {
                self.cycle();
                helpers::call(self, 0x0010);
            }
            0xd8 => {
                self.cycle();
                if self.regs.flags().c() {
                    helpers::ret(self);
                    self.cycle();
                }
            }
            0xd9 => {
                helpers::ret(self);
                self.cycle();
                self.ime = ImeState::Enabling;
            }
            0xda => {
                let val = self.fetch_word();
                if self.regs.flags().c() {
                    self.cycle();
                    helpers::jp(self, val);
                }
            }
            0xdb => {
                helpers::invalid();
            }
            0xdc => {
                let val = self.fetch_word();
                if self.regs.flags().c() {
                    self.cycle();
                    helpers::call(self, val);
                }
            }
            0xdd => {
                helpers::invalid();
            }
            0xde => {
                let val = self.fetch_byte();
                helpers::sbc(self, val);
            }
            0xdf => {
                self.cycle();
                helpers::call(self, 0x0018);
            }
            0xe0 => {
                let addr = 0xff00 + self.fetch_byte() as u16;
                self.write_byte(addr, self.regs.a());
            }
            0xe1 => {
                let val = self.stack_pop();
                self.regs.set_hl(val);
            }
            0xe2 => {
                self.write_byte(0xff00 + self.regs.c() as u16, self.regs.a());
            }
            0xe3 => {
                helpers::invalid();
            }
            0xe4 => {
                helpers::invalid();
            }
            0xe5 => {
                self.cycle();
                self.stack_push(self.regs.hl());
            }
            0xe6 => {
                let val = self.fetch_byte();
                helpers::and(self, val);
            }
            0xe7 => {
                self.cycle();
                helpers::call(self, 0x0020);
            }
            0xe8 => {
                let val = self.fetch_byte() as i8;
                let sp = helpers::add_sp(self, val);
                self.cycle();
                self.cycle();
                self.regs.set_sp(sp);
            }
            0xe9 => {
                helpers::jp(self, self.regs.hl());
            }
            0xea => {
                let addr = self.fetch_word();
                self.write_byte(addr, self.regs.a());
            }
            0xeb => {
                helpers::invalid();
            }
            0xec => {
                helpers::invalid();
            }
            0xed => {
                helpers::invalid();
            }
            0xee => {
                let val = self.fetch_byte();
                helpers::xor(self, val);
            }
            0xef => {
                self.cycle();
                helpers::call(self, 0x0028);
            }
            0xf0 => {
                let addr = 0xff00 + self.fetch_byte() as u16;
                let val = self.read_byte(addr);
                self.regs.set_a(val);
            }
            0xf1 => {
                let val = self.stack_pop();
                self.regs.set_af(val);
            }
            0xf2 => {
                let val = self.read_byte(0xff00 + self.regs.c() as u16);
                self.regs.set_a(val);
            }
            0xf3 => {
                self.ime = ImeState::Disabled;
            }
            0xf4 => {
                helpers::invalid();
            }
            0xf5 => {
                self.cycle();
                self.stack_push(self.regs.af());
            }
            0xf6 => {
                let val = self.fetch_byte();
                helpers::or(self, val);
            }
            0xf7 => {
                self.cycle();
                helpers::call(self, 0x0030);
            }
            0xf8 => {
                let val = self.fetch_byte() as i8;
                let hl = helpers::add_sp(self, val);
                self.cycle();
                self.regs.set_hl(hl);
            }
            0xf9 => {
                self.cycle();
                self.regs.set_sp(self.regs.hl());
            }
            0xfa => {
                let addr = self.fetch_word();
                let val = self.read_byte(addr);
                self.regs.set_a(val);
            }
            0xfb => {
                log::debug!("CPU: Enabling IME");
                self.ime = ImeState::Enabling;
            }
            0xfc => {
                helpers::invalid();
            }
            0xfd => {
                helpers::invalid();
            }
            0xfe => {
                let val = self.fetch_byte();
                helpers::cp(self, val);
            }
            0xff => {
                self.cycle();
                helpers::call(self, 0x0038);
            }
        }
    }
}
