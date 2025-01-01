use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use gabbro::{ButtonState, Joypad, Lcd, LcdColor, Speaker, APU_SAMPLE_RATE};
use sdl2::audio::AudioCallback;

use crate::AUDIO_SAMPLE_RATE;

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadError;

#[derive(Debug)]
pub struct MutexJoypad {
    state: Arc<Mutex<ButtonState>>,
}

impl MutexJoypad {
    pub fn new(state: Arc<Mutex<ButtonState>>) -> Self {
        Self { state }
    }
}

impl Joypad for MutexJoypad {
    type Error = ThreadError;
    fn get_button_state(&mut self) -> Result<ButtonState, Self::Error> {
        Ok(*self.state.lock().map_err(|_| ThreadError)?)
    }
}

pub enum LcdMessage {
    Pixel(LcdColor),
    Draw,
}

#[derive(Debug)]
pub struct ChannelLcd {
    pixel_snd: Sender<LcdMessage>,
    prev_time: Instant,
}

impl ChannelLcd {
    pub fn new(pixel_snd: Sender<LcdMessage>) -> Self {
        Self {
            pixel_snd,
            prev_time: Instant::now(),
        }
    }
}

impl Lcd for ChannelLcd {
    type Error = ThreadError;
    fn frame_ready(&mut self) -> Result<(), Self::Error> {
        self.pixel_snd
            .send(LcdMessage::Draw)
            .map_err(|_| ThreadError)?;
        let elapsed = self.prev_time.elapsed();
        if elapsed.as_millis() < 17 {
            thread::sleep(Duration::from_millis(17) - elapsed);
        }
        self.prev_time = Instant::now();
        Ok(())
    }
    fn push_pixel(&mut self, color: LcdColor) -> Result<(), Self::Error> {
        self.pixel_snd
            .send(LcdMessage::Pixel(color))
            .map_err(|_| ThreadError)
    }
}

#[derive(Debug)]
pub struct AudioReceiver {
    audio_rcv: Receiver<(f32, f32)>,
    volume: f32,
}

impl AudioReceiver {
    pub fn new(audio_rcv: Receiver<(f32, f32)>, volume: f32) -> Self {
        Self { audio_rcv, volume }
    }
}

impl AudioCallback for AudioReceiver {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for i in 0..out.len() / 2 {
            let (left, right) = self.audio_rcv.recv().expect("Audio receive error");
            out[i * 2] = left * self.volume;
            out[i * 2 + 1] = right * self.volume;
        }
    }
}

#[derive(Debug)]
pub struct AudioSender {
    sample_sum: (f32, f32),
    sample_count: usize,
    audio_snd: Sender<(f32, f32)>,
}

impl AudioSender {
    const SAMPLE_RATE: usize = APU_SAMPLE_RATE / AUDIO_SAMPLE_RATE;
    pub fn new(audio_snd: Sender<(f32, f32)>) -> Self {
        Self {
            sample_sum: (0., 0.),
            sample_count: 0,
            audio_snd,
        }
    }
}

impl Speaker for AudioSender {
    type Error = ThreadError;
    fn push_sample(&mut self, left: f32, right: f32) -> Result<(), Self::Error> {
        self.sample_sum.0 += left;
        self.sample_sum.1 += right;
        self.sample_count += 1;
        if self.sample_count >= Self::SAMPLE_RATE {
            let left_sample = self.sample_sum.0 / Self::SAMPLE_RATE as f32;
            let right_sample = self.sample_sum.1 / Self::SAMPLE_RATE as f32;
            self.audio_snd
                .send((left_sample, right_sample))
                .map_err(|_| ThreadError)?;
            self.sample_count = 0;
            self.sample_sum = (0., 0.);
        }
        Ok(())
    }
}
