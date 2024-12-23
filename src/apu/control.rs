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

    pub fn enable_channel(&mut self, ch_idx: usize) {
        self.control |= 1 << ch_idx as u8;
    }

    pub fn disable_channel(&mut self, ch_idx: usize) {
        self.control &= !(1 << ch_idx as u8);
    }

    pub fn channel_right(&self, ch_idx: usize) -> bool {
        (self.panning >> ch_idx as u8) & 1 != 0
    }

    pub fn channel_left(&self, ch_idx: usize) -> bool {
        (self.panning >> (ch_idx as u8 + 4)) & 1 != 0
    }

    pub fn right_volume(&self) -> u8 {
        (self.volume & 0x07) + 1
    }

    pub fn left_volume(&self) -> u8 {
        ((self.volume >> 4) & 0x07) + 1
    }
}
