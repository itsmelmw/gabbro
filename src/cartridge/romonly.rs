use crate::cartridge::{Mbc, ReadRam, ReadRom, Shutdown, WriteRam, WriteRom};

pub struct RomOnly<'a> {
    rom: &'a [u8],
}

impl<'a> RomOnly<'a> {
    pub fn new(rom: &'a [u8]) -> Self {
        Self { rom }
    }
}

impl ReadRom for RomOnly<'_> {
    fn read_rom(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}

impl WriteRom for RomOnly<'_> {
    fn write_rom(&mut self, _addr: u16, _val: u8) {}
}

impl ReadRam for RomOnly<'_> {
    fn read_ram(&self, _addr: u16) -> u8 {
        0xff
    }
}

impl WriteRam for RomOnly<'_> {
    fn write_ram(&mut self, _addr: u16, _val: u8) {}
}

impl Shutdown for RomOnly<'_> {
    fn shutdown(&mut self) {}
}

impl Mbc for RomOnly<'_> {
    fn name(&self) -> &'static str {
        "ROM ONLY"
    }
}
