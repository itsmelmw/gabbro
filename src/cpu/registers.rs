/// The CPU flags register `F`. Stores the `Z`, `N`, `H` and `C` flags.
#[derive(Clone, Copy)]
pub struct Flags {
    byte: u8,
}

impl Flags {
    /// Reads the value of the Carry flag `C`.
    pub fn c(&self) -> bool {
        (self.byte >> 4) & 1 != 0
    }
    /// Reads the value of the Half Carry flag `H`.
    pub fn h(&self) -> bool {
        (self.byte >> 5) & 1 != 0
    }
    /// Reads the value of the Subtract flag `N`.
    pub fn n(&self) -> bool {
        (self.byte >> 6) & 1 != 0
    }
    /// Reads the value of the Zero flag `Z`.
    pub fn z(&self) -> bool {
        (self.byte >> 7) & 1 != 0
    }
    /// Writes `val as u8` to the Carry flag `C`.
    pub fn set_c(&mut self, val: bool) {
        self.byte = (self.byte & !(1 << 4)) | (val as u8) << 4
    }
    /// Writes `val as u8` to the Half Carry flag `H`.
    pub fn set_h(&mut self, val: bool) {
        self.byte = (self.byte & !(1 << 5)) | (val as u8) << 5
    }
    /// Writes `val as u8` to the Subtract flag `N`.
    pub fn set_n(&mut self, val: bool) {
        self.byte = (self.byte & !(1 << 6)) | (val as u8) << 6
    }
    /// Writes `val as u8` to the Zero flag `Z`.
    pub fn set_z(&mut self, val: bool) {
        self.byte = (self.byte & !(1 << 7)) | (val as u8) << 7
    }
}

/// Stores the values of all 8-bit CPU registers.
#[derive(Clone, Copy)]
#[repr(C)]
struct R8 {
    f: u8,
    a: u8,
    c: u8,
    b: u8,
    e: u8,
    d: u8,
    l: u8,
    h: u8,
}

/// Stores the values of all 16-bit CPU registers.
#[derive(Clone, Copy)]
#[repr(C)]
struct R16 {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

/// A union of all CPU registers, overlapping correctly.
pub union Regs {
    flags: Flags,
    r8: R8,
    r16: R16,
}

impl Regs {
    /// Initializes a new set of CPU registers.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            r16: R16 {
                af: 0x01b0,
                bc: 0x0013,
                de: 0x00d8,
                hl: 0x014d,
                pc: 0x0100,
                sp: 0xfffe,
            },
        }
    }

    /// Reads the value from the `A` register.
    pub fn a(&self) -> u8 {
        unsafe { self.r8.a }
    }

    /// Writes `val` to the `A` register.
    pub fn set_a(&mut self, val: u8) {
        self.r8.a = val;
    }

    /// Retrieves the `Flags` containing the flags in the `F` register.
    pub fn flags(&self) -> &Flags {
        unsafe { &self.flags }
    }

    /// Retrieves a mutable reference to `Flags` containing the flags in the `F` register.
    pub fn flags_mut(&mut self) -> &mut Flags {
        unsafe { &mut self.flags }
    }

    /// Reads the value from the `B` register.
    pub fn b(&self) -> u8 {
        unsafe { self.r8.b }
    }

    /// Writes `val` to the `B` register.
    pub fn set_b(&mut self, val: u8) {
        self.r8.b = val
    }

    /// Reads the value from the `C` register.
    pub fn c(&self) -> u8 {
        unsafe { self.r8.c }
    }

    /// Writes `val` to the `C` register.
    pub fn set_c(&mut self, val: u8) {
        self.r8.c = val
    }

    /// Reads the value from the `D` register.
    pub fn d(&self) -> u8 {
        unsafe { self.r8.d }
    }

    /// Writes `val` to the `D` register.
    pub fn set_d(&mut self, val: u8) {
        self.r8.d = val
    }

    /// Reads the value from the `E` register.
    pub fn e(&self) -> u8 {
        unsafe { self.r8.e }
    }

    /// Writes `val` to the `E` register.
    pub fn set_e(&mut self, val: u8) {
        self.r8.e = val
    }

    /// Reads the value from the `H` register.
    pub fn h(&self) -> u8 {
        unsafe { self.r8.h }
    }

    /// Writes `val` to the `H` register.
    pub fn set_h(&mut self, val: u8) {
        self.r8.h = val
    }

    /// Reads the value from the `L` register.
    pub fn l(&self) -> u8 {
        unsafe { self.r8.l }
    }

    /// Writes `val` to the `L` register.
    pub fn set_l(&mut self, val: u8) {
        self.r8.l = val
    }

    /// Reads the value from the `AF` register.
    pub fn af(&self) -> u16 {
        unsafe { self.r16.af }
    }

    /// Writes `val` to the `AF` register.
    pub fn set_af(&mut self, val: u16) {
        self.r16.af = val & 0xfff0;
    }

    /// Reads the value from the `BC` register.
    pub fn bc(&self) -> u16 {
        unsafe { self.r16.bc }
    }

    /// Writes `val` to the `BC` register.
    pub fn set_bc(&mut self, val: u16) {
        self.r16.bc = val
    }

    /// Reads the value from the `DE` register.
    pub fn de(&self) -> u16 {
        unsafe { self.r16.de }
    }

    /// Writes `val` to the `DE` register.
    pub fn set_de(&mut self, val: u16) {
        self.r16.de = val
    }

    /// Reads the value from the `HL` register.
    pub fn hl(&self) -> u16 {
        unsafe { self.r16.hl }
    }

    /// Writes `val` to the `HL` register.
    pub fn set_hl(&mut self, val: u16) {
        self.r16.hl = val
    }

    /// Reads the value from the `PC` register.
    pub fn pc(&self) -> u16 {
        unsafe { self.r16.pc }
    }

    /// Writes `val` to the `PC` register.
    pub fn set_pc(&mut self, val: u16) {
        self.r16.pc = val
    }

    /// Reads the value from the `SP` register.
    pub fn sp(&self) -> u16 {
        unsafe { self.r16.sp }
    }

    /// Writes `val` to the `SP` register.
    pub fn set_sp(&mut self, val: u16) {
        self.r16.sp = val
    }

    /// Increments `PC`.
    pub(super) fn inc_pc(&mut self) {
        unsafe { self.r16.pc = self.r16.pc.wrapping_add(1) }
    }

    /// Increments `SP`.
    pub(super) fn inc_sp(&mut self) {
        unsafe { self.r16.sp = self.r16.sp.wrapping_add(1) }
    }

    /// Decrements `SP`.
    pub(super) fn dec_sp(&mut self) {
        unsafe { self.r16.sp = self.r16.sp.wrapping_sub(1) }
    }

    /// Increments `HL`.
    pub(super) fn inc_hl(&mut self) {
        unsafe { self.r16.hl = self.r16.hl.wrapping_add(1) }
    }

    /// Decrements `HL`.
    pub(super) fn dec_hl(&mut self) {
        unsafe { self.r16.hl = self.r16.hl.wrapping_sub(1) }
    }
}
