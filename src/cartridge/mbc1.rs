use crate::cartridge::Mbc;
use std::fmt;

/// A memory bank controller of type 1.
/// Stores its registers, as well as ROM and RAM.
pub struct Mbc1 {
    ram_enable: usize,
    rom_bank: usize,
    ram_bank: usize,
    bank_mode: usize,

    addr_mask: usize,
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Mbc1 {
    /// Creates a new memory bank controller of type 1.
    pub fn new(rom: Vec<u8>, rom_banks: usize) -> Self {
        Self {
            ram_enable: 0,
            rom_bank: 0,
            ram_bank: 0,
            bank_mode: 0,
            addr_mask: 0x3fff | ((rom_banks - 1) << 14),
            rom,
            ram: vec![0; 0x2000],
        }
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, addr: u16) -> u8 {
        // Bit 00 - 13 decided by address
        let base_addr = addr as usize & 0x3fff;
        // Bit 14 - 18 decided by rom bank
        let bank_addr = if addr <= 0x3fff {
            0
        } else if self.rom_bank == 0 {
            1
        } else {
            self.rom_bank & 0x1f
        };
        // Bit 19 - 20 decided by mode and ram bank
        let mode_addr = (self.bank_mode * self.ram_bank) & 0x03;

        self.rom[(base_addr | bank_addr << 14 | mode_addr << 19) & self.addr_mask]
    }

    fn write_rom(&mut self, addr: u16, val: u8) {
        if addr <= 0x1fff {
            self.ram_enable = val as usize & 0x0f;
        } else if addr <= 0x3fff {
            self.rom_bank = val as usize & 0x1f;
        } else if addr <= 0x5fff {
            self.ram_bank = val as usize & 0x03;
        } else if addr <= 0x7fff {
            self.bank_mode = val as usize & 0x01;
        }
    }

    fn read_ram(&self, addr: u16) -> u8 {
        if self.ram_enable != 0x0a {
            return 0xff;
        }
        // Bit 00 - 12 decided by address
        let base_addr = addr as usize;
        // Bit 13 - 14 decided by ram bank
        let bank_addr = self.ram_bank;
        self.ram[base_addr | bank_addr << 13]
    }

    fn write_ram(&mut self, addr: u16, val: u8) {
        if self.ram_enable != 0x0a {
            return;
        }
        // Bit 00 - 12 decided by address
        let base_addr = addr as usize;
        // Bit 13 - 14 decided by ram bank
        let bank_addr = self.ram_bank;
        self.ram[base_addr | bank_addr << 13] = val;
    }
}

impl fmt::Display for Mbc1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MBC1")
    }
}
