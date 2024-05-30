pub mod control;
pub mod noise;
pub mod pulse;
pub mod wave;

use crate::apu::{control::MasterControl, noise::Noise, pulse::Pulse, wave::Wave};

const MAX_FREQ: usize = 131072;

pub struct Apu {
    pub master: MasterControl,
    pub ch1: Pulse<true>,
    pub ch2: Pulse<false>,
    pub ch3: Wave,
    pub ch4: Noise,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            master: MasterControl::new(),
            ch1: Pulse::new(),
            ch2: Pulse::new(),
            ch3: Wave::new(),
            ch4: Noise::new(),
        }
    }

    pub fn step(&mut self) {}
}

pub enum SweepDir {
    Increase,
    Decrease,
}

impl SweepDir {
    pub fn diff(&self) -> i8 {
        match self {
            Self::Increase => 1,
            Self::Decrease => -1,
        }
    }
}

pub struct VolumeEnvelope {
    volume: u8,
    direction: SweepDir,
    pace: u8,
}

impl VolumeEnvelope {
    pub fn new() -> Self {
        Self {
            volume: 0,
            direction: SweepDir::Decrease,
            pace: 0,
        }
    }

    pub fn start(&mut self, volume: u8, direction: SweepDir, pace: u8) {
        self.volume = volume;
        self.direction = direction;
        self.pace = pace;
    }

    pub fn step(&mut self) -> u8 {
        0
    }
}

pub struct FrequencySweep {
    frequency: u16,
    direction: SweepDir,
    pace: u8,
    step: u8,
}

impl FrequencySweep {
    pub fn new() -> Self {
        Self {
            frequency: 0,
            direction: SweepDir::Increase,
            pace: 0,
            step: 0,
        }
    }

    pub fn start(&mut self, frequency: u16, direction: SweepDir, pace: u8, step: u8) {
        self.frequency = frequency;
        self.direction = direction;
        self.pace = pace;
        self.step = step;
    }
}

pub struct LengthTimer {
    enabled: bool,
    length: u8,
}

impl LengthTimer {
    pub fn new() -> Self {
        Self {
            enabled: false,
            length: 0,
        }
    }

    pub fn start(&mut self, enabled: bool, length: u8) {
        self.enabled = enabled;
        self.length = length;
    }
}
