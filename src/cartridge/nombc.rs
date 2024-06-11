use crate::cartridge::Mbc;
use std::fmt;

pub struct NoMbc {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl NoMbc {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            ram: vec![0; 0x2000],
        }
    }
}

impl Mbc for NoMbc {
    fn read_rom(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
    fn write_rom(&mut self, _: u16, _: u8) {}
    fn read_ram(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }
    fn write_ram(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }
}

impl fmt::Display for NoMbc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No MBC")
    }
}
