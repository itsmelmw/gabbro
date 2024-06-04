use crate::apu::SweepDir;

use super::{LengthTimer, VolumeEnvelope};

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

    pub fn length_timer(&self) -> u8 {
        self.nrx1 & 0x3f
    }

    pub fn envelope_pace(&self) -> u8 {
        self.nrx2 & 0x07
    }

    pub fn envelope_dir(&self) -> SweepDir {
        match (self.nrx2 >> 3) & 1 {
            0 => SweepDir::Decrease,
            1 => SweepDir::Increase,
            _ => unreachable!(),
        }
    }

    pub fn volume(&self) -> u8 {
        (self.nrx2 >> 4) & 0x0f
    }

    pub fn period(&self) -> u16 {
        let shift = self.clock_shift() as u16;
        let divider = Self::DIVIDER_TABLE[self.clock_divider() as usize];
        divider << shift
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

    pub fn lfsr_step(&mut self) {
        let bit = !((self.lfsr & 0x01) ^ ((self.lfsr >> 1) & 0x01));
        self.lfsr |= bit << 15;
        self.lfsr >>= 1;
        if let LfsrWidth::B7 = self.lfsr_width() {
            self.lfsr = (self.lfsr & 0xffbf) | bit << 6;
        }
    }

    pub fn start(&mut self) {
        self.length_timer
            .start(self.length_timer(), self.length_enabled());
        self.volume_envelope
            .start(self.volume(), self.envelope_dir(), self.envelope_pace());
        self.ticks = 0;
        self.lfsr = 0;

        println!("Noise start: period: {}, width: {:?}, shift: {}, divider: {}, length: {}, volume: {}, envelope dir: {:?}, envelope pace: {}",
            self.period(),
            match self.lfsr_width() {
                LfsrWidth::B15 => "15",
                LfsrWidth::B7 => "7",
            },
            self.clock_shift(),
            self.clock_divider(),
            self.length_timer(),
            self.volume(),
            match self.envelope_dir() {
                SweepDir::Increase => "Increase",
                SweepDir::Decrease => "Decrease",
            },
            self.envelope_pace(),
        );
    }

    pub fn sample(&mut self) -> f32 {
        if self.length_timer.shut_down() {
            return 0.;
        }
        let volume = self.volume_envelope.current_volume();

        self.ticks += 1;
        if self.ticks >= self.period() as usize {
            self.ticks = 0;
            self.lfsr_step();
        }

        if self.lfsr & 1 != 0 {
            volume
        } else {
            0.
        }
    }
}
