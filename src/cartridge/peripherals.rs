use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

// Need these structs to be able to check whether an MBC has a feature or not,
// since `NoFeat` also needs to implement the features with defaults.
pub struct NoFeat;
pub struct Feat<F>(pub F);

pub type NoRam = ();
pub type Ram = Vec<u8>;

pub trait ExtRam {
    fn data(&self) -> Option<&[u8]>;
}

impl ExtRam for NoRam {
    fn data(&self) -> Option<&[u8]> {
        None
    }
}
impl ExtRam for Ram {
    fn data(&self) -> Option<&[u8]> {
        Some(self)
    }
}

pub trait Battery {
    fn store_data(&mut self, _data: &[u8]) {}
    fn load_data(&mut self) -> Option<Vec<u8>> {
        None
    }
}

impl Battery for () {}
impl Battery for NoFeat {}

impl Battery for PathBuf {
    fn store_data(&mut self, data: &[u8]) {
        let mut file = File::create(self).expect("Failed to write save file");
        file.write_all(data).unwrap();
    }

    fn load_data(&mut self) -> Option<Vec<u8>> {
        let mut file = File::open(self).ok()?;
        let mut data = Vec::with_capacity(file.metadata().unwrap().len() as usize);
        file.read_to_end(&mut data)
            .expect("Failed to read save file");
        Some(data)
    }
}

impl<F> Battery for Feat<F>
where
    F: Battery,
{
    fn store_data(&mut self, data: &[u8]) {
        self.0.store_data(data);
    }

    fn load_data(&mut self) -> Option<Vec<u8>> {
        self.0.load_data()
    }
}

pub trait Rtc {}

impl Rtc for () {}
impl Rtc for NoFeat {}

impl<F> Rtc for Feat<F> where F: Rtc {}
