pub mod mbc1;
pub mod mbc3;
pub mod peripherals;
pub mod romonly;
use mbc1::Mbc1;
use mbc3::Mbc3;
use peripherals::{Battery, Clock, Ram};
use romonly::RomOnly;
use std::str;

#[derive(Debug)]
pub enum CartridgeError<CB>
where
    CB: Battery,
{
    InvalidTitle,
    InvalidRomSize,
    WrongRomSize,
    InvalidRamSize,
    WrongSaveSize,
    InvalidLicensee,
    InvalidMbc,
    UnsupportedMbc(&'static str),
    BatteryError(CB::Error),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShutdownError<CB>
where
    CB: Battery,
{
    BatteryError(CB::Error),
}

pub trait GetRom {
    fn rom(&self) -> &[u8];
}

pub trait ReadRom {
    fn read_rom(&self, addr: u16) -> u8;
}

pub trait WriteRom {
    fn write_rom(&mut self, addr: u16, val: u8);
}

pub trait ReadRam {
    fn read_ram(&self, addr: u16) -> u8;
}

pub trait WriteRam {
    fn write_ram(&mut self, addr: u16, val: u8);
}

pub trait Shutdown<CB>
where
    CB: Battery,
{
    fn shutdown(&mut self) -> Result<(), ShutdownError<CB>>;
}

pub trait Mbc<CB>: GetRom + ReadRom + WriteRom + ReadRam + WriteRam + Shutdown<CB>
where
    CB: Battery,
{
    fn cart_type(&self) -> &'static str;
}

/// Stores some header information of the ROM, as well as the MBC.
pub struct Cartridge<'a, CB = ()>
where
    CB: Battery + 'a,
{
    pub mbc: Box<dyn Mbc<CB> + 'a>,
}

impl<'a> Cartridge<'a> {
    pub fn builder() -> CartridgeBuilder<'a, (), (), false> {
        CartridgeBuilder::<'a, (), (), false>::new()
    }
}

impl<'a, CB> Cartridge<'a, CB>
where
    CB: Battery + 'a,
{
    const ROM_BANK_SIZE: usize = 0x4000;
    const RAM_BANK_SIZE: usize = 0x2000;

    /// Initializes a new cartridge by reading information from the header of the ROM.
    fn new<CC>(rom: &'a [u8], battery: CB, clock: CC) -> Result<Self, CartridgeError<CB>>
    where
        CC: Clock + 'a,
    {
        Self::check_rom_size(rom)?;
        let ram = Self::create_ram(rom)?;
        let mbc = Self::get_mbc(rom, ram, battery, clock)?;

        Ok(Self { mbc })
    }

    pub fn shutdown(&mut self) -> Result<(), ShutdownError<CB>> {
        self.mbc.shutdown()
    }

    /// Reads the number of ROM banks the ROM uses.
    /// Returns an error if it is not a valid value or the ROM size is wrong.
    fn check_rom_size(rom: &[u8]) -> Result<(), CartridgeError<CB>> {
        let rom_banks = match rom[0x0148] {
            0x00 => 2,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x07 => 256,
            0x08 => 512,
            _ => return Err(CartridgeError::InvalidRomSize),
        };
        if rom.len() != rom_banks * Self::ROM_BANK_SIZE {
            return Err(CartridgeError::WrongRomSize);
        }
        Ok(())
    }

    /// Reads the number of RAM banks the ROM uses.
    /// Returns an error if it is not a valid value.
    fn create_ram(rom: &[u8]) -> Result<Vec<u8>, CartridgeError<CB>> {
        let rom_banks = match rom[0x0149] {
            0x00 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => return Err(CartridgeError::InvalidRamSize),
        };
        Ok(vec![0; rom_banks * Self::RAM_BANK_SIZE])
    }

    /// Retrieves the memory bank controller the ROM uses.
    /// Returns an error if the MBC is invalid or unsupported.
    fn get_mbc<CC>(
        rom: &'a [u8],
        ram: Ram,
        battery: CB,
        clock: CC,
    ) -> Result<Box<dyn Mbc<CB> + 'a>, CartridgeError<CB>>
    where
        CC: Clock + 'a,
    {
        println!("rom[0x0147]: {:#04x}", rom[0x0147]);
        match rom[0x0147] {
            0x00 => Ok(Box::new(RomOnly::new(rom))),
            0x01 => Ok(Box::new(Mbc1::rom_only(rom))),
            0x02 => Ok(Box::new(Mbc1::with_ram(rom, ram))),
            0x03 => Ok(Box::new(Mbc1::with_ram_battery(rom, ram, battery)?)),
            0x08 => Err(CartridgeError::UnsupportedMbc("ROM + RAM")),
            0x09 => Err(CartridgeError::UnsupportedMbc("ROM + RAM + BATTERY")),
            0x05 | 0x06 => Err(CartridgeError::UnsupportedMbc("MBC2")),
            0x0b..=0x0d => Err(CartridgeError::UnsupportedMbc("MMM01")),
            0x0f => Ok(Box::new(Mbc3::with_battery_clock(rom, battery, clock)?)),
            0x10 => Ok(Box::new(Mbc3::with_ram_battery_clock(
                rom, ram, battery, clock,
            )?)),
            0x11 => Ok(Box::new(Mbc3::rom_only(rom))),
            0x12 => Ok(Box::new(Mbc3::with_ram(rom, ram))),
            0x13 => Ok(Box::new(Mbc3::with_ram_battery(rom, ram, battery)?)),
            0x19..=0x1e => Err(CartridgeError::UnsupportedMbc("MBC5")),
            0x20 => Err(CartridgeError::UnsupportedMbc("MBC6")),
            0x22 => Err(CartridgeError::UnsupportedMbc("MBC7")),
            0xfc => Err(CartridgeError::UnsupportedMbc("POCKET CAMERA")),
            0xfd => Err(CartridgeError::UnsupportedMbc("BANDAI TAMA5")),
            0xfe => Err(CartridgeError::UnsupportedMbc("HuC3")),
            0xff => Err(CartridgeError::UnsupportedMbc("HuC1")),
            _ => Err(CartridgeError::InvalidMbc),
        }
    }

    /// Reads the title of the cartridge. Returns an error if it encounters invalid UTF-8.
    pub fn title(&self) -> Result<&str, CartridgeError<CB>> {
        str::from_utf8(&self.mbc.rom()[0x0134..0x0143]).map_err(|_| CartridgeError::InvalidTitle)
    }

    /// Reads the version of the cartridge.
    pub fn version(&self) -> usize {
        self.mbc.rom()[0x014c] as usize
    }

    /// Reads the number of ROM banks of the cartridge. Returns an error if it's invalid.
    pub fn rom_banks(&self) -> Result<usize, CartridgeError<CB>> {
        match self.mbc.rom()[0x0148] {
            0x00 => Ok(2),
            0x01 => Ok(4),
            0x02 => Ok(8),
            0x03 => Ok(16),
            0x04 => Ok(32),
            0x05 => Ok(64),
            0x06 => Ok(128),
            0x07 => Ok(256),
            0x08 => Ok(512),
            _ => Err(CartridgeError::InvalidRomSize),
        }
    }

    /// Reads the size of the ROM.
    pub fn rom_size(&self) -> Result<usize, CartridgeError<CB>> {
        Ok(self.rom_banks()? * Self::ROM_BANK_SIZE)
    }

    /// Reads the number of RAM banks of the cartridge.
    pub fn ram_banks(&self) -> Result<usize, CartridgeError<CB>> {
        match self.mbc.rom()[0x0149] {
            0x00 => Ok(0),
            0x02 => Ok(1),
            0x03 => Ok(4),
            0x04 => Ok(16),
            0x05 => Ok(8),
            _ => Err(CartridgeError::InvalidRamSize),
        }
    }

    /// Reads the number of RAM banks of the cartridge.
    pub fn ram_size(&self) -> Result<usize, CartridgeError<CB>> {
        Ok(self.ram_banks()? * Self::RAM_BANK_SIZE)
    }

    /// Get the cartridge type of the ROM.
    pub fn cart_type(&self) -> &'static str {
        self.mbc.cart_type()
    }

    /// Reads the name of the licensee from the ROM.
    /// Checks for both the old and the new format.
    /// Returns an error if the licensee is invalid.
    pub fn licensee(&self) -> Result<&'static str, CartridgeError<CB>> {
        match self.mbc.rom()[0x014b] {
            0x00 => Ok("None"),
            0x01 => Ok("Nintendo"),
            0x08 => Ok("Capcom"),
            0x09 => Ok("Hot-B"),
            0x0a => Ok("Jaleco"),
            0x0b => Ok("Coconuts Japan"),
            0x0c => Ok("Elite Systems"),
            0x13 => Ok("EA (Electronic Arts)"),
            0x18 => Ok("Hudsonsoft"),
            0x19 => Ok("ITC Entertainment"),
            0x1a => Ok("Yanoman"),
            0x1d => Ok("Japan Clary"),
            0x1f => Ok("Virgin Interactive"),
            0x24 => Ok("PCM Complete"),
            0x25 => Ok("San-X"),
            0x28 => Ok("Kotobuki Systems"),
            0x29 => Ok("Seta"),
            0x30 => Ok("Infogrames"),
            0x31 => Ok("Nintendo"),
            0x32 => Ok("Bandai"),
            0x33 => self.licensee_new(),
            0x34 => Ok("Konami"),
            0x35 => Ok("HectorSoft"),
            0x38 => Ok("Capcom"),
            0x39 => Ok("Banpresto"),
            0x3c => Ok(".Entertainment i"),
            0x3e => Ok("Gremlin"),
            0x41 => Ok("Ubisoft"),
            0x42 => Ok("Atlus"),
            0x44 => Ok("Malibu"),
            0x46 => Ok("Angel"),
            0x47 => Ok("Spectrum Holoby"),
            0x49 => Ok("Irem"),
            0x4a => Ok("Virgin Interactive"),
            0x4d => Ok("Malibu"),
            0x4f => Ok("U.S. Gold"),
            0x50 => Ok("Absolute"),
            0x51 => Ok("Acclaim"),
            0x52 => Ok("Activision"),
            0x53 => Ok("American Sammy"),
            0x54 => Ok("GameTek"),
            0x55 => Ok("Park Place"),
            0x56 => Ok("LJN"),
            0x57 => Ok("Matchbox"),
            0x59 => Ok("Milton Bradley"),
            0x5a => Ok("Mindscape"),
            0x5b => Ok("Romstar"),
            0x5c => Ok("Naxat Soft"),
            0x5d => Ok("Tradewest"),
            0x60 => Ok("Titus"),
            0x61 => Ok("Virgin Interactive"),
            0x67 => Ok("Ocean Interactive"),
            0x69 => Ok("EA (Electronic Arts)"),
            0x6e => Ok("Elite Systems"),
            0x6f => Ok("Electro Brain"),
            0x70 => Ok("Infogrames"),
            0x71 => Ok("Interplay"),
            0x72 => Ok("Broderbund"),
            0x73 => Ok("Sculptered Soft"),
            0x75 => Ok("The Sales Curve"),
            0x78 => Ok("t.hq"),
            0x79 => Ok("Accolade"),
            0x7a => Ok("Triffix Entertainment"),
            0x7c => Ok("Microprose"),
            0x7f => Ok("Kemco"),
            0x80 => Ok("Misawa Entertainment"),
            0x83 => Ok("Lozc"),
            0x86 => Ok("Tokuma Shoten Intermedia"),
            0x8b => Ok("Bullet-Proof Software"),
            0x8c => Ok("Vic Tokai"),
            0x8e => Ok("Ape"),
            0x8f => Ok("I’Max"),
            0x91 => Ok("Chunsoft Co."),
            0x92 => Ok("Video System"),
            0x93 => Ok("Tsubaraya Productions Co."),
            0x95 => Ok("Varie Corporation"),
            0x96 => Ok("Yonezawa/S’Pal"),
            0x97 => Ok("Kaneko"),
            0x99 => Ok("Arc"),
            0x9a => Ok("Nihon Bussan"),
            0x9b => Ok("Tecmo"),
            0x9c => Ok("Imagineer"),
            0x9d => Ok("Banpresto"),
            0x9f => Ok("Nova"),
            0xa1 => Ok("Hori Electric"),
            0xa2 => Ok("Bandai"),
            0xa4 => Ok("Konami"),
            0xa6 => Ok("Kawada"),
            0xa7 => Ok("Takara"),
            0xa9 => Ok("Technos Japan"),
            0xaa => Ok("Broderbund"),
            0xac => Ok("Toei Animation"),
            0xad => Ok("Toho"),
            0xaf => Ok("Namco"),
            0xb0 => Ok("acclaim"),
            0xb1 => Ok("ASCII or Nexsoft"),
            0xb2 => Ok("Bandai"),
            0xb4 => Ok("Square Enix"),
            0xb6 => Ok("HAL Laboratory"),
            0xb7 => Ok("SNK"),
            0xb9 => Ok("Pony Canyon"),
            0xba => Ok("Culture Brain"),
            0xbb => Ok("Sunsoft"),
            0xbd => Ok("Sony Imagesoft"),
            0xbf => Ok("Sammy"),
            0xc0 => Ok("Taito"),
            0xc2 => Ok("Kemco"),
            0xc3 => Ok("Squaresoft"),
            0xc4 => Ok("Tokuma Shoten Intermedia"),
            0xc5 => Ok("Data East"),
            0xc6 => Ok("Tonkinhouse"),
            0xc8 => Ok("Koei"),
            0xc9 => Ok("UFL"),
            0xca => Ok("Ultra"),
            0xcb => Ok("Vap"),
            0xcc => Ok("Use Corporation"),
            0xcd => Ok("Meldac"),
            0xce => Ok(".Pony Canyon or"),
            0xcf => Ok("Angel"),
            0xd0 => Ok("Taito"),
            0xd1 => Ok("Sofel"),
            0xd2 => Ok("Quest"),
            0xd3 => Ok("Sigma Enterprises"),
            0xd4 => Ok("ASK Kodansha Co."),
            0xd6 => Ok("Naxat Soft"),
            0xd7 => Ok("Copya System"),
            0xd9 => Ok("Banpresto"),
            0xda => Ok("Tomy"),
            0xdb => Ok("LJN"),
            0xdd => Ok("NCS"),
            0xde => Ok("Human"),
            0xdf => Ok("Altron"),
            0xe0 => Ok("Jaleco"),
            0xe1 => Ok("Towa Chiki"),
            0xe2 => Ok("Yutaka"),
            0xe3 => Ok("Varie"),
            0xe5 => Ok("Epcoh"),
            0xe7 => Ok("Athena"),
            0xe8 => Ok("Asmik ACE Entertainment"),
            0xe9 => Ok("Natsume"),
            0xea => Ok("King Records"),
            0xeb => Ok("Atlus"),
            0xec => Ok("Epic/Sony Records"),
            0xee => Ok("IGS"),
            0xf0 => Ok("A Wave"),
            0xf3 => Ok("Extreme Entertainment"),
            0xff => Ok("LJN"),
            _ => Err(CartridgeError::InvalidLicensee),
        }
    }

    /// Reads the name of the licensee from the ROM in the new format.
    fn licensee_new(&self) -> Result<&'static str, CartridgeError<CB>> {
        match (
            self.mbc.rom()[0x0144] as char,
            self.mbc.rom()[0x0145] as char,
        ) {
            ('0', '0') => Ok("None"),
            ('0', '1') => Ok("Nintendo R&D1"),
            ('0', '8') => Ok("Capcom"),
            ('1', '3') => Ok("Electronic Arts"),
            ('1', '8') => Ok("Hudson Soft"),
            ('1', '9') => Ok("b-ai"),
            ('2', '0') => Ok("kss"),
            ('2', '2') => Ok("pow"),
            ('2', '4') => Ok("PCM Complete"),
            ('2', '5') => Ok("san-x"),
            ('2', '8') => Ok("Kemco Japan"),
            ('2', '9') => Ok("seta"),
            ('3', '0') => Ok("Viacom"),
            ('3', '1') => Ok("Nintendo"),
            ('3', '2') => Ok("Bandai"),
            ('3', '3') => Ok("Ocean/Acclaim"),
            ('3', '4') => Ok("Konami"),
            ('3', '5') => Ok("Hector"),
            ('3', '7') => Ok("Taito"),
            ('3', '8') => Ok("Hudson"),
            ('3', '9') => Ok("Banpresto"),
            ('4', '1') => Ok("Ubi Soft"),
            ('4', '2') => Ok("Atlus"),
            ('4', '4') => Ok("Malibu"),
            ('4', '6') => Ok("angel"),
            ('4', '7') => Ok("Bullet-Proof"),
            ('4', '9') => Ok("irem"),
            ('5', '0') => Ok("Absolute"),
            ('5', '1') => Ok("Acclaim"),
            ('5', '2') => Ok("Activision"),
            ('5', '3') => Ok("American sammy"),
            ('5', '4') => Ok("Konami"),
            ('5', '5') => Ok("Hi tech entertainment"),
            ('5', '6') => Ok("LJN"),
            ('5', '7') => Ok("Matchbox"),
            ('5', '8') => Ok("Mattel"),
            ('5', '9') => Ok("Milton Bradley"),
            ('6', '0') => Ok("Titus"),
            ('6', '1') => Ok("Virgin"),
            ('6', '4') => Ok("LucasArts"),
            ('6', '7') => Ok("Ocean"),
            ('6', '9') => Ok("Electronic Arts"),
            ('7', '0') => Ok("Infogrames"),
            ('7', '1') => Ok("Interplay"),
            ('7', '2') => Ok("Broderbund"),
            ('7', '3') => Ok("sculptured"),
            ('7', '5') => Ok("sci"),
            ('7', '8') => Ok("THQ"),
            ('7', '9') => Ok("Accolade"),
            ('8', '0') => Ok("misawa"),
            ('8', '3') => Ok("lozc"),
            ('8', '6') => Ok("Tokuma Shoten Intermedia"),
            ('8', '7') => Ok("Tsukuda Original"),
            ('9', '1') => Ok("Chunsoft"),
            ('9', '2') => Ok("Video system"),
            ('9', '3') => Ok("Ocean/Acclaim"),
            ('9', '5') => Ok("Varie"),
            ('9', '6') => Ok("Yonezawa/s’pal"),
            ('9', '7') => Ok("Kaneko"),
            ('9', '9') => Ok("Pack in soft"),
            ('A', '4') => Ok("Konami (Yu-Gi-Oh!)"),
            _ => Err(CartridgeError::InvalidLicensee),
        }
    }
}

