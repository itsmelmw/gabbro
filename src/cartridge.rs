mod header;
pub mod mbc1;
pub mod nombc;
pub use mbc1::Mbc1;
pub use nombc::NoMbc;
use std::fmt;

pub trait Mbc: fmt::Display {
    /// Reads the value at `addr` from the selected ROM bank.
    fn read_rom(&self, addr: u16) -> u8;
    /// Writes `val` to `addr` in the selected ROM bank.
    /// The MBC's usually handle this by writing `val` to one of their internal registers.
    fn write_rom(&mut self, addr: u16, val: u8);

    /// Reads the value at `addr` from the selected RAM bank.
    fn read_ram(&self, addr: u16) -> u8;
    /// Writes `val` to `addr` in the selected RAM bank.
    fn write_ram(&mut self, addr: u16, val: u8);
}

/// Stores some header information of the ROM, as well as the MBC.
pub struct Cartridge {
    title: String,
    version: u8,
    licensee: &'static str,
    rom_banks: usize,
    ram_banks: usize,
    pub mbc: Box<dyn Mbc>,
}

impl Cartridge {
    /// Initializes a new cartridge by reading information from the header of the ROM.
    pub fn new(rom: Vec<u8>) -> Result<Self, &'static str> {
        let title = header::get_title(&rom)?;
        let version = header::get_version(&rom);
        let licensee = header::get_licensee(&rom)?;
        let rom_banks = header::get_rom_banks(&rom)?;
        let ram_banks = header::get_ram_banks(&rom)?;
        let mbc = header::get_mbc(rom, rom_banks, ram_banks)?;
        Ok(Self {
            title,
            version,
            licensee,
            rom_banks,
            ram_banks,
            mbc,
        })
    }
    /// Logs the information in the ROM header.
    pub fn log_header(&self) {
        log::info!("################################");
        log::info!("  {:-16}{:.16}", "ROM title:", self.title);
        log::info!("  {:-16}{:.16}", "ROM version:", self.version);
        log::info!("  {:-16}{:.16}", "ROM licensee:", self.licensee);
        log::info!("  {:-16}{:.16}", "MBC type:", self.mbc);
        log::info!("  {:-16}{:.16}", "ROM banks:", self.rom_banks);
        log::info!("  {:-16}{:.16}", "RAM banks:", self.ram_banks);
        log::info!("################################");
    }
}
