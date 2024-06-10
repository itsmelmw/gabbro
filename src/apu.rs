pub mod control;
pub mod noise;
pub mod pulse;
pub mod wave;

use crate::{
    apu::{control::MasterControl, noise::Noise, pulse::Pulse, wave::Wave},
    peripherals::Speaker,
};

const APU_TICK_RATE: usize = 1048576;

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
        }
    }

    pub fn step(&mut self) {
        let mut samples = [0.; 4];
        samples[0] = self.ch1.sample();
        samples[1] = self.ch2.sample();

        samples[3] = self.ch4.sample();

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
    }
}

#[derive(Default)]
pub enum SweepDir {
    #[default]
    Decrease,
    Increase,
}

#[derive(Default)]
pub struct LengthTimer {
    enabled: bool,
    length: u8,
    ticks: usize,
}

impl LengthTimer {
    const TICK_RATE: usize = APU_TICK_RATE / 256;

    pub fn start(&mut self, length: u8, enabled: bool) {
        self.length = length;
        self.enabled = enabled;
        self.ticks = 0;
    }

    pub fn shut_down(&mut self) -> bool {
        if !self.enabled {
            return false;
        }
        if self.length >= 0x40 {
            return true;
        }
        self.ticks += 1;
        if self.ticks >= Self::TICK_RATE {
            self.ticks = 0;
            self.length += 1;
        }
        false
    }
}

#[derive(Default)]
pub struct VolumeEnvelope {
    volume: u8,
    direction: SweepDir,
    pace: u8,
    ticks: usize,
}

impl VolumeEnvelope {
    const TICK_RATE: usize = APU_TICK_RATE / 64;

    pub fn start(&mut self, volume: u8, direction: SweepDir, pace: u8) {
        self.volume = volume;
        self.direction = direction;
        self.pace = pace;
        self.ticks = 0;
    }

    pub fn current_volume(&mut self) -> f32 {
        if self.pace > 0 {
            self.ticks += 1;
            if self.ticks >= Self::TICK_RATE * self.pace as usize {
                self.ticks = 0;
                match self.direction {
                    SweepDir::Increase => {
                        if self.volume < 15 {
                            self.volume += 1
                        }
                    }
                    SweepDir::Decrease => {
                        if self.volume > 0 {
                            self.volume -= 1
                        }
                    }
                }
            }
        }
        self.volume as f32 / 15.
    }
}

#[derive(Default)]
pub struct PeriodSweep {
    period: u16,
    direction: SweepDir,
    pace: u8,
    step: u8,
    ticks: usize,
}

impl PeriodSweep {
    const TICK_RATE: usize = APU_TICK_RATE / 128;

    pub fn start(&mut self, period: u16, direction: SweepDir, pace: u8, step: u8) {
        self.period = period;
        self.direction = direction;
        self.pace = pace;
        self.step = step;
        self.ticks = 0;
    }

    pub fn current_period(&mut self) -> Option<u16> {
        if self.period == 0 {
            return None;
        }
        self.ticks += 1;
        if self.pace > 0 && self.ticks >= Self::TICK_RATE * self.pace as usize {
            self.ticks = 0;
            let diff = self.period >> self.step;
            self.period = match self.direction {
                SweepDir::Increase => {
                    let period = self.period.saturating_add(diff);
                    if period > 0x7ff {
                        0
                    } else {
                        period
                    }
                }
                SweepDir::Decrease => self.period.wrapping_sub(diff),
            };
            Some(self.period)
        } else {
            None
        }
    }
}
