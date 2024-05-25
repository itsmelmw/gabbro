mod debugger;
mod ui;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use debugger::GameboyDebugger;
use gabbro::Gameboy;
use std::{env, fs, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

fn main() {
    if let Some(rom_path) = env::args().nth(1) {
        let rom = fs::read(rom_path)
            .map_err(|_| log::error!("ROM file could not be opened"))
            .unwrap();

        let mut gb = Gameboy::builder(rom).build();

        terminal::enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut debugger = GameboyDebugger::new(&mut gb);
        run_debugger(&mut terminal, &mut debugger).unwrap();

        terminal::disable_raw_mode().unwrap();
        execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    } else {
        log::error!("Please provide a path to a valid Game Boy ROM.");
    }
}

fn run_debugger<B: Backend>(
    terminal: &mut Terminal<B>,
    debugger: &mut GameboyDebugger,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, debugger))?;

        if crossterm::event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => debugger.push_input(c),
                    KeyCode::Backspace => debugger.pop_input(),
                    KeyCode::Enter => {
                        if debugger.run_command() {
                            return Ok(());
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
