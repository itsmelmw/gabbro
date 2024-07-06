use crate::apu::LengthTimer;

pub struct Wave {
    length_timer: LengthTimer,
    waveform_idx: usize,
    ticks: usize,
    pub nrx0: u8,
    pub nrx1: u8,
    pub nrx2: u8,
    pub nrx3: u8,
    pub nrx4: u8,
    pub waveform: [u8; 0x10],
}

impl Wave {
    const WAVEFORM_SIZE: usize = 32;
    pub fn new() -> Self {
        Self {
            length_timer: LengthTimer::default(),
            waveform_idx: 0,
            ticks: 0,
            nrx0: 0,
            nrx1: 0,
            nrx2: 0,
            nrx3: 0,
            nrx4: 0,
            waveform: [0; 0x10],
        }
    }

    fn dac_enabled(&self) -> bool {
        (self.nrx0 >> 7) & 1 != 0
    }

    fn length_timer(&self) -> u8 {
        0xff - self.nrx1
    }

    fn volume(&self) -> f32 {
        match (self.nrx2 >> 5) & 0x03 {
            0 => 0.,
            1 => 1.,
            2 => 0.5,
            3 => 0.25,
            _ => unreachable!(),
        }
    }

    fn period(&self) -> u16 {
        let low = self.nrx3 as u16;
        let high = (self.nrx4 as u16 & 0x07) << 8;
        (0x800 - (high | low)) / 2
    }

    fn length_enabled(&self) -> bool {
        (self.nrx4 >> 6) & 1 != 0
    }

    fn waveform_sample(&self, idx: usize) -> u8 {
        let byte = self.waveform[idx / 2];
        if idx % 2 == 0 {
            (byte >> 4) & 0x0f
        } else {
            byte & 0x0f
        }
    }

    pub fn start(&mut self) {
        self.ticks = 0;
        self.waveform_idx = 0;
        self.length_timer
            .start(self.length_timer(), self.length_enabled());
    }

    pub fn sample(&mut self) -> Option<f32> {
        if !self.dac_enabled() || !self.length_timer.current_state() {
            return None;
        }
        let volume = self.volume();

        self.ticks += 1;
        if self.ticks >= self.period() as usize {
            self.ticks = 0;
            self.waveform_idx = (self.waveform_idx + 1) % Self::WAVEFORM_SIZE;
        }
        Some(volume * (self.waveform_sample(self.waveform_idx) as f32 / 15.))
    }
}
