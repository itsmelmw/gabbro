pub mod control;
pub mod noise;
pub mod pulse;
pub mod wave;

use crate::{
    apu::{control::MasterControl, noise::Noise, pulse::Pulse, wave::Wave},
    peripherals::Speaker,
};

const TICK_RATE: usize = 1048576;
const VOLUME_ENVELOPE_TICK_RATE: usize = TICK_RATE / 64;
const LENGTH_TIMER_TICK_RATE: usize = TICK_RATE / 256;

pub struct Apu<S>
where
    S: Speaker,
{
    pub master: MasterControl,
    pub ch1: Pulse<true>,
    pub ch2: Pulse<false>,
    pub ch3: Wave,
    pub ch4: Noise,
    speaker: S,
    sample_counter: usize,
}

impl<S> Apu<S>
where
    S: Speaker,
{
    pub fn new(speaker: S) -> Self {
        Self {
            master: MasterControl::new(),
            ch1: Pulse::new(),
            ch2: Pulse::new(),
            ch3: Wave::new(),
            ch4: Noise::new(),
            speaker,
            sample_counter: 0,
        }
    }

    pub fn step(&mut self) {
        let mut samples = [0.; 4];
        samples[0] = self.ch1.sample();
        samples[1] = self.ch2.sample();

        if self.speaker.sampling_rate() > 0 {
            self.sample_counter += 1;
            if self.sample_counter >= TICK_RATE / self.speaker.sampling_rate() {
                let (mut left_sample, mut right_sample) = (0., 0.);
                if self.master.apu_enabled() {
                    for (ch_idx, sample) in samples.iter().enumerate() {
                        if self.master.channel_left(ch_idx) {
                            left_sample += sample;
                        }
                        if self.master.channel_right(ch_idx) {
                            right_sample += sample;
                        }
                    }
                }
                left_sample *= self.master.left_volume() as f32 / 8.;
                right_sample *= self.master.right_volume() as f32 / 8.;
                self.speaker.push_sample(left_sample, right_sample);
                self.sample_counter = 0;
            }
        }
    }
}

pub enum SweepDir {
    Increase,
    Decrease,
}

pub struct LengthTimer {
    enabled: bool,
    length: u8,
    ticks: usize,
}

impl LengthTimer {
    pub fn new() -> Self {
        Self {
            enabled: false,
            length: 0,
            ticks: 0,
        }
    }

    pub fn start(&mut self, length: u8, enabled: bool) {
        self.length = length;
        self.enabled = enabled;
        self.ticks = 0;
    }

    pub fn shut_down(&mut self) -> bool {
        if !self.enabled {
            return false;
        }
        if self.length == 0x80 {
            return true;
        }
        self.ticks += 1;
        if self.ticks >= LENGTH_TIMER_TICK_RATE {
            self.ticks = 0;
            self.length += 1;
        }
        false
    }
}

pub struct VolumeEnvelope {
    volume: u8,
    direction: SweepDir,
    pace: u8,
    ticks: usize,
}

impl VolumeEnvelope {
    pub fn new() -> Self {
        Self {
            volume: 0,
            direction: SweepDir::Decrease,
            pace: 0,
            ticks: 0,
        }
    }

    pub fn start(&mut self, volume: u8, direction: SweepDir, pace: u8) {
        self.volume = volume;
        self.direction = direction;
        self.pace = pace;
        self.ticks = 0;
    }

    pub fn current_volume(&mut self) -> f32 {
        if self.volume == 0 {
            return 0.;
        }
        self.ticks += 1;
        if self.ticks >= VOLUME_ENVELOPE_TICK_RATE * self.pace as usize {
            self.ticks = 0;
            match self.direction {
                SweepDir::Increase => self.volume += 1,
                SweepDir::Decrease => self.volume -= 1,
            }
            if self.volume > 15 {
                self.volume = 0;
            }
        }
        self.volume as f32 / 15.
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

    // pub fn current_frequency(&mut self) -> u16 {}
}
