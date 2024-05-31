use crate::apu::SweepDir;

use super::{FrequencySweep, LengthTimer, VolumeEnvelope};

pub struct Pulse<const FS: bool> {
    frequency_sweep: FrequencySweep,
    volume_envelope: VolumeEnvelope,
    length_timer: LengthTimer,
    period_samples: usize,
    pub nrx0: u8,
    pub nrx1: u8,
    pub nrx2: u8,
    pub nrx3: u8,
    pub nrx4: u8,
}

impl<const FS: bool> Pulse<FS> {
    pub fn new() -> Self {
        Self {
            frequency_sweep: FrequencySweep::new(),
            volume_envelope: VolumeEnvelope::new(),
            length_timer: LengthTimer::new(),
            period_samples: 0,
            nrx0: 0,
            nrx1: 0,
            nrx2: 0,
            nrx3: 0,
            nrx4: 0,
        }
    }

    pub fn length_timer(&self) -> u8 {
        self.nrx1 & 0x3f
    }

    pub fn duty_cycle(&self) -> f32 {
        match (self.nrx1 >> 6) & 0x03 {
            0 => 0.125,
            1 => 0.25,
            2 => 0.5,
            3 => 0.75,
            _ => unreachable!(),
        }
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

    pub fn frequency(&self) -> u16 {
        let low = self.nrx3 as u16;
        let high = (self.nrx4 as u16 & 0x07) << 8;
        0x800 - (high | low)
    }

    pub fn length_enabled(&self) -> bool {
        (self.nrx4 >> 6) & 1 != 0
    }

    pub fn start(&mut self) {
        self.length_timer
            .start(self.length_timer(), self.length_enabled());
        self.volume_envelope
            .start(self.volume(), self.envelope_dir(), self.envelope_pace());
    }

    pub fn sample(&mut self) -> f32
    where
        Self: UpdateFrequency,
    {
        if self.length_timer.shut_down() {
            return 0.;
        }
        let vol = self.volume_envelope.current_volume();
        self.update_frequency();

        let period = self.frequency() as usize * 8;
        if self.period_samples >= period {
            self.period_samples = 0;
        }

        let period_prog = self.period_samples as f32 / period as f32;
        let duty_cycle = self.duty_cycle();
        self.period_samples += 1;

        if period_prog < duty_cycle {
            vol
        } else {
            -vol
        }
    }
}

pub trait UpdateFrequency {
    fn update_frequency(&mut self);
}

impl UpdateFrequency for Pulse<false> {
    fn update_frequency(&mut self) {}
}

impl UpdateFrequency for Pulse<true> {
    fn update_frequency(&mut self) {}
}

impl Pulse<true> {
    pub fn sweep_step(&self) -> u8 {
        self.nrx0 & 0x07
    }

    pub fn sweep_direction(&self) -> SweepDir {
        match (self.nrx0 >> 3) & 1 {
            0 => SweepDir::Increase,
            1 => SweepDir::Decrease,
            _ => unreachable!(),
        }
    }

    pub fn sweep_pace(&self) -> u8 {
        (self.nrx0 >> 4) & 0x07
    }
}
