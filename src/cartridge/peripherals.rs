use crate::peripherals::NoError;
use std::{
    fmt::Debug,
    fs::OpenOptions,
    io::{Error as IoError, Read, Write},
    path::PathBuf,
    time::Instant,
};

// Need these structs to be able to check whether an MBC has a feature or not,
// since `NoFeat` also needs to implement the features with defaults.
pub struct NoFeat;
pub struct Feat<F>(pub F);

pub type NoRam = ();
pub type Ram = Vec<u8>;

pub trait Battery {
    type Error: Debug;
    fn store_data(&mut self, _data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn load_data(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(None)
    }
}

impl Battery for () {
    type Error = NoError;
}
impl Battery for NoFeat {
    type Error = NoError;
}

impl Battery for PathBuf {
    type Error = IoError;
    fn store_data(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self)?;
        file.write_all(data)?;
        Ok(())
    }

    fn load_data(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
        match OpenOptions::new().read(true).open(self) {
            Ok(mut file) => {
                let mut data = Vec::with_capacity(file.metadata()?.len() as usize);
                file.read_to_end(&mut data)?;
                Ok(Some(data))
            }
            Err(_) => Ok(None),
        }
    }
}

impl<F> Battery for Feat<F>
where
    F: Battery,
{
    type Error = F::Error;
    fn store_data(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.0.store_data(data)
    }

    fn load_data(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
        self.0.load_data()
    }
}

pub trait Clock {
    /// Returns the number of seconds passed since the last time this function was called,
    /// or since the emulator started if no calls were made yet.
    fn seconds_passed(&mut self) -> usize {
        0
    }
}

impl Clock for () {}
impl Clock for NoFeat {}

impl Clock for Instant {
    fn seconds_passed(&mut self) -> usize {
        let passed = self.elapsed().as_secs() as usize;
        *self = Instant::now();
        passed
    }
}

impl<F> Clock for Feat<F>
where
    F: Clock,
{
    fn seconds_passed(&mut self) -> usize {
        self.0.seconds_passed()
    }
}
