use crate::cartridge::{Cartridge, CartridgeError};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub trait ExternalRam {
    fn read(&self, _addr: usize) -> u8 {
        0xff
    }
    fn write(&mut self, _addr: usize, _val: u8) {}
}

pub struct BatteryBackedRam<CS>
where
    CS: SaveStorage,
{
    ram: Vec<u8>,
    storage: CS,
}
pub type Ram = BatteryBackedRam<()>;

impl Ram {
    pub fn new(ram: Vec<u8>) -> Self {
        Self::with_storage(ram, ()).unwrap()
    }
}

impl<CS> BatteryBackedRam<CS>
where
    CS: SaveStorage,
{
    pub fn with_storage(mut ram: Vec<u8>, mut storage: CS) -> Result<Self, CartridgeError> {
        if let Some(saved) = storage.load() {
            if saved.len() == ram.len() {
                ram.copy_from_slice(&saved);
            } else {
                return Err(CartridgeError::WrongSaveSize);
            }
        }
        Ok(Self { ram, storage })
    }
}

impl<CS> Drop for BatteryBackedRam<CS>
where
    CS: SaveStorage,
{
    fn drop(&mut self) {
        self.storage.save(&self.ram);
    }
}

impl ExternalRam for () {}

impl<CS> ExternalRam for BatteryBackedRam<CS>
where
    CS: SaveStorage,
{
    fn read(&self, addr: usize) -> u8 {
        self.ram[addr]
    }
    fn write(&mut self, addr: usize, val: u8) {
        self.ram[addr] = val;
    }
}

pub trait SaveStorage {
    fn save(&mut self, _ram: &[u8]) {}
    fn load(&mut self) -> Option<Vec<u8>> {
        None
    }
}

impl SaveStorage for () {}

pub struct SaveFile {
    filepath: PathBuf,
}

impl SaveFile {
    pub fn new(filepath: PathBuf) -> Self {
        Self { filepath }
    }
}

impl SaveStorage for SaveFile {
    fn save(&mut self, ram: &[u8]) {
        let mut file = File::create(&self.filepath).expect("Failed to write save file");
        file.write_all(ram).unwrap();
    }

    fn load(&mut self) -> Option<Vec<u8>> {
        let mut file = File::open(&self.filepath).ok()?;
        let mut ram = Vec::with_capacity(Cartridge::RAM_BANK_SIZE);
        file.read_to_end(&mut ram)
            .expect("Failed to read save file");
        Some(ram)
        //let mut ram = Vec::with_capacity(Cartridge::RAM_BANK_SIZE);
        //self.file.read_to_end(&mut ram).ok()?;
        //Some(ram)
    }
}
