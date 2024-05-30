pub struct MasterControl {
    pub control: u8,
    pub panning: u8,
    pub volume: u8,
}

impl MasterControl {
    pub fn new() -> Self {
        Self {
            control: 0,
            panning: 0,
            volume: 0,
        }
    }

    pub fn apu_enabled(&self) -> bool {
        (self.control >> 7) & 1 != 0
    }

    pub fn ch1_enabled(&self) -> bool {
        self.control & 1 != 0
    }

    pub fn ch2_enabled(&self) -> bool {
        (self.control >> 1) & 1 != 0
    }

    pub fn ch3_enabled(&self) -> bool {
        (self.control >> 2) & 1 != 0
    }

    pub fn ch4_enabled(&self) -> bool {
        (self.control >> 3) & 1 != 0
    }

    pub fn ch1_right(&self) -> bool {
        self.panning & 1 != 0
    }

    pub fn ch2_right(&self) -> bool {
        (self.panning >> 1) & 1 != 0
    }

    pub fn ch3_right(&self) -> bool {
        (self.panning >> 2) & 1 != 0
    }

    pub fn ch4_right(&self) -> bool {
        (self.panning >> 3) & 1 != 0
    }

    pub fn ch1_left(&self) -> bool {
        (self.panning >> 4) & 1 != 0
    }

    pub fn ch2_left(&self) -> bool {
        (self.panning >> 5) & 1 != 0
    }

    pub fn ch3_left(&self) -> bool {
        (self.panning >> 6) & 1 != 0
    }

    pub fn ch4_left(&self) -> bool {
        (self.panning >> 7) & 1 != 0
    }

    pub fn right_volume(&self) -> u8 {
        self.volume & 0x07
    }

    pub fn left_volume(&self) -> u8 {
        (self.volume >> 4) & 0x07
    }
}
