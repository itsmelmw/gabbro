use crate::{
    cpu::Cpu,
    peripherals::{Joypad, Lcd, Serial},
};

/// For invalid instructions.
pub fn invalid() {
    log::error!("Encountered an invalid instruction.");
    panic!("Aborting");
}

/// Jump to address `addr`.
pub fn jp<L, J, S>(cpu: &mut Cpu<L, J, S>, addr: u16)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    cpu.regs.set_pc(addr);
}

/// Jump to relative address `PC + val`.
pub fn jr<L, J, S>(cpu: &mut Cpu<L, J, S>, val: i8)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    cpu.regs.set_pc(cpu.regs.pc().wrapping_add(val as u16));
}

/// Push `PC` to the stack, and jump to address `addr`. Takes two machine cycles.
pub fn call<L, J, S>(cpu: &mut Cpu<L, J, S>, addr: u16)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    cpu.stack_push(cpu.regs.pc());
    jp(cpu, addr);
}

/// Pop a value from the stack and jump to it. Takes two machine cycles.
pub fn ret<L, J, S>(cpu: &mut Cpu<L, J, S>)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let addr = cpu.stack_pop();
    jp(cpu, addr);
}

/// Calculates the binary-coded decimal of `A` right after an addition or subtraction.
/// Flags: `Z-0C`.
pub fn daa<L, J, S>(cpu: &mut Cpu<L, J, S>)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let mut res = cpu.regs.a() as u16;
    if !cpu.regs.flags().n() {
        if cpu.regs.flags().h() || (res & 0x0f) > 0x09 {
            res = res.wrapping_add(0x06);
        }
        if cpu.regs.flags().c() || res > 0x9f {
            res = res.wrapping_add(0x60);
            cpu.regs.flags_mut().set_c(true);
        }
    } else {
        if cpu.regs.flags().h() {
            res = res.wrapping_sub(0x06);
        }
        if cpu.regs.flags().c() {
            res = res.wrapping_sub(0x60);
        }
    }

    cpu.regs.flags_mut().set_z(res as u8 == 0);
    cpu.regs.flags_mut().set_h(false);

    cpu.regs.set_a(res as u8);
}

/// Flips the bits of `A`.
/// Flags: `-11-`.
pub fn cpl<L, J, S>(cpu: &mut Cpu<L, J, S>)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    cpu.regs.flags_mut().set_n(true);
    cpu.regs.flags_mut().set_h(true);

    cpu.regs.set_a(!cpu.regs.a());
}

/// Sets the carry flag
/// Flags: `-001`.
pub fn scf<L, J, S>(cpu: &mut Cpu<L, J, S>)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(true);
}

/// Flips the carry flag
/// Flags: `-00C`.
pub fn ccf<L, J, S>(cpu: &mut Cpu<L, J, S>)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let c = cpu.regs.flags().c();

    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(!c);
}

/// Returns `val + 1`.
/// Flags: `Z0H-`.
pub fn inc<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8) -> u8
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let res = val.wrapping_add(1);

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h((val & 0x0f) == 0x0f);

    res
}

/// Returns `val - 1`.
/// Flags: `Z1H-`.
pub fn dec<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8) -> u8
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let res = val.wrapping_sub(1);

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(true);
    cpu.regs.flags_mut().set_h((val & 0x0f) == 0x00);

    res
}

/// Adds `val` to `A`.
/// Flags: `Z0HC`.
pub fn add<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let a = cpu.regs.a();
    let res = a as u16 + val as u16;

    cpu.regs.flags_mut().set_z(res as u8 == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h((a & 0x0f) + (val & 0x0f) > 0x0f);
    cpu.regs.flags_mut().set_c(res > 0xff);

    cpu.regs.set_a(res as u8);
}

/// Adds `val` to `HL`.
/// Flags: `-0HC`.
pub fn add_hl<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u16)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let hl = cpu.regs.hl();
    let res = hl as u32 + val as u32;

    cpu.regs.flags_mut().set_n(false);
    cpu.regs
        .flags_mut()
        .set_h((hl & 0xfff) + (val & 0xfff) > 0xfff);
    cpu.regs.flags_mut().set_c(res > 0xffff);

    cpu.regs.set_hl(res as u16);
}

/// Returns `SP + val`.
/// Flags: `00HC`.
pub fn add_sp<L, J, S>(cpu: &mut Cpu<L, J, S>, val: i8) -> u16
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let uval = val as u16;
    let sp = cpu.regs.sp();
    let res = sp.wrapping_add(uval);

    cpu.regs.flags_mut().set_z(false);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs
        .flags_mut()
        .set_h((sp & 0x0f) + (uval & 0x0f) > 0x0f);
    cpu.regs
        .flags_mut()
        .set_c((sp & 0xff) + (uval & 0xff) > 0xff);

    res
}

/// Adds `val` and the carry flag to `A`.
/// Flags: `Z0HC`.
pub fn adc<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let a = cpu.regs.a();
    let c = cpu.regs.flags().c() as u8;
    let res = a as u16 + val as u16 + c as u16;

    cpu.regs.flags_mut().set_z(res as u8 == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs
        .flags_mut()
        .set_h((a & 0x0f) + (val & 0x0f) + c > 0x0f);
    cpu.regs.flags_mut().set_c(res > 0xff);

    cpu.regs.set_a(res as u8);
}

