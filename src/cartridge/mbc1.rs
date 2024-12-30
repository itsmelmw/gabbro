use crate::cartridge::{
    peripherals::{Battery, ExtRam, Feat, NoFeat, NoRam, Ram},
    Cartridge, Mbc, ReadRam, ReadRom, WriteRam, WriteRom,
};

/// A memory bank controller of type MBC1.
/// Stores its registers, as well as ROM and RAM.
pub struct Mbc1<'a, RAM, BAT>
where
    RAM: ExtRam,
    BAT: Battery,
{
    ram_enable: usize,
    rom_bank: usize,
    ram_bank: usize,
    bank_mode: usize,

    addr_mask: usize,
    rom: &'a [u8],
    ram: RAM,
    battery: BAT,
}

impl<'a, RAM, BAT> Mbc1<'a, RAM, BAT>
where
    RAM: ExtRam,
    BAT: Battery,
{
    /// Creates a new memory bank controller of type MBC1.
    pub fn from_parts(rom: &'a [u8], ram: RAM, battery: BAT) -> Self {
        let rom_banks = rom.len() / Cartridge::ROM_BANK_SIZE;
        Self {
            ram_enable: 0,
            rom_bank: 0,
            ram_bank: 0,
            bank_mode: 0,
            addr_mask: 0x3fff | ((rom_banks - 1) << 14),
            rom,
            ram,
            battery,
        }
    }
}

impl<'a> Mbc1<'a, NoRam, NoFeat> {
    pub fn rom_only(rom: &'a [u8]) -> Self {
        Self::from_parts(rom, (), NoFeat)
    }
}

impl<'a> Mbc1<'a, Ram, NoFeat> {
    /// Creates a new memory bank controller of type MBC1 with RAM.
    pub fn with_ram(rom: &'a [u8], ram: Ram) -> Self {
        Self::from_parts(rom, ram, NoFeat)
    }
}

impl<'a, BAT> Mbc1<'a, Ram, Feat<BAT>>
where
    BAT: Battery,
{
    /// Creates a new memory bank controller of type MBC1 with RAM and battery.
    pub fn with_ram_battery(rom: &'a [u8], mut ram: Ram, mut battery: BAT) -> Self {
        if let Some(data) = battery.load_data() {
            ram = data;
        }
        Self::from_parts(rom, ram, Feat(battery))
    }
}

impl<RAM, BAT> ReadRom for Mbc1<'_, RAM, BAT>
where
    RAM: ExtRam,
    BAT: Battery,
{
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

        self.rom[(base_addr | (bank_addr << 14) | (mode_addr << 19)) & self.addr_mask]
    }
}

impl<RAM, BAT> WriteRom for Mbc1<'_, RAM, BAT>
where
    RAM: ExtRam,
    BAT: Battery,
{
    fn write_rom(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1fff => self.ram_enable = val as usize & 0x0f,
            0x2000..=0x3fff => self.rom_bank = val as usize & 0x1f,
            0x4000..=0x5fff => self.ram_bank = val as usize & 0x03,
            0x6000..=0x7fff => self.bank_mode = val as usize & 0x01,
            _ => unreachable!(),
        }
    }
}

impl<BAT> ReadRam for Mbc1<'_, NoRam, BAT>
where
    BAT: Battery,
{
    fn read_ram(&self, _addr: u16) -> u8 {
        0xff
    }
}

impl<BAT> WriteRam for Mbc1<'_, NoRam, BAT>
where
    BAT: Battery,
{
    fn write_ram(&mut self, _addr: u16, _val: u8) {}
}

impl<BAT> ReadRam for Mbc1<'_, Ram, BAT>
where
    BAT: Battery,
{
    fn read_ram(&self, addr: u16) -> u8 {
        if self.ram_enable != 0x0a {
            return 0xff;
        }
        // Bit 00 - 12 decided by address
        let base_addr = addr as usize;
        // Bit 13 - 14 decided by ram bank
        let bank_addr = self.ram_bank;
        self.ram[base_addr | (bank_addr << 13)]
    }
}

impl<BAT> WriteRam for Mbc1<'_, Ram, BAT>
where
    BAT: Battery,
{
    fn write_ram(&mut self, addr: u16, val: u8) {
        if self.ram_enable != 0x0a {
            return;
        }
        // Bit 00 - 12 decided by address
        let base_addr = addr as usize;
        // Bit 13 - 14 decided by ram bank
        let bank_addr = self.ram_bank;
        self.ram[base_addr | (bank_addr << 13)] = val;
    }
}

// TODO: Fix without `Drop`. Could remove `ExtRam` as well then
// and just force RAM to be Vec<u8> or ().
impl<RAM, BAT> Drop for Mbc1<'_, RAM, BAT>
where
    RAM: ExtRam,
    BAT: Battery,
{
    fn drop(&mut self) {
        if let Some(data) = self.ram.data() {
            self.battery.store_data(data);
        }
    }
}

impl Mbc for Mbc1<'_, NoRam, NoFeat> {
    fn name(&self) -> &'static str {
        "MBC1"
    }
}

impl Mbc for Mbc1<'_, Ram, NoFeat> {
    fn name(&self) -> &'static str {
        "MBC1 + RAM"
    }
}

impl<BAT> Mbc for Mbc1<'_, Ram, Feat<BAT>>
where
    BAT: Battery,
{
    fn name(&self) -> &'static str {
        "MBC1 + RAM + BATTERY"
    }

    fn shutdown(&mut self) {
        self.battery.store_data(&self.ram);
    }
}
