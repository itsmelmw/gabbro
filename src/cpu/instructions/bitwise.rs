use crate::{
    cpu::instructions::helpers,
    cpu::Cpu,
    peripherals::{Cable, Joypad, Lcd},
};

pub const BITWISE_PREFIX: u8 = 0xcb;

impl<L, J, C> Cpu<L, J, C>
where
    L: Lcd,
    J: Joypad,
    C: Cable,
{
    pub(in crate::cpu) fn execute_bitwise(&mut self, opcode: u8) {
        match opcode {
            0x0 => {
                let res = helpers::rlc(self, self.regs.b());
                self.regs.set_b(res);
            }
            0x1 => {
                let res = helpers::rlc(self, self.regs.c());
                self.regs.set_c(res);
            }
            0x2 => {
                let res = helpers::rlc(self, self.regs.d());
                self.regs.set_d(res);
            }
            0x3 => {
                let res = helpers::rlc(self, self.regs.e());
                self.regs.set_e(res);
            }
            0x4 => {
                let res = helpers::rlc(self, self.regs.h());
                self.regs.set_h(res);
            }
            0x5 => {
                let res = helpers::rlc(self, self.regs.l());
                self.regs.set_l(res);
            }
            0x6 => {
                let val = self.read_byte(self.regs.hl());
                let res = helpers::rlc(self, val);
                self.write_byte(self.regs.hl(), res);
            }
            0x7 => {
                let res = helpers::rlc(self, self.regs.a());
                self.regs.set_a(res);
            }
            0x8 => {
                let res = helpers::rrc(self, self.regs.b());
                self.regs.set_b(res);
            }
            0x9 => {
                let res = helpers::rrc(self, self.regs.c());
                self.regs.set_c(res);
            }
            0xa => {
                let res = helpers::rrc(self, self.regs.d());
                self.regs.set_d(res);
            }
            0xb => {
                let res = helpers::rrc(self, self.regs.e());
                self.regs.set_e(res);
            }
            0xc => {
                let res = helpers::rrc(self, self.regs.h());
                self.regs.set_h(res);
            }
            0xd => {
                let res = helpers::rrc(self, self.regs.l());
                self.regs.set_l(res);
            }
            0xe => {
                let val = self.read_byte(self.regs.hl());
                let res = helpers::rrc(self, val);
                self.write_byte(self.regs.hl(), res);
            }
            0xf => {
                let res = helpers::rrc(self, self.regs.a());
                self.regs.set_a(res);
            }
            0x10 => {
                let res = helpers::rl(self, self.regs.b());
                self.regs.set_b(res);
            }
            0x11 => {
                let res = helpers::rl(self, self.regs.c());
                self.regs.set_c(res);
            }
            0x12 => {
                let res = helpers::rl(self, self.regs.d());
                self.regs.set_d(res);
            }
            0x13 => {
                let res = helpers::rl(self, self.regs.e());
                self.regs.set_e(res);
            }
            0x14 => {
                let res = helpers::rl(self, self.regs.h());
                self.regs.set_h(res);
            }
            0x15 => {
                let res = helpers::rl(self, self.regs.l());
                self.regs.set_l(res);
            }
            0x16 => {
                let val = self.read_byte(self.regs.hl());
                let res = helpers::rl(self, val);
                self.write_byte(self.regs.hl(), res);
            }
            0x17 => {
                let res = helpers::rl(self, self.regs.a());
                self.regs.set_a(res);
            }
            0x18 => {
                let res = helpers::rr(self, self.regs.b());
                self.regs.set_b(res);
            }
            0x19 => {
                let res = helpers::rr(self, self.regs.c());
                self.regs.set_c(res);
            }
            0x1a => {
                let res = helpers::rr(self, self.regs.d());
                self.regs.set_d(res);
            }
            0x1b => {
                let res = helpers::rr(self, self.regs.e());
                self.regs.set_e(res);
            }
            0x1c => {
                let res = helpers::rr(self, self.regs.h());
                self.regs.set_h(res);
            }
            0x1d => {
                let res = helpers::rr(self, self.regs.l());
                self.regs.set_l(res);
            }
            0x1e => {
                let val = self.read_byte(self.regs.hl());
                let res = helpers::rr(self, val);
                self.write_byte(self.regs.hl(), res);
            }
            0x1f => {
                let res = helpers::rr(self, self.regs.a());
                self.regs.set_a(res);
            }
            0x20 => {
                let res = helpers::sla(self, self.regs.b());
                self.regs.set_b(res);
            }
            0x21 => {
                let res = helpers::sla(self, self.regs.c());
                self.regs.set_c(res);
            }
            0x22 => {
                let res = helpers::sla(self, self.regs.d());
                self.regs.set_d(res);
            }
            0x23 => {
                let res = helpers::sla(self, self.regs.e());
                self.regs.set_e(res);
            }
            0x24 => {
                let res = helpers::sla(self, self.regs.h());
                self.regs.set_h(res);
            }
            0x25 => {
                let res = helpers::sla(self, self.regs.l());
                self.regs.set_l(res);
            }
            0x26 => {
                let val = self.read_byte(self.regs.hl());
                let res = helpers::sla(self, val);
                self.write_byte(self.regs.hl(), res);
            }
            0x27 => {
                let res = helpers::sla(self, self.regs.a());
                self.regs.set_a(res);
            }
            0x28 => {
                let res = helpers::sra(self, self.regs.b());
                self.regs.set_b(res);
            }
            0x29 => {
                let res = helpers::sra(self, self.regs.c());
                self.regs.set_c(res);
            }
            0x2a => {
                let res = helpers::sra(self, self.regs.d());
                self.regs.set_d(res);
            }
            0x2b => {
                let res = helpers::sra(self, self.regs.e());
                self.regs.set_e(res);
            }
            0x2c => {
                let res = helpers::sra(self, self.regs.h());
                self.regs.set_h(res);
            }
            0x2d => {
                let res = helpers::sra(self, self.regs.l());
                self.regs.set_l(res);
            }
            0x2e => {
                let val = self.read_byte(self.regs.hl());
                let res = helpers::sra(self, val);
                self.write_byte(self.regs.hl(), res);
            }
            0x2f => {
                let res = helpers::sra(self, self.regs.a());
                self.regs.set_a(res);
            }
            0x30 => {
                let res = helpers::swap(self, self.regs.b());
                self.regs.set_b(res);
            }
            0x31 => {
                let res = helpers::swap(self, self.regs.c());
                self.regs.set_c(res);
            }
            0x32 => {
                let res = helpers::swap(self, self.regs.d());
                self.regs.set_d(res);
            }
            0x33 => {
                let res = helpers::swap(self, self.regs.e());
                self.regs.set_e(res);
            }
            0x34 => {
                let res = helpers::swap(self, self.regs.h());
                self.regs.set_h(res);
            }
            0x35 => {
                let res = helpers::swap(self, self.regs.l());
                self.regs.set_l(res);
            }
            0x36 => {
                let val = self.read_byte(self.regs.hl());
                let res = helpers::swap(self, val);
                self.write_byte(self.regs.hl(), res);
            }
            0x37 => {
                let res = helpers::swap(self, self.regs.a());
                self.regs.set_a(res);
            }
            0x38 => {
                let res = helpers::srl(self, self.regs.b());
                self.regs.set_b(res);
            }
            0x39 => {
                let res = helpers::srl(self, self.regs.c());
                self.regs.set_c(res);
            }
            0x3a => {
                let res = helpers::srl(self, self.regs.d());
                self.regs.set_d(res);
            }
            0x3b => {
                let res = helpers::srl(self, self.regs.e());
                self.regs.set_e(res);
            }
            0x3c => {
                let res = helpers::srl(self, self.regs.h());
                self.regs.set_h(res);
            }
            0x3d => {
                let res = helpers::srl(self, self.regs.l());
                self.regs.set_l(res);
            }
            0x3e => {
                let val = self.read_byte(self.regs.hl());
                let res = helpers::srl(self, val);
                self.write_byte(self.regs.hl(), res);
            }
            0x3f => {
                let res = helpers::srl(self, self.regs.a());
                self.regs.set_a(res);
            }
            0x40 => {
                helpers::bit(self, 0, self.regs.b());
            }
            0x41 => {
                helpers::bit(self, 0, self.regs.c());
            }
            0x42 => {
                helpers::bit(self, 0, self.regs.d());
            }
            0x43 => {
                helpers::bit(self, 0, self.regs.e());
            }
            0x44 => {
                helpers::bit(self, 0, self.regs.h());
            }
            0x45 => {
                helpers::bit(self, 0, self.regs.l());
            }
            0x46 => {
                let val = self.read_byte(self.regs.hl());
                helpers::bit(self, 0, val);
            }
            0x47 => {
                helpers::bit(self, 0, self.regs.a());
            }
            0x48 => {
                helpers::bit(self, 1, self.regs.b());
            }
            0x49 => {
                helpers::bit(self, 1, self.regs.c());
            }
            0x4a => {
                helpers::bit(self, 1, self.regs.d());
            }
            0x4b => {
                helpers::bit(self, 1, self.regs.e());
            }
            0x4c => {
                helpers::bit(self, 1, self.regs.h());
            }
            0x4d => {
                helpers::bit(self, 1, self.regs.l());
            }
            0x4e => {
                let val = self.read_byte(self.regs.hl());
                helpers::bit(self, 1, val);
            }
            0x4f => {
                helpers::bit(self, 1, self.regs.a());
            }
            0x50 => {
                helpers::bit(self, 2, self.regs.b());
            }
            0x51 => {
                helpers::bit(self, 2, self.regs.c());
            }
            0x52 => {
                helpers::bit(self, 2, self.regs.d());
            }
            0x53 => {
                helpers::bit(self, 2, self.regs.e());
            }
            0x54 => {
                helpers::bit(self, 2, self.regs.h());
            }
            0x55 => {
                helpers::bit(self, 2, self.regs.l());
            }
            0x56 => {
                let val = self.read_byte(self.regs.hl());
                helpers::bit(self, 2, val);
            }
            0x57 => {
                helpers::bit(self, 2, self.regs.a());
            }
            0x58 => {
                helpers::bit(self, 3, self.regs.b());
            }
            0x59 => {
                helpers::bit(self, 3, self.regs.c());
            }
            0x5a => {
                helpers::bit(self, 3, self.regs.d());
            }
            0x5b => {
                helpers::bit(self, 3, self.regs.e());
            }
            0x5c => {
                helpers::bit(self, 3, self.regs.h());
            }
            0x5d => {
                helpers::bit(self, 3, self.regs.l());
            }
            0x5e => {
                let val = self.read_byte(self.regs.hl());
                helpers::bit(self, 3, val);
            }
            0x5f => {
                helpers::bit(self, 3, self.regs.a());
            }
            0x60 => {
                helpers::bit(self, 4, self.regs.b());
            }
            0x61 => {
                helpers::bit(self, 4, self.regs.c());
            }
            0x62 => {
                helpers::bit(self, 4, self.regs.d());
            }
            0x63 => {
                helpers::bit(self, 4, self.regs.e());
            }
            0x64 => {
                helpers::bit(self, 4, self.regs.h());
            }
            0x65 => {
                helpers::bit(self, 4, self.regs.l());
            }
            0x66 => {
                let val = self.read_byte(self.regs.hl());
                helpers::bit(self, 4, val);
            }
            0x67 => {
                helpers::bit(self, 4, self.regs.a());
            }
            0x68 => {
                helpers::bit(self, 5, self.regs.b());
            }
            0x69 => {
                helpers::bit(self, 5, self.regs.c());
            }
            0x6a => {
                helpers::bit(self, 5, self.regs.d());
            }
            0x6b => {
                helpers::bit(self, 5, self.regs.e());
            }
            0x6c => {
                helpers::bit(self, 5, self.regs.h());
            }
            0x6d => {
                helpers::bit(self, 5, self.regs.l());
            }
            0x6e => {
                let val = self.read_byte(self.regs.hl());
                helpers::bit(self, 5, val);
            }
            0x6f => {
                helpers::bit(self, 5, self.regs.a());
            }
            0x70 => {
                helpers::bit(self, 6, self.regs.b());
            }
            0x71 => {
                helpers::bit(self, 6, self.regs.c());
            }
            0x72 => {
                helpers::bit(self, 6, self.regs.d());
            }
            0x73 => {
                helpers::bit(self, 6, self.regs.e());
            }
            0x74 => {
                helpers::bit(self, 6, self.regs.h());
            }
            0x75 => {
                helpers::bit(self, 6, self.regs.l());
            }
            0x76 => {
                let val = self.read_byte(self.regs.hl());
                helpers::bit(self, 6, val);
            }
            0x77 => {
                helpers::bit(self, 6, self.regs.a());
            }
            0x78 => {
                helpers::bit(self, 7, self.regs.b());
            }
            0x79 => {
                helpers::bit(self, 7, self.regs.c());
            }
            0x7a => {
                helpers::bit(self, 7, self.regs.d());
            }
            0x7b => {
                helpers::bit(self, 7, self.regs.e());
            }
            0x7c => {
                helpers::bit(self, 7, self.regs.h());
            }
            0x7d => {
                helpers::bit(self, 7, self.regs.l());
            }
            0x7e => {
                let val = self.read_byte(self.regs.hl());
                helpers::bit(self, 7, val);
            }
            0x7f => {
                helpers::bit(self, 7, self.regs.a());
            }
            0x80 => {
                self.regs.set_b(helpers::res(0, self.regs.b()));
            }
            0x81 => {
                self.regs.set_c(helpers::res(0, self.regs.c()));
            }
            0x82 => {
                self.regs.set_d(helpers::res(0, self.regs.d()));
            }
            0x83 => {
                self.regs.set_e(helpers::res(0, self.regs.e()));
            }
            0x84 => {
                self.regs.set_h(helpers::res(0, self.regs.h()));
            }
            0x85 => {
                self.regs.set_l(helpers::res(0, self.regs.l()));
            }
            0x86 => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::res(0, val));
            }
            0x87 => {
                self.regs.set_a(helpers::res(0, self.regs.a()));
            }
            0x88 => {
                self.regs.set_b(helpers::res(1, self.regs.b()));
            }
            0x89 => {
                self.regs.set_c(helpers::res(1, self.regs.c()));
            }
            0x8a => {
                self.regs.set_d(helpers::res(1, self.regs.d()));
            }
            0x8b => {
                self.regs.set_e(helpers::res(1, self.regs.e()));
            }
            0x8c => {
                self.regs.set_h(helpers::res(1, self.regs.h()));
            }
            0x8d => {
                self.regs.set_l(helpers::res(1, self.regs.l()));
            }
            0x8e => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::res(1, val));
            }
            0x8f => {
                self.regs.set_a(helpers::res(1, self.regs.a()));
            }
            0x90 => {
                self.regs.set_b(helpers::res(2, self.regs.b()));
            }
            0x91 => {
                self.regs.set_c(helpers::res(2, self.regs.c()));
            }
            0x92 => {
                self.regs.set_d(helpers::res(2, self.regs.d()));
            }
            0x93 => {
                self.regs.set_e(helpers::res(2, self.regs.e()));
            }
            0x94 => {
                self.regs.set_h(helpers::res(2, self.regs.h()));
            }
            0x95 => {
                self.regs.set_l(helpers::res(2, self.regs.l()));
            }
            0x96 => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::res(2, val));
            }
            0x97 => {
                self.regs.set_a(helpers::res(2, self.regs.a()));
            }
            0x98 => {
                self.regs.set_b(helpers::res(3, self.regs.b()));
            }
            0x99 => {
                self.regs.set_c(helpers::res(3, self.regs.c()));
            }
            0x9a => {
                self.regs.set_d(helpers::res(3, self.regs.d()));
            }
            0x9b => {
                self.regs.set_e(helpers::res(3, self.regs.e()));
            }
            0x9c => {
                self.regs.set_h(helpers::res(3, self.regs.h()));
            }
            0x9d => {
                self.regs.set_l(helpers::res(3, self.regs.l()));
            }
            0x9e => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::res(3, val));
            }
            0x9f => {
                self.regs.set_a(helpers::res(3, self.regs.a()));
            }
            0xa0 => {
                self.regs.set_b(helpers::res(4, self.regs.b()));
            }
            0xa1 => {
                self.regs.set_c(helpers::res(4, self.regs.c()));
            }
            0xa2 => {
                self.regs.set_d(helpers::res(4, self.regs.d()));
            }
            0xa3 => {
                self.regs.set_e(helpers::res(4, self.regs.e()));
            }
            0xa4 => {
                self.regs.set_h(helpers::res(4, self.regs.h()));
            }
            0xa5 => {
                self.regs.set_l(helpers::res(4, self.regs.l()));
            }
            0xa6 => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::res(4, val));
            }
            0xa7 => {
                self.regs.set_a(helpers::res(4, self.regs.a()));
            }
            0xa8 => {
                self.regs.set_b(helpers::res(5, self.regs.b()));
            }
            0xa9 => {
                self.regs.set_c(helpers::res(5, self.regs.c()));
            }
            0xaa => {
                self.regs.set_d(helpers::res(5, self.regs.d()));
            }
            0xab => {
                self.regs.set_e(helpers::res(5, self.regs.e()));
            }
            0xac => {
                self.regs.set_h(helpers::res(5, self.regs.h()));
            }
            0xad => {
                self.regs.set_l(helpers::res(5, self.regs.l()));
            }
            0xae => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::res(5, val));
            }
            0xaf => {
                self.regs.set_a(helpers::res(5, self.regs.a()));
            }
            0xb0 => {
                self.regs.set_b(helpers::res(6, self.regs.b()));
            }
            0xb1 => {
                self.regs.set_c(helpers::res(6, self.regs.c()));
            }
            0xb2 => {
                self.regs.set_d(helpers::res(6, self.regs.d()));
            }
            0xb3 => {
                self.regs.set_e(helpers::res(6, self.regs.e()));
            }
            0xb4 => {
                self.regs.set_h(helpers::res(6, self.regs.h()));
            }
            0xb5 => {
                self.regs.set_l(helpers::res(6, self.regs.l()));
            }
            0xb6 => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::res(6, val));
            }
            0xb7 => {
                self.regs.set_a(helpers::res(6, self.regs.a()));
            }
            0xb8 => {
                self.regs.set_b(helpers::res(7, self.regs.b()));
            }
            0xb9 => {
                self.regs.set_c(helpers::res(7, self.regs.c()));
            }
            0xba => {
                self.regs.set_d(helpers::res(7, self.regs.d()));
            }
            0xbb => {
                self.regs.set_e(helpers::res(7, self.regs.e()));
            }
            0xbc => {
                self.regs.set_h(helpers::res(7, self.regs.h()));
            }
            0xbd => {
                self.regs.set_l(helpers::res(7, self.regs.l()));
            }
            0xbe => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::res(7, val));
            }
            0xbf => {
                self.regs.set_a(helpers::res(7, self.regs.a()));
            }
            0xc0 => {
                self.regs.set_b(helpers::set(0, self.regs.b()));
            }
            0xc1 => {
                self.regs.set_c(helpers::set(0, self.regs.c()));
            }
            0xc2 => {
                self.regs.set_d(helpers::set(0, self.regs.d()));
            }
            0xc3 => {
                self.regs.set_e(helpers::set(0, self.regs.e()));
            }
            0xc4 => {
                self.regs.set_h(helpers::set(0, self.regs.h()));
            }
            0xc5 => {
                self.regs.set_l(helpers::set(0, self.regs.l()));
            }
            0xc6 => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::set(0, val));
            }
            0xc7 => {
                self.regs.set_a(helpers::set(0, self.regs.a()));
            }
            0xc8 => {
                self.regs.set_b(helpers::set(1, self.regs.b()));
            }
            0xc9 => {
                self.regs.set_c(helpers::set(1, self.regs.c()));
            }
            0xca => {
                self.regs.set_d(helpers::set(1, self.regs.d()));
            }
            0xcb => {
                self.regs.set_e(helpers::set(1, self.regs.e()));
            }
            0xcc => {
                self.regs.set_h(helpers::set(1, self.regs.h()));
            }
            0xcd => {
                self.regs.set_l(helpers::set(1, self.regs.l()));
            }
            0xce => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::set(1, val));
            }
            0xcf => {
                self.regs.set_a(helpers::set(1, self.regs.a()));
            }
            0xd0 => {
                self.regs.set_b(helpers::set(2, self.regs.b()));
            }
            0xd1 => {
                self.regs.set_c(helpers::set(2, self.regs.c()));
            }
            0xd2 => {
                self.regs.set_d(helpers::set(2, self.regs.d()));
            }
            0xd3 => {
                self.regs.set_e(helpers::set(2, self.regs.e()));
            }
            0xd4 => {
                self.regs.set_h(helpers::set(2, self.regs.h()));
            }
            0xd5 => {
                self.regs.set_l(helpers::set(2, self.regs.l()));
            }
            0xd6 => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::set(2, val));
            }
            0xd7 => {
                self.regs.set_a(helpers::set(2, self.regs.a()));
            }
            0xd8 => {
                self.regs.set_b(helpers::set(3, self.regs.b()));
            }
            0xd9 => {
                self.regs.set_c(helpers::set(3, self.regs.c()));
            }
            0xda => {
                self.regs.set_d(helpers::set(3, self.regs.d()));
            }
            0xdb => {
                self.regs.set_e(helpers::set(3, self.regs.e()));
            }
            0xdc => {
                self.regs.set_h(helpers::set(3, self.regs.h()));
            }
            0xdd => {
                self.regs.set_l(helpers::set(3, self.regs.l()));
            }
            0xde => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::set(3, val));
            }
            0xdf => {
                self.regs.set_a(helpers::set(3, self.regs.a()));
            }
            0xe0 => {
                self.regs.set_b(helpers::set(4, self.regs.b()));
            }
            0xe1 => {
                self.regs.set_c(helpers::set(4, self.regs.c()));
            }
            0xe2 => {
                self.regs.set_d(helpers::set(4, self.regs.d()));
            }
            0xe3 => {
                self.regs.set_e(helpers::set(4, self.regs.e()));
            }
            0xe4 => {
                self.regs.set_h(helpers::set(4, self.regs.h()));
            }
            0xe5 => {
                self.regs.set_l(helpers::set(4, self.regs.l()));
            }
            0xe6 => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::set(4, val));
            }
            0xe7 => {
                self.regs.set_a(helpers::set(4, self.regs.a()));
            }
            0xe8 => {
                self.regs.set_b(helpers::set(5, self.regs.b()));
            }
            0xe9 => {
                self.regs.set_c(helpers::set(5, self.regs.c()));
            }
            0xea => {
                self.regs.set_d(helpers::set(5, self.regs.d()));
            }
            0xeb => {
                self.regs.set_e(helpers::set(5, self.regs.e()));
            }
            0xec => {
                self.regs.set_h(helpers::set(5, self.regs.h()));
            }
            0xed => {
                self.regs.set_l(helpers::set(5, self.regs.l()));
            }
            0xee => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::set(5, val));
            }
            0xef => {
                self.regs.set_a(helpers::set(5, self.regs.a()));
            }
            0xf0 => {
                self.regs.set_b(helpers::set(6, self.regs.b()));
            }
            0xf1 => {
                self.regs.set_c(helpers::set(6, self.regs.c()));
            }
            0xf2 => {
                self.regs.set_d(helpers::set(6, self.regs.d()));
            }
            0xf3 => {
                self.regs.set_e(helpers::set(6, self.regs.e()));
            }
            0xf4 => {
                self.regs.set_h(helpers::set(6, self.regs.h()));
            }
            0xf5 => {
                self.regs.set_l(helpers::set(6, self.regs.l()));
            }
            0xf6 => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::set(6, val));
            }
            0xf7 => {
                self.regs.set_a(helpers::set(6, self.regs.a()));
            }
            0xf8 => {
                self.regs.set_b(helpers::set(7, self.regs.b()));
            }
            0xf9 => {
                self.regs.set_c(helpers::set(7, self.regs.c()));
            }
            0xfa => {
                self.regs.set_d(helpers::set(7, self.regs.d()));
            }
            0xfb => {
                self.regs.set_e(helpers::set(7, self.regs.e()));
            }
            0xfc => {
                self.regs.set_h(helpers::set(7, self.regs.h()));
            }
            0xfd => {
                self.regs.set_l(helpers::set(7, self.regs.l()));
            }
            0xfe => {
                let val = self.read_byte(self.regs.hl());
                self.write_byte(self.regs.hl(), helpers::set(7, val));
            }
            0xff => {
                self.regs.set_a(helpers::set(7, self.regs.a()));
            }
        }
    }
}
