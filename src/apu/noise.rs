use crate::apu::{LengthTimer, SweepDir, VolumeEnvelope};

pub enum LfsrWidth {
    B15 = 0,
    B7 = 1,
}

pub struct Noise {
    volume_envelope: VolumeEnvelope,
    length_timer: LengthTimer,
    lfsr: u16,
    ticks: usize,
    pub nrx1: u8,
    pub nrx2: u8,
    pub nrx3: u8,
    pub nrx4: u8,
}

impl Noise {
    // Assumes 1MHz clock
    const DIVIDER_TABLE: [u16; 8] = [2, 4, 8, 12, 16, 20, 24, 28];

    pub fn new() -> Self {
        Self {
            volume_envelope: VolumeEnvelope::default(),
            length_timer: LengthTimer::default(),
            lfsr: 0,
            ticks: 0,
            nrx1: 0,
            nrx2: 0,
            nrx3: 0,
            nrx4: 0,
        }
    }

    fn length_timer(&self) -> u8 {
        0x40 - (self.nrx1 & 0x3f)
    }

    fn envelope_pace(&self) -> u8 {
        self.nrx2 & 0x07
    }

    fn envelope_dir(&self) -> SweepDir {
        match (self.nrx2 >> 3) & 1 {
            0 => SweepDir::Decrease,
            1 => SweepDir::Increase,
            _ => unreachable!(),
        }
    }

    fn volume(&self) -> u8 {
        (self.nrx2 >> 4) & 0x0f
    }

    fn period(&self) -> u16 {
        let shift = self.clock_shift() as u16;
        let divider = Self::DIVIDER_TABLE[self.clock_divider() as usize];
        divider << shift
    }

    fn clock_divider(&self) -> u8 {
        self.nrx3 & 0x07
    }

    fn lfsr_width(&self) -> LfsrWidth {
        match (self.nrx3 >> 3) & 1 {
            0 => LfsrWidth::B15,
            1 => LfsrWidth::B7,
            _ => unreachable!(),
        }
    }

    fn clock_shift(&self) -> u8 {
        (self.nrx3 >> 4) & 0x0f
    }

    fn length_enabled(&self) -> bool {
        (self.nrx4 >> 6) & 1 != 0
    }

    fn lfsr_step(&mut self) {
        let bit = !((self.lfsr & 0x01) ^ ((self.lfsr >> 1) & 0x01));
        self.lfsr |= bit << 15;
        if let LfsrWidth::B7 = self.lfsr_width() {
            self.lfsr = (self.lfsr & 0xff7f) | bit << 7;
        }
        self.lfsr >>= 1;
    }

    pub fn start(&mut self) {
        self.ticks = 0;
        self.lfsr = 0;
        self.length_timer
            .start(self.length_timer(), self.length_enabled());
        self.volume_envelope
            .start(self.volume(), self.envelope_dir(), self.envelope_pace());
    }

    pub fn sample(&mut self) -> Option<f32> {
        if self.nrx2 & 0xf8 == 0 || !self.length_timer.step() {
            return None;
        }
        let volume = self.volume_envelope.step();

        self.ticks += 1;
        if self.ticks >= self.period() as usize {
            self.ticks = 0;
            self.lfsr_step();
        }
        Some(volume * (self.lfsr & 1) as f32)
    }
}
