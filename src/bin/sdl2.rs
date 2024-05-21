use gabbro::{Gameboy, Sdl2Joypad, Sdl2Lcd};
use std::{env, fs};

fn main() {
    #[cfg(feature = "logger")]
    env_logger::builder().parse_env("GABBRO_LOG").init();
    if let Some(rom_path) = env::args().nth(1) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let event_pump = sdl.event_pump().unwrap();

        let lcd = Sdl2Lcd::new(&video_subsystem, false);
        let joypad = Sdl2Joypad::new(event_pump);

        let rom = fs::read(rom_path)
            .map_err(|_| log::error!("ROM file could not be opened"))
            .unwrap();

        let mut gb = Gameboy::builder(rom).joypad(joypad).lcd(lcd).build();

        gb.run();
    } else {
        log::error!("Please provide a path to a valid Game Boy ROM.");
    }
}
