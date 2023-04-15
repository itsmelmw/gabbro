/// Temporary implementation of the audio processing unit to have something to read/write from/to.
pub struct Apu {
    pub mem: [u8; 0x30],
}

impl Apu {
    pub fn new() -> Self {
        Self { mem: [0; 0x30] }
    }
}