pub struct CartridgeBuilder<'a, CB, CC, const ROM: bool>
where
    CB: Battery + 'a,
    CC: Clock + 'a,
{
    rom: &'a [u8],
    battery: CB,
    clock: CC,
}

impl<'a, CB, CC, const ROM: bool> CartridgeBuilder<'a, CB, CC, ROM>
where
    CB: Battery + 'a,
    CC: Clock + 'a,
{
    pub fn new() -> CartridgeBuilder<'a, (), (), false> {
        CartridgeBuilder {
            rom: &[],
            battery: (),
            clock: (),
        }
    }
}

impl<'a, CB, CC> CartridgeBuilder<'a, CB, CC, false>
where
    CB: Battery + 'a,
    CC: Clock + 'a,
{
    pub fn rom(self, rom: &'a [u8]) -> CartridgeBuilder<'a, CB, CC, true> {
        CartridgeBuilder {
            rom,
            battery: self.battery,
            clock: self.clock,
        }
    }
}

impl<'a, CC, const ROM: bool> CartridgeBuilder<'a, (), CC, ROM>
where
    CC: Clock + 'a,
{
    pub fn battery<CB>(self, battery: CB) -> CartridgeBuilder<'a, CB, CC, ROM>
    where
        CB: Battery + 'a,
    {
        CartridgeBuilder {
            rom: self.rom,
            battery,
            clock: self.clock,
        }
    }
}

impl<'a, CB, const ROM: bool> CartridgeBuilder<'a, CB, (), ROM>
where
    CB: Battery + 'a,
{
    pub fn clock<CC>(self, clock: CC) -> CartridgeBuilder<'a, CB, CC, ROM>
    where
        CC: Clock + 'a,
    {
        CartridgeBuilder {
            rom: self.rom,
            battery: self.battery,
            clock,
        }
    }
}

impl<'a, CB, CC> CartridgeBuilder<'a, CB, CC, true>
where
    CB: Battery + 'a,
    CC: Clock + 'a,
{
    pub fn build(self) -> Result<Cartridge<'a, CB>, CartridgeError<CB>>
    where
        CB: Battery,
    {
        Cartridge::new(self.rom, self.battery, self.clock)
    }
}
