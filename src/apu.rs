pub mod control;
pub mod noise;
pub mod pulse;
pub mod wave;

use crate::{
    apu::{control::MasterControl, noise::Noise, pulse::Pulse, wave::Wave},
    peripherals::Speaker,
};

pub const APU_SAMPLE_RATE: usize = 1048576;

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
        let (mut left_sample, mut right_sample) = (0., 0.);

        if self.master.apu_enabled() {
            let samples = [
                self.ch1.sample(),
                self.ch2.sample(),
                self.ch3.sample(),
                self.ch4.sample(),
            ];

            for (ch_idx, sample) in samples.iter().enumerate() {
                match sample {
                    Some(sample) => {
                        if self.master.channel_left(ch_idx) {
                            left_sample += sample;
                        }
                        if self.master.channel_right(ch_idx) {
                            right_sample += sample;
                        }
                    }
                    None => self.master.disable_channel(ch_idx),
                }
            }
            left_sample *= self.master.left_volume() as f32 / 8.;
            right_sample *= self.master.right_volume() as f32 / 8.;
        }

        self.speaker.push_sample(left_sample, right_sample);
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff10 => self.ch1.nrx0 | 0x80,
            0xff11 => self.ch1.nrx1 | 0x3f,
            0xff12 => self.ch1.nrx2,
            0xff13 => 0xff,
            0xff14 => self.ch1.nrx4 | 0xbf,
            0xff15 => 0xff,
            0xff16 => self.ch2.nrx1 | 0x3f,
            0xff17 => self.ch2.nrx2,
            0xff18 => 0xff,
            0xff19 => self.ch2.nrx4 | 0xbf,
            0xff1a => self.ch3.nrx0 | 0x7f,
            0xff1b => 0xff,
            0xff1c => self.ch3.nrx2 | 0x9f,
            0xff1d => 0xff,
            0xff1e => self.ch3.nrx4 | 0xbf,
            0xff1f => 0xff,
            0xff20 => 0xff,
            0xff21 => self.ch4.nrx2,
            0xff22 => self.ch4.nrx3,
            0xff23 => self.ch4.nrx4 | 0xbf,
            0xff24 => self.master.volume,
            0xff25 => self.master.panning,
            0xff26 => self.master.control | 0x70,
            0xff27..=0xff2f => 0xff,
            0xff30..=0xff3f => self.ch3.waveform[addr as usize - 0xff30],
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if !self.master.apu_enabled() && addr != 0xff26 {
            return;
        }
        match addr {
            0xff10 => self.ch1.nrx0 = value & 0x7f,
            0xff11 => self.ch1.nrx1 = value,
            0xff12 => self.ch1.nrx2 = value,
            0xff13 => self.ch1.nrx3 = value,
            0xff14 => {
                self.ch1.nrx4 = value & 0xc7;
                if (value >> 7) & 1 != 0 {
                    self.master.enable_channel(0);
                    self.ch1.start();
                }
            }
            0xff15 => {}
            0xff16 => self.ch2.nrx1 = value,
            0xff17 => self.ch2.nrx2 = value,
            0xff18 => self.ch2.nrx3 = value,
            0xff19 => {
                self.ch2.nrx4 = value & 0xc7;
                if (value >> 7) & 1 != 0 {
                    self.master.enable_channel(1);
                    self.ch2.start();
                }
            }
            0xff1a => self.ch3.nrx0 = value & 0x80,
            0xff1b => self.ch3.nrx1 = value,
            0xff1c => self.ch3.nrx2 = value & 0x60,
            0xff1d => self.ch3.nrx3 = value,
            0xff1e => {
                self.ch3.nrx4 = value & 0xc7;
                if (value >> 7) & 1 != 0 {
                    self.master.enable_channel(2);
                    self.ch3.start();
                }
            }
            0xff1f => {}
            0xff20 => self.ch4.nrx1 = value & 0x3f,
            0xff21 => self.ch4.nrx2 = value,
            0xff22 => self.ch4.nrx3 = value,
            0xff23 => {
                self.ch4.nrx4 = value & 0xc0;
                if (value >> 7) & 1 != 0 {
                    self.master.enable_channel(3);
                    self.ch4.start();
                }
            }
            0xff24 => self.master.volume = value,
            0xff25 => self.master.panning = value,
            0xff26 => self.master.control = (self.master.control & 0x0f) | (value & 0x80),
            0xff27..=0xff2f => {}
            0xff30..=0xff3f => self.ch3.waveform[addr as usize - 0xff30] = value,
            _ => unreachable!(),
        }
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
    const TICK_RATE: usize = APU_SAMPLE_RATE / 256;

    pub fn start(&mut self, length: u8, enabled: bool) {
        self.length = length;
        self.enabled = enabled;
        self.ticks = 0;
    }

    pub fn current_state(&mut self) -> bool {
        if !self.enabled {
            return true;
        }
        if self.length == 0 {
            return false;
        }
        self.ticks += 1;
        if self.ticks >= Self::TICK_RATE {
            self.ticks = 0;
            self.length -= 1;
        }
        true
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
    const TICK_RATE: usize = APU_SAMPLE_RATE / 64;

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

pub enum PeriodSweepResult {
    Update(u16),
    Disable,
    Nothing,
}

pub trait SweepControl {
    fn start(&mut self, period: u16, direction: SweepDir, pace: u8, step: u8);
    fn current_period(&mut self) -> PeriodSweepResult;
}

impl SweepControl for () {
    fn start(&mut self, _period: u16, _direction: SweepDir, _pace: u8, _step: u8) {}

    fn current_period(&mut self) -> PeriodSweepResult {
        PeriodSweepResult::Nothing
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
    const TICK_RATE: usize = APU_SAMPLE_RATE / 128;
}

impl SweepControl for PeriodSweep {
    fn start(&mut self, period: u16, direction: SweepDir, pace: u8, step: u8) {
        self.period = period;
        self.direction = direction;
        self.pace = pace;
        self.step = step;
        self.ticks = 0;
    }

    fn current_period(&mut self) -> PeriodSweepResult {
        if self.period == 0 {
            return PeriodSweepResult::Nothing;
        }
        self.ticks += 1;
        if self.pace > 0 && self.ticks >= Self::TICK_RATE * self.pace as usize {
            self.ticks = 0;
            let diff = self.period >> self.step;
            match self.direction {
                SweepDir::Increase => {
                    self.period = self.period.saturating_add(diff);
                    if self.period > 0x7ff {
                        self.period = 0;
                        return PeriodSweepResult::Disable;
                    }
                }
                SweepDir::Decrease => self.period = self.period.wrapping_sub(diff),
            };
            PeriodSweepResult::Update(self.period)
        } else {
            PeriodSweepResult::Nothing
        }
    }
}
