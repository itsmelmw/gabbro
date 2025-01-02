use crate::cartridge::{
    peripherals::{Battery, Feat, NoFeat, NoRam, Ram, Rtc},
    Cartridge, CartridgeError, Mbc, ReadRam, ReadRom, Shutdown, ShutdownError, WriteRam, WriteRom,
};

pub struct Mbc3<'a, RAM, BAT, RTC>
where
    BAT: Battery,
    RTC: Rtc,
{
    ram_enable: usize,
    rom_bank: usize,
    ram_bank: usize,
    bank_mode: usize,

    addr_mask: usize,
    rom: &'a [u8],
    ram: RAM,
    battery: BAT,
    rtc: RTC,
}

impl<'a, RAM, BAT, RTC> Mbc3<'a, RAM, BAT, RTC>
where
    BAT: Battery,
    RTC: Rtc,
{
    pub fn from_parts(rom: &'a [u8], ram: RAM, battery: BAT, rtc: RTC) -> Self {
        let rom_banks = rom.len() / Cartridge::<BAT>::ROM_BANK_SIZE;
        Self {
            ram_enable: 0,
            rom_bank: 0,
            ram_bank: 0,
            bank_mode: 0,
            addr_mask: 0x3fff | ((rom_banks - 1) << 14),
            rom,
            ram,
            battery,
            rtc,
        }
    }
}

impl<'a> Mbc3<'a, NoRam, NoFeat, NoFeat> {
    pub fn rom_only(rom: &'a [u8]) -> Self {
        Self::from_parts(rom, (), NoFeat, NoFeat)
    }
}

impl<'a> Mbc3<'a, Ram, NoFeat, NoFeat> {
    pub fn with_ram(rom: &'a [u8], ram: Ram) -> Self {
        Self::from_parts(rom, ram, NoFeat, NoFeat)
    }
}

impl<'a, BAT> Mbc3<'a, Ram, Feat<BAT>, NoFeat>
where
    BAT: Battery,
{
    pub fn with_ram_battery(
        rom: &'a [u8],
        ram: Ram,
        mut battery: BAT,
    ) -> Result<Self, CartridgeError<BAT>> {
        match battery.load_data() {
            Ok(Some(data)) => Ok(Self::from_parts(rom, data, Feat(battery), NoFeat)),
            Ok(None) => Ok(Self::from_parts(rom, ram, Feat(battery), NoFeat)),
            Err(err) => Err(CartridgeError::BatteryError(err)),
        }
    }
}

impl<RAM, BAT, RTC> ReadRom for Mbc3<'_, RAM, BAT, RTC>
where
    BAT: Battery,
    RTC: Rtc,
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

impl<RAM, BAT, RTC> WriteRom for Mbc3<'_, RAM, BAT, RTC>
where
    BAT: Battery,
    RTC: Rtc,
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

impl<BAT, RTC> ReadRam for Mbc3<'_, NoRam, BAT, RTC>
where
    BAT: Battery,
    RTC: Rtc,
{
    fn read_ram(&self, _addr: u16) -> u8 {
        0xff
    }
}

impl<BAT, RTC> WriteRam for Mbc3<'_, NoRam, BAT, RTC>
where
    BAT: Battery,
    RTC: Rtc,
{
    fn write_ram(&mut self, _addr: u16, _val: u8) {}
}

impl<BAT, RTC> ReadRam for Mbc3<'_, Ram, BAT, RTC>
where
    BAT: Battery,
    RTC: Rtc,
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

impl<BAT, RTC> WriteRam for Mbc3<'_, Ram, BAT, RTC>
where
    BAT: Battery,
    RTC: Rtc,
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

impl<RAM, BAT, RTC> Shutdown<BAT> for Mbc3<'_, RAM, NoFeat, RTC>
where
    BAT: Battery,
    RTC: Rtc,
{
    fn shutdown(&mut self) -> Result<(), ShutdownError<BAT>> {
        Ok(())
    }
}

impl<BAT, RTC> Shutdown<BAT> for Mbc3<'_, Ram, Feat<BAT>, RTC>
where
    BAT: Battery,
    RTC: Rtc,
{
    fn shutdown(&mut self) -> Result<(), ShutdownError<BAT>> {
        self.battery
            .store_data(&self.ram)
            .map_err(|err| ShutdownError::BatteryError(err))
    }
}

impl<BAT> Mbc<BAT> for Mbc3<'_, NoRam, NoFeat, NoFeat>
where
    BAT: Battery,
{
    fn name(&self) -> &'static str {
        "MBC3"
    }
}

impl<BAT> Mbc<BAT> for Mbc3<'_, Ram, NoFeat, NoFeat>
where
    BAT: Battery,
{
    fn name(&self) -> &'static str {
        "MBC3 + RAM"
    }
}

impl<BAT> Mbc<BAT> for Mbc3<'_, Ram, Feat<BAT>, NoFeat>
where
    BAT: Battery,
{
    fn name(&self) -> &'static str {
        "MBC3 + RAM + BATTERY"
    }
}