/// Subtracts `val` from `A`.
/// Flags: `Z1HC`.
pub fn sub<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let a = cpu.regs.a();
    let res = a.wrapping_sub(val);

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(true);
    cpu.regs.flags_mut().set_h((a & 0x0f) < (val & 0x0f));
    cpu.regs.flags_mut().set_c(a < val);

    cpu.regs.set_a(res);
}

/// Subtracts `val` and the carry flag from `A`.
/// Flags: `Z1HC`.
pub fn sbc<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let a = cpu.regs.a();
    let c = cpu.regs.flags().c() as u8;
    let res = (a as u16).wrapping_sub(val as u16).wrapping_sub(c as u16);

    cpu.regs.flags_mut().set_z(res as u8 == 0);
    cpu.regs.flags_mut().set_n(true);
    cpu.regs.flags_mut().set_h((a & 0x0f) < ((val & 0x0f) + c));
    cpu.regs.flags_mut().set_c(res > 0xff);

    cpu.regs.set_a(res as u8);
}

/// Sets `A` to the bitwise AND of `A` and `val`.
/// Flags: `Z010`.
pub fn and<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let a = cpu.regs.a();
    let res = a & val;

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(true);
    cpu.regs.flags_mut().set_c(false);

    cpu.regs.set_a(res);
}

/// Sets `A` to the bitwise XOR of `A` and `val`.
/// Flags: `Z000`.
pub fn xor<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let a = cpu.regs.a();
    let res = a ^ val;

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(false);

    cpu.regs.set_a(res);
}

/// Sets `A` to the bitwise OR of `A` and `val`.
/// Flags: `Z000`.
pub fn or<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let a = cpu.regs.a();
    let res = a | val;

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(false);

    cpu.regs.set_a(res);
}

/// Compares `A` and `val`, and sets flags accordingly.
/// Flags are set as if `val` is subtracted from `A`.
/// Flags: `Z1HC`.
pub fn cp<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let a = cpu.regs.a();

    cpu.regs.flags_mut().set_z(a == val);
    cpu.regs.flags_mut().set_n(true);
    cpu.regs.flags_mut().set_h((a & 0x0f) < (val & 0x0f));
    cpu.regs.flags_mut().set_c(a < val);
}

/// Rotate `val` to the left once.
/// Flags: `Z00C`.
pub fn rlc<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8) -> u8
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let bit = val >> 7;
    let res = (val << 1) | bit;

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(bit != 0);

    res
}

/// Rotate `val` to the right once.
/// Flags: `Z00C`.
pub fn rrc<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8) -> u8
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let bit = val & 0x01;
    let res = (val >> 1) | (bit << 7);

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(bit != 0);

    res
}

/// Rotate `val` to the left once, through the carry flag.
/// Flags: `Z00C`.
pub fn rl<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8) -> u8
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let c = cpu.regs.flags().c() as u8;
    let bit = val >> 7;
    let res = (val << 1) | c;

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(bit != 0);

    res
}

/// Rotate `val` to the right once, through the carry flag.
/// Flags: `Z00C`.
pub fn rr<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8) -> u8
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let c = cpu.regs.flags().c() as u8;
    let bit = val & 0x01;
    let res = (val >> 1) | (c << 7);

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(bit != 0);

    res
}

/// Shift `val` to the left once, into the carry flag.
/// Least significant bit is set to 0.
/// Flags: `Z00C`.
pub fn sla<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8) -> u8
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let bit = val >> 7;
    let res = val << 1;

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(bit != 0);

    res
}

/// Shift `val` to the right once, into the carry flag.
/// Most significant bit does not change.
/// Flags: `Z00C`.
pub fn sra<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8) -> u8
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let bit = val & 0x01;
    let res = (val & 0x80) | (val >> 1);

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(bit != 0);

    res
}

/// Swaps the upper and lower nibble of `val`.
/// Flags: `Z000`.
pub fn swap<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8) -> u8
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let res = (val << 4) | (val >> 4);

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(false);

    res
}

/// Shift `val` to the right once, into the carry flag.
/// Most significant bit is set to 0.
/// Flags: `Z00C`.
pub fn srl<L, J, S>(cpu: &mut Cpu<L, J, S>, val: u8) -> u8
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    let bit = val & 0x01;
    let res = val >> 1;

    cpu.regs.flags_mut().set_z(res == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(false);
    cpu.regs.flags_mut().set_c(bit != 0);

    res
}

/// Checks if the `bit`th bit of `val` is set,
/// and sets the zero flag accordingly.
/// Flags: `Z01-`.
pub fn bit<L, J, S>(cpu: &mut Cpu<L, J, S>, bit: u8, val: u8)
where
    L: Lcd,
    J: Joypad,
    S: Serial,
{
    cpu.regs.flags_mut().set_z((val & (1 << bit)) == 0);
    cpu.regs.flags_mut().set_n(false);
    cpu.regs.flags_mut().set_h(true);
}

/// Resets the `bit`th bit of val.
/// Flags: `----`.
pub fn res(bit: u8, val: u8) -> u8 {
    val & !(1 << bit)
}

/// Sets the `bit`th bit of val.
/// Flags: `----`.
pub fn set(bit: u8, val: u8) -> u8 {
    val | (1 << bit)
}
