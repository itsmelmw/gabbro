use crate::cartridge::{
    peripherals::{Battery, Clock, Feat, NoFeat, NoRam, Ram},
    Cartridge, CartridgeError, GetRom, Mbc, ReadRam, ReadRom, Shutdown, ShutdownError, WriteRam,
    WriteRom,
};

pub struct Rtc<CLK>
where
    CLK: Clock,
{
    clock: CLK,
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub days_lower: u8,
    pub days_upper: u8,
}

impl<CLK> Rtc<CLK>
where
    CLK: Clock,
{
    pub fn new(clock: CLK) -> Self {
        Self {
            clock,
            seconds: 0,
            minutes: 0,
            hours: 0,
            days_lower: 0,
            days_upper: 0,
        }
    }

    pub fn latch(&mut self) {
        let mut passed = self.clock.seconds_passed();

        self.seconds += (passed % 60) as u8;
        passed /= 60;
        if self.seconds >= 60 {
            self.seconds -= 60;
            passed += 1;
        }

        self.minutes += (passed % 60) as u8;
        passed /= 60;
        if self.minutes >= 60 {
            self.minutes -= 60;
            passed += 1;
        }

        self.hours += (passed % 24) as u8;
        passed /= 24;
        if self.hours >= 24 {
            self.hours -= 24;
            passed += 1;
        }

        let (new, carry) = self.days_lower.overflowing_add((passed & 0xff) as u8);
        self.days_lower = new;

        let new = (passed >> 8) as u8 + carry as u8 + (self.days_upper & 0x01);
        self.days_upper =
            ((((new & 0xfe) != 0) as u8) << 7) | (self.days_upper & 0x40) | (new & 0x01);
    }
}

pub struct Mbc3<'a, RAM, BAT, CLK>
where
    BAT: Battery,
    CLK: Clock,
{
    ram_enable: usize,
    rom_bank: usize,
    ram_rtc_bank: usize,
    latch: usize,

    addr_mask: usize,
    rom: &'a [u8],
    ram: RAM,
    battery: BAT,
    rtc: Rtc<CLK>,
}

impl<'a, RAM, BAT, CLK> Mbc3<'a, RAM, BAT, CLK>
where
    BAT: Battery,
    CLK: Clock,
{
    pub fn from_parts(rom: &'a [u8], ram: RAM, battery: BAT, clock: CLK) -> Self {
        let rom_banks = rom.len() / Cartridge::<BAT>::ROM_BANK_SIZE;
        Self {
            ram_enable: 0,
            rom_bank: 0,
            ram_rtc_bank: 0,
            latch: 0,
            addr_mask: 0x3fff | ((rom_banks - 1) << 14),
            rom,
            ram,
            battery,
            rtc: Rtc::new(clock),
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

impl<'a, BAT, CLK> Mbc3<'a, NoRam, Feat<BAT>, Feat<CLK>>
where
    BAT: Battery,
    CLK: Clock,
{
    pub fn with_battery_clock(
        rom: &'a [u8],
        battery: BAT,
        clock: CLK,
    ) -> Result<Self, CartridgeError<BAT>> {
        // TODO: Load clock data
        Ok(Self::from_parts(rom, (), Feat(battery), Feat(clock)))
    }
}

impl<'a, BAT, CLK> Mbc3<'a, Ram, Feat<BAT>, Feat<CLK>>
where
    BAT: Battery,
    CLK: Clock,
{
    pub fn with_ram_battery_clock(
        rom: &'a [u8],
        ram: Ram,
        mut battery: BAT,
        clock: CLK,
    ) -> Result<Self, CartridgeError<BAT>> {
        // TODO: Load clock data
        match battery.load_data() {
            Ok(Some(data)) => Ok(Self::from_parts(rom, data, Feat(battery), Feat(clock))),
            Ok(None) => Ok(Self::from_parts(rom, ram, Feat(battery), Feat(clock))),
            Err(err) => Err(CartridgeError::BatteryError(err)),
        }
    }
}

impl<RAM, BAT, CLK> GetRom for Mbc3<'_, RAM, BAT, CLK>
where
    BAT: Battery,
    CLK: Clock,
{
    fn rom(&self) -> &[u8] {
        self.rom
    }
}

impl<RAM, BAT, CLK> ReadRom for Mbc3<'_, RAM, BAT, CLK>
where
    BAT: Battery,
    CLK: Clock,
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

        self.rom[(base_addr | (bank_addr << 14)) & self.addr_mask]
    }
}

impl<RAM, BAT> WriteRom for Mbc3<'_, RAM, BAT, NoFeat>
where
    BAT: Battery,
{
    fn write_rom(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1fff => self.ram_enable = val as usize & 0x0f,
            0x2000..=0x3fff => self.rom_bank = val as usize & 0x1f,
            0x4000..=0x5fff => self.ram_rtc_bank = val as usize & 0x03,
            0x6000..=0x7fff => self.latch = val as usize & 0x01,
            _ => unreachable!(),
        }
    }
}

