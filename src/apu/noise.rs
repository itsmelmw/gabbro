use crate::apu::SweepDir;

pub enum LfsrWidth {
    B15 = 0,
    B7 = 1,
}

pub struct Noise {
    pub nrx0: u8,
    pub nrx1: u8,
    pub nrx2: u8,
    pub nrx3: u8,
    pub nrx4: u8,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            nrx0: 0,
            nrx1: 0,
            nrx2: 0,
            nrx3: 0,
            nrx4: 0,
        }
    }

    pub fn length_timer(&self) -> u8 {
        self.nrx1 & 0x3f
    }

    pub fn envelope_pace(&self) -> u8 {
        self.nrx2 & 0x07
    }

    pub fn envelope_dir(&self) -> SweepDir {
        match (self.nrx2 >> 3) & 1 {
            0 => SweepDir::Increase,
            1 => SweepDir::Decrease,
            _ => unreachable!(),
        }
    }

    pub fn volume(&self) -> u8 {
        (self.nrx2 >> 4) & 0x0f
    }

    pub fn clock_divider(&self) -> u8 {
        self.nrx3 & 0x07
    }

    pub fn lfsr_width(&self) -> LfsrWidth {
        match (self.nrx3 >> 3) & 1 {
            0 => LfsrWidth::B15,
            1 => LfsrWidth::B7,
            _ => unreachable!(),
        }
    }

    pub fn clock_shift(&self) -> u8 {
        (self.nrx3 >> 4) & 0x0f
    }

    pub fn length_enabled(&self) -> bool {
        (self.nrx4 >> 6) & 1 != 0
    }

    pub fn triggered(&self) -> bool {
        (self.nrx4 >> 7) & 1 != 0
    }
}
