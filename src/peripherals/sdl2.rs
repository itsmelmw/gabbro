use super::{ButtonState, Joypad, Lcd, LcdColor};
use crate::{LCD_HEIGHT, LCD_WIDTH};
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::VideoSubsystem;
use sdl2::{event::Event, keyboard::Scancode, EventPump};
use std::cell::RefCell;
use std::thread;
use std::time::{Duration, Instant};

const SCALE: usize = 4;
const FRAME_TIME: u64 = 17;

/// Implementation of the Game Boy joypad using SDL2's [`EventPump`].
pub struct Sdl2Joypad {
    state: ButtonState,
    events: EventPump,
}

impl Sdl2Joypad {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            state: ButtonState::new(),
            events: event_pump,
        }
    }
}

impl Joypad for Sdl2Joypad {
    fn get_button_state(&mut self) -> ButtonState {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => panic!("Exited"),
                Event::KeyDown {
                    scancode: Some(button),
                    ..
                } => match button {
                    Scancode::Right => self.state.right = true,
                    Scancode::Left => self.state.left = true,
                    Scancode::Up => self.state.up = true,
                    Scancode::Down => self.state.down = true,
                    Scancode::X => self.state.a = true,
                    Scancode::Z => self.state.b = true,
                    Scancode::Backspace => self.state.select = true,
                    Scancode::Return => self.state.start = true,
                    _ => (),
                },
                Event::KeyUp {
                    scancode: Some(button),
                    ..
                } => match button {
                    Scancode::Right => self.state.right = false,
                    Scancode::Left => self.state.left = false,
                    Scancode::Up => self.state.up = false,
                    Scancode::Down => self.state.down = false,
                    Scancode::X => self.state.a = false,
                    Scancode::Z => self.state.b = false,
                    Scancode::Backspace => self.state.select = false,
                    Scancode::Return => self.state.start = false,
                    _ => (),
                },
                _ => (),
            }
        }
        self.state
    }
}

/// Implementation of the Game Boy LCD using SDL2's [`Canvas`] and [`Texture`].
/// Limits the FPS to around 60.
pub struct Sdl2Lcd {
    canvas: Canvas<Window>,
    _creator: TextureCreator<WindowContext>,
    texture: RefCell<Texture<'static>>,
    buffer: [u32; LCD_WIDTH * LCD_HEIGHT],
    ptr: usize,
    palette: [u32; 4],
    prev_time: Instant,
}

impl Sdl2Lcd {
    pub fn new(video_subsystem: &VideoSubsystem, retro: bool) -> Self {
        let window = video_subsystem
            .window(
                "Gabbro Game Boy Emulator",
                (LCD_WIDTH * SCALE) as u32,
                (LCD_HEIGHT * SCALE) as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().accelerated().build().unwrap();
        canvas
            .set_logical_size(LCD_WIDTH as u32, LCD_HEIGHT as u32)
            .unwrap();

        let creator = canvas.texture_creator();
        let texture = creator
            .create_texture_streaming(PixelFormatEnum::RGB888, LCD_WIDTH as u32, LCD_HEIGHT as u32)
            .unwrap();

        let texture = unsafe { std::mem::transmute::<_, Texture<'static>>(texture) };

        let palette = if retro {
            [0x009bbc0f, 0x008bac0f, 0x00306230, 0x000f380f]
        } else {
            [0x00ffffff, 0x00aaaaaa, 0x00555555, 0x00000000]
        };

        Self {
            canvas,
            _creator: creator,
            texture: RefCell::new(texture),
            buffer: [0; LCD_WIDTH * LCD_HEIGHT],
            ptr: 0,
            palette,
            prev_time: Instant::now(),
        }
    }
    fn raw_buffer(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.buffer.as_ptr() as *const u8, self.buffer.len() * 4)
        }
    }
}

impl Lcd for Sdl2Lcd {
    fn push_pixel(&mut self, color: LcdColor) {
        let val = self.palette[color as usize];
        self.buffer[self.ptr] = val;
        self.ptr += 1;
    }
    fn frame_ready(&mut self) {
        let mut texture = self.texture.borrow_mut();
        texture
            .update(None, self.raw_buffer(), 4 * LCD_WIDTH)
            .unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
        self.ptr = 0;

        let frame_time = self.prev_time.elapsed().as_millis() as u64;
        if frame_time > FRAME_TIME {
            log::warn!("Frame took too long: {}ms", frame_time);
        } else {
            thread::sleep(Duration::from_millis(17 - frame_time));
        }
        self.prev_time = Instant::now();
    }
}
