use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use gabbro::{ButtonState, Joypad, Lcd, LcdColor};

pub struct MutexJoypad {
    state: Arc<Mutex<ButtonState>>,
}

impl MutexJoypad {
    pub fn new(state: Arc<Mutex<ButtonState>>) -> Self {
        Self { state }
    }
}

impl Joypad for MutexJoypad {
    fn get_button_state(&mut self) -> ButtonState {
        *self.state.lock().unwrap()
    }
}

pub enum LcdMessage {
    Pixel(LcdColor),
    Draw,
}

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
    fn frame_ready(&mut self) {
        self.pixel_snd.send(LcdMessage::Draw).unwrap();
        let elapsed = self.prev_time.elapsed();
        if elapsed.as_millis() < 17 {
            thread::sleep(Duration::from_millis(17) - elapsed);
        }
        self.prev_time = Instant::now();
    }
    fn push_pixel(&mut self, color: LcdColor) {
        self.pixel_snd.send(LcdMessage::Pixel(color)).unwrap();
    }
}
