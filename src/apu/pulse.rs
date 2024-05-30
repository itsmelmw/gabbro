use crate::apu::{SweepDir, MAX_FREQ};

use super::{FrequencySweep, LengthTimer, VolumeEnvelope};

pub enum DutyCycle {
    OneEighth,
    Quarter,
    Half,
    ThreeQuarters,
}

pub struct Pulse<const FS: bool> {
    frequency_sweep: FrequencySweep,
    volume_envelope: VolumeEnvelope,
    length_timer: LengthTimer,
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

    pub fn duty_cycle(&self) -> DutyCycle {
        match (self.nrx1 >> 6) & 0x03 {
            0 => DutyCycle::OneEighth,
            1 => DutyCycle::Quarter,
            2 => DutyCycle::Half,
            3 => DutyCycle::ThreeQuarters,
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
        high | low
    }

    pub fn length_enabled(&self) -> bool {
        (self.nrx4 >> 6) & 1 != 0
    }

    pub fn start(&self) {}
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
