mod peripherals;

use gabbro::{ButtonState, Gameboy, LcdColor, LCD_HEIGHT, LCD_WIDTH};
use peripherals::{AudioReceiver, AudioSender, ChannelLcd, LcdMessage, MutexJoypad};
use sdl2::{audio::AudioSpecDesired, event::Event, keyboard::Scancode, pixels::PixelFormatEnum};
use std::{
    env, fs,
    sync::{mpsc, Arc, Mutex},
    thread,
};

const WINDOW_SCALE: usize = 4;
const AUDIO_SAMPLE_RATE: usize = 22050;

fn main() -> Result<(), String> {
    #[cfg(feature = "logger")]
    env_logger::builder().parse_env("GABBRO_LOG").init();
    let rom_path = env::args()
        .nth(1)
        .ok_or("Please provide a path to a valid Game Boy ROM.".to_string())?;
    let rom = fs::read(rom_path).map_err(|e| e.to_string())?;

    let sdl = sdl2::init()?;

    // Set up the render window
    let video_subsystem = sdl.video()?;

    let window = video_subsystem
        .window(
            "Gabbro Game Boy Emulator",
            (LCD_WIDTH * WINDOW_SCALE) as u32,
            (LCD_HEIGHT * WINDOW_SCALE) as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    canvas
        .set_logical_size(LCD_WIDTH as u32, LCD_HEIGHT as u32)
        .map_err(|e| e.to_string())?;

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_streaming(PixelFormatEnum::RGB888, LCD_WIDTH as u32, LCD_HEIGHT as u32)
        .map_err(|e| e.to_string())?;

    // Set up the audio
    let audio = sdl.audio()?;
    let spec = AudioSpecDesired {
        freq: Some(AUDIO_SAMPLE_RATE as i32),
        channels: Some(2),
        samples: Some(256),
    };
    let (audio_snd, audio_rcv) = mpsc::channel();
    let player = audio.open_playback(None, &spec, |_| AudioReceiver::new(audio_rcv, 0.1))?;
    player.resume();

    // Set up the event pump
    let mut event_pump = sdl.event_pump()?;

    // Set up the emulator
    let joypad_state = Arc::new(Mutex::new(ButtonState::new()));
    let (pixel_snd, pixel_rcv) = mpsc::channel();
    let mut pixel_buffer = [0u8; LCD_WIDTH * LCD_HEIGHT * 4];
    let mut pixel_ptr = 0;

    let lcd = ChannelLcd::new(pixel_snd);
    let joypad = MutexJoypad::new(joypad_state.clone());
    let speaker = AudioSender::new(audio_snd);

    thread::spawn(move || {
        let mut gb = Gameboy::builder(rom)
            .lcd(lcd)
            .joypad(joypad)
            .speaker(speaker)
            .build();
        gb.run();
    });

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    scancode: Some(button),
                    ..
                } => {
                    let mut state = joypad_state.lock().unwrap();
                    match button {
                        Scancode::Right => state.right = true,
                        Scancode::Left => state.left = true,
                        Scancode::Up => state.up = true,
                        Scancode::Down => state.down = true,
                        Scancode::X => state.a = true,
                        Scancode::Z => state.b = true,
                        Scancode::Backspace => state.select = true,
                        Scancode::Return => state.start = true,
                        _ => (),
                    }
                }
                Event::KeyUp {
                    scancode: Some(button),
                    ..
                } => {
                    let mut state = joypad_state.lock().unwrap();
                    match button {
                        Scancode::Right => state.right = false,
                        Scancode::Left => state.left = false,
                        Scancode::Up => state.up = false,
                        Scancode::Down => state.down = false,
                        Scancode::X => state.a = false,
                        Scancode::Z => state.b = false,
                        Scancode::Backspace => state.select = false,
                        Scancode::Return => state.start = false,
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        while let Ok(message) = pixel_rcv.try_recv() {
            match message {
                LcdMessage::Pixel(color) => {
                    let bytes = match color {
                        LcdColor::White => [0xff, 0xff, 0xff, 0x00],
                        LcdColor::LightGray => [0xaa, 0xaa, 0xaa, 0x00],
                        LcdColor::DarkGray => [0x55, 0x55, 0x55, 0x00],
                        LcdColor::Black => [0x00, 0x00, 0x00, 0x00],
                    };
                    for byte in bytes {
                        pixel_buffer[pixel_ptr] = byte;
                        pixel_ptr += 1;
                    }
                }
                LcdMessage::Draw => {
                    texture
                        .update(None, &pixel_buffer, 4 * LCD_WIDTH)
                        .map_err(|e| e.to_string())?;
                    canvas.copy(&texture, None, None)?;
                    canvas.present();
                    pixel_ptr = 0;
                }
            }
        }
    }

    Ok(())
}
