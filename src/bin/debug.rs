use gabbro::Gameboy;
use std::{env, fs, io::stdin};

fn main() {
    if let Some(rom_path) = env::args().nth(1) {
        let rom = fs::read(rom_path)
            .map_err(|_| log::error!("ROM file could not be opened"))
            .unwrap();

        let mut gb = Gameboy::builder(rom).build();

        loop {
            let pc = gb.regs().pc();
            let (bytes, mnemonic) = gb.disasm_at(pc);
            println!(
                "{:#06x} {:08} - {}",
                pc,
                bytes
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<String>>()
                    .join(" "),
                mnemonic
            );

            let mut command = String::new();
            stdin().read_line(&mut command).unwrap();

            gb.step();
        }
    } else {
        log::error!("Please provide a path to a valid Game Boy ROM.");
    }
}
