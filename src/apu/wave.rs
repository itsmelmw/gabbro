pub struct Wave {
    pub nrx0: u8,
    pub nrx1: u8,
    pub nrx2: u8,
    pub nrx3: u8,
    pub nrx4: u8,
    pub wave: [u8; 0x10],
}

impl Wave {
    pub fn new() -> Self {
        Self {
            nrx0: 0,
            nrx1: 0,
            nrx2: 0,
            nrx3: 0,
            nrx4: 0,
            wave: [0; 0x10],
        }
    }

    pub fn dac_enabled(&self) -> bool {
        (self.nrx0 >> 7) & 1 != 0
    }

    pub fn length_timer(&self) -> u8 {
        self.nrx1
    }

    pub fn volume(&self) -> u8 {
        (self.nrx2 >> 5) & 0x03
    }

    pub fn period(&self) -> u16 {
        let low = self.nrx3 as u16;
        let high = (self.nrx4 as u16 & 0x07) << 8;
        high | low
    }

    pub fn length_enabled(&self) -> bool {
        (self.nrx4 >> 6) & 1 != 0
    }

    pub fn triggered(&self) -> bool {
        (self.nrx4 >> 7) & 1 != 0
    }

    pub fn sample(&self, idx: usize) -> u8 {
        let byte = self.wave[idx / 2];
        if idx % 2 == 0 {
            (byte >> 4) & 0x0f
        } else {
            byte & 0x0f
        }
    }
}
