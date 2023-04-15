use gabbro::Gameboy;
use std::{env, fs};

fn main() {
    #[cfg(feature = "logger")]
    env_logger::builder().parse_env("GABBRO_LOG").init();
    if let Some(rom_path) = env::args().nth(1) {
        let rom = fs::read(rom_path)
            .map_err(|_| log::error!("ROM file could not be opened"))
            .unwrap();

        let mut gb = Gameboy::builder(rom).build();
        gb.run();
    } else {
        log::error!("Please provide a path to a valid Game Boy ROM.");
    }
}