impl<RAM, BAT, CLK> WriteRom for Mbc3<'_, RAM, BAT, Feat<CLK>>
where
    BAT: Battery,
    CLK: Clock,
{
    fn write_rom(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1fff => self.ram_enable = val as usize & 0x0f,
            0x2000..=0x3fff => self.rom_bank = val as usize & 0x1f,
            0x4000..=0x5fff => self.ram_rtc_bank = val as usize & 0x0f,
            0x6000..=0x7fff => {
                if self.latch == 0 && val == 1 {
                    self.rtc.latch();
                }
                self.latch = val as usize & 0x01
            }
            _ => unreachable!(),
        }
    }
}

impl<BAT, CLK> ReadRam for Mbc3<'_, NoRam, BAT, CLK>
where
    BAT: Battery,
    CLK: Clock,
{
    fn read_ram(&self, _addr: u16) -> u8 {
        0xff
    }
}

impl<BAT, CLK> WriteRam for Mbc3<'_, NoRam, BAT, CLK>
where
    BAT: Battery,
    CLK: Clock,
{
    fn write_ram(&mut self, _addr: u16, _val: u8) {}
}

impl<BAT> ReadRam for Mbc3<'_, Ram, BAT, NoFeat>
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
        let bank_addr = self.ram_rtc_bank;
        self.ram[base_addr | (bank_addr << 13)]
    }
}

impl<BAT, CLK> ReadRam for Mbc3<'_, Ram, BAT, Feat<CLK>>
where
    BAT: Battery,
    CLK: Clock,
{
    fn read_ram(&self, addr: u16) -> u8 {
        if self.ram_enable != 0x0a {
            return 0xff;
        }
        if self.ram_rtc_bank <= 0x03 {
            // Bit 00 - 12 decided by address
            let base_addr = addr as usize;
            // Bit 13 - 14 decided by ram bank
            let bank_addr = self.ram_rtc_bank;
            self.ram[base_addr | (bank_addr << 13)]
        } else {
            match addr {
                0x08 => self.rtc.seconds,
                0x09 => self.rtc.minutes,
                0x0a => self.rtc.hours,
                0x0b => self.rtc.days_lower,
                0x0c => self.rtc.days_upper,
                _ => 0xff,
            }
        }
    }
}

impl<BAT> WriteRam for Mbc3<'_, Ram, BAT, NoFeat>
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
        let bank_addr = self.ram_rtc_bank;
        self.ram[base_addr | (bank_addr << 13)] = val;
    }
}

impl<BAT, CLK> WriteRam for Mbc3<'_, Ram, BAT, Feat<CLK>>
where
    BAT: Battery,
    CLK: Clock,
{
    fn write_ram(&mut self, addr: u16, val: u8) {
        if self.ram_enable != 0x0a {
            return;
        }
        if self.ram_rtc_bank <= 0x03 {
            // Bit 00 - 12 decided by address
            let base_addr = addr as usize;
            // Bit 13 - 14 decided by ram bank
            let bank_addr = self.ram_rtc_bank;
            self.ram[base_addr | (bank_addr << 13)] = val;
        } else {
            match addr {
                0x08 => self.rtc.seconds = val,
                0x09 => self.rtc.minutes = val,
                0x0a => self.rtc.hours = val,
                0x0b => self.rtc.days_lower = val,
                0x0c => self.rtc.days_upper = val,
                _ => {}
            }
        }
    }
}

impl<RAM, BAT, CLK> Shutdown<BAT> for Mbc3<'_, RAM, NoFeat, CLK>
where
    BAT: Battery,
    CLK: Clock,
{
    fn shutdown(&mut self) -> Result<(), ShutdownError<BAT>> {
        Ok(())
    }
}

impl<BAT, CLK> Shutdown<BAT> for Mbc3<'_, NoRam, Feat<BAT>, CLK>
where
    BAT: Battery,
    CLK: Clock,
{
    fn shutdown(&mut self) -> Result<(), ShutdownError<BAT>> {
        Ok(())
    }
}

impl<BAT, CLK> Shutdown<BAT> for Mbc3<'_, Ram, Feat<BAT>, CLK>
where
    BAT: Battery,
    CLK: Clock,
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
    fn cart_type(&self) -> &'static str {
        "MBC3"
    }
}

impl<BAT> Mbc<BAT> for Mbc3<'_, Ram, NoFeat, NoFeat>
where
    BAT: Battery,
{
    fn cart_type(&self) -> &'static str {
        "MBC3 + RAM"
    }
}

impl<BAT> Mbc<BAT> for Mbc3<'_, Ram, Feat<BAT>, NoFeat>
where
    BAT: Battery,
{
    fn cart_type(&self) -> &'static str {
        "MBC3 + RAM + BATTERY"
    }
}

impl<BAT, CLK> Mbc<BAT> for Mbc3<'_, NoRam, Feat<BAT>, Feat<CLK>>
where
    BAT: Battery,
    CLK: Clock,
{
    fn cart_type(&self) -> &'static str {
        "MBC3 + TIMER + BATTERY"
    }
}

impl<BAT, CLK> Mbc<BAT> for Mbc3<'_, Ram, Feat<BAT>, Feat<CLK>>
where
    BAT: Battery,
    CLK: Clock,
{
    fn cart_type(&self) -> &'static str {
        "MBC3 + TIMER + RAM + BATTERY"
    }
}
