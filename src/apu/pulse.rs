use crate::apu::{LengthTimer, PeriodSweep, SweepDir, VolumeEnvelope};

pub struct Pulse<const FS: bool>
where
    Self: SweepControl,
{
    period_sweep: <Self as SweepControl>::SweepType,
    volume_envelope: VolumeEnvelope,
    length_timer: LengthTimer,
    waveform_idx: usize,
    ticks: usize,
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
    const WAVEFORM_SIZE: usize = 8;
    const WAVEFORMS: [[u8; 8]; 4] = [
        [1, 1, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 1],
    ];

    pub fn new() -> Self {
        Self {
            period_sweep: <Self as SweepControl>::SweepType::default(),
            volume_envelope: VolumeEnvelope::default(),
            length_timer: LengthTimer::default(),
            waveform_idx: 0,
            ticks: 0,
            nrx0: <Self as SweepControl>::SweepRegType::default(),
            nrx1: 0,
            nrx2: 0,
            nrx3: 0,
            nrx4: 0,
        }
    }

    fn length_timer(&self) -> u8 {
        0x40 - (self.nrx1 & 0x3f)
    }

    fn waveform(&self) -> &[u8; 8] {
        &Self::WAVEFORMS[(self.nrx1 >> 6) as usize]
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
        let low = self.nrx3 as u16;
        let high = (self.nrx4 as u16 & 0x07) << 8;
        0x800 - (high | low)
    }

    fn length_enabled(&self) -> bool {
        (self.nrx4 >> 6) & 1 != 0
    }

    pub fn start(&mut self) {
        self.ticks = 0;
        self.waveform_idx = 0;
        self.length_timer
            .start(self.length_timer(), self.length_enabled());
        self.volume_envelope
            .start(self.volume(), self.envelope_dir(), self.envelope_pace());
        self.start_sweep();
    }

    pub fn sample(&mut self) -> Option<f32> {
        if self.nrx2 & 0xf8 == 0 || !self.length_timer.current_state() {
            return None;
        }
        let volume = self.volume_envelope.current_volume();
        self.update_period();

        self.ticks += 1;
        if self.ticks >= self.period() as usize {
            self.ticks = 0;
            self.waveform_idx = (self.waveform_idx + 1) % Self::WAVEFORM_SIZE;
        }
        Some(volume * self.waveform()[self.waveform_idx] as f32)
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
            0x800 - self.period(),
            self.sweep_direction(),
            self.sweep_pace(),
            self.sweep_step(),
        )
    }

    fn update_period(&mut self) {
        if let Some(neg_period) = self.period_sweep.current_period() {
            self.set_period(neg_period);
        }
    }
}

impl Pulse<true> {
    fn sweep_step(&self) -> u8 {
        self.nrx0 & 0x07
    }

    fn sweep_direction(&self) -> SweepDir {
        match (self.nrx0 >> 3) & 1 {
            0 => SweepDir::Increase,
            1 => SweepDir::Decrease,
            _ => unreachable!(),
        }
    }

    fn sweep_pace(&self) -> u8 {
        (self.nrx0 >> 4) & 0x07
    }

    fn set_period(&mut self, period: u16) {
        self.nrx3 = (period & 0xff) as u8;
        self.nrx4 = (self.nrx4 & 0xf8) | ((period >> 8) & 0x07) as u8;
    }
}
