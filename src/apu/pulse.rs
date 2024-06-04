use crate::apu::SweepDir;

use super::{LengthTimer, PeriodSweep, VolumeEnvelope};

pub struct Pulse<const FS: bool>
where
    Self: SweepControl,
{
    period_sweep: <Self as SweepControl>::SweepType,
    volume_envelope: VolumeEnvelope,
    length_timer: LengthTimer,
    period_samples: usize,
    pub nrx0: <Self as SweepControl>::SweepRegType,
    pub nrx1: u8,
    pub nrx2: u8,
    pub nrx3: u8,
    pub nrx4: u8,
}

impl<const FS: bool> Pulse<FS>
where
    Self: SweepControl,
{
    pub fn new() -> Self {
        Self {
            period_sweep: <Self as SweepControl>::SweepType::default(),
            volume_envelope: VolumeEnvelope::default(),
            length_timer: LengthTimer::default(),
            period_samples: 0,
            nrx0: <Self as SweepControl>::SweepRegType::default(),
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

    pub fn period(&self) -> u16 {
        let low = self.nrx3 as u16;
        let high = (self.nrx4 as u16 & 0x07) << 8;
        high | low
    }

    pub fn length_enabled(&self) -> bool {
        (self.nrx4 >> 6) & 1 != 0
    }

    pub fn start(&mut self) {
        self.length_timer
            .start(self.length_timer(), self.length_enabled());
        self.volume_envelope
            .start(self.volume(), self.envelope_dir(), self.envelope_pace());
        self.start_sweep();
    }

    pub fn sample(&mut self) -> f32 {
        if self.length_timer.shut_down() {
            return 0.;
        }
        let volume = self.volume_envelope.current_volume();
        self.update_period();

        let period = (0x800 - self.period()) as usize * 8;
        if self.period_samples >= period {
            self.period_samples = 0;
        }

        let period_prog = self.period_samples as f32 / period as f32;
        let duty_cycle = self.duty_cycle();
        self.period_samples += 1;

        if period_prog < duty_cycle {
            volume
        } else {
            0.
        }
    }
}

pub trait SweepControl {
    type SweepType: Default;
    type SweepRegType: Default;
    fn start_sweep(&mut self);
    fn update_period(&mut self);
}

impl SweepControl for Pulse<false> {
    type SweepType = ();
    type SweepRegType = ();
    fn start_sweep(&mut self) {}
    fn update_period(&mut self) {}
}

impl SweepControl for Pulse<true> {
    type SweepType = PeriodSweep;
    type SweepRegType = u8;

    fn start_sweep(&mut self) {
        self.period_sweep.start(
            self.period(),
            self.sweep_direction(),
            self.sweep_pace(),
            self.sweep_step(),
        )
    }

    fn update_period(&mut self) {
        if let Some(period) = self.period_sweep.current_period() {
            self.nrx3 = (period & 0xff) as u8;
            self.nrx4 = (self.nrx4 & 0xf8) | ((period >> 8) & 0x07) as u8;
        }
    }
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
