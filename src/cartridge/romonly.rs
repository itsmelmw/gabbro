use crate::cartridge::Mbc;

pub struct RomOnly<'a> {
    rom: &'a [u8],
}

impl<'a> RomOnly<'a> {
    pub fn new(rom: &'a [u8]) -> Self {
        Self { rom }
    }
}

impl Mbc for RomOnly<'_> {
    fn read_rom(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
    fn write_rom(&mut self, _addr: u16, _val: u8) {}
    fn read_ram(&self, _addr: u16) -> u8 {
        0xff
    }
    fn write_ram(&mut self, _addr: u16, _val: u8) {}
    fn shutdown(&self) {}
}

//impl<const RAM: bool, const BAT: bool> Mbc for NoMbc<'_, RAM, BAT> {
//    fn name(&self) -> &'static str {
//        "ROM ONLY"
//    }
//    fn read_rom(&self, addr: u16) -> u8 {
//        self.rom[addr as usize]
//    }
//    fn write_rom(&mut self, _addr: u16, _val: u8) {}
//    fn read_ram(&self, addr: u16) -> u8 {
//        self.ram[addr as usize]
//    }
//    fn write_ram(&mut self, addr: u16, val: u8) {
//        self.ram[addr as usize] = val;
//    }
//}
//
//impl<const BAT: bool> Mbc for NoMbc<'_, true, BAT> {
//    fn name(&self) -> &'static str {
//        "ROM + RAM"
//    }
//    fn read_rom(&self, addr: u16) -> u8 {
//        self.rom[addr as usize]
//    }
//    fn write_rom(&mut self, _addr: u16, _val: u8) {}
//    fn read_ram(&self, addr: u16) -> u8 {
//        self.ram[addr as usize]
//    }
//    fn write_ram(&mut self, addr: u16, val: u8) {
//        self.ram[addr as usize] = val;
//    }
//}
