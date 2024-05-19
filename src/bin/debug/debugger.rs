use gabbro::{Gameboy, Mnemonic, Regs};
use std::num::ParseIntError;

pub struct GameboyDebugger<'a> {
    gameboy: &'a mut Gameboy,
    input: String,
    output: String,
    breakpoints: Vec<u16>,
    watchpoints: Vec<u16>,
}

impl<'a> GameboyDebugger<'a> {
    pub fn new(gameboy: &'a mut Gameboy) -> Self {
        Self {
            gameboy,
            input: String::new(),
            output: String::new(),
            breakpoints: Vec::new(),
            watchpoints: Vec::new(),
        }
    }

    pub fn push_input(&mut self, c: char) {
        self.input.push(c)
    }

    pub fn pop_input(&mut self) {
        self.input.pop();
    }

    pub fn input(&self) -> &str {
        self.input.as_str()
    }

    pub fn output(&self) -> &str {
        self.output.as_str()
    }

    pub fn disasm(&self, amt: usize) -> Vec<(bool, u16, Vec<u8>, Mnemonic)> {
        let mut disassembly = Vec::new();
        let mut addr = self.gameboy.regs().pc();
        let mut at_pc = true;
        for _ in 0..amt {
            let (bytes, mnemonic) = self.gameboy.disasm_at(addr);
            let size = bytes.len() as u16;
            disassembly.push((at_pc, addr, bytes, mnemonic));
            at_pc = false;
            if let Some(next_addr) = addr.checked_add(size) {
                addr = next_addr;
            } else {
                break;
            }
        }
        disassembly
    }

    pub fn stack(&self, amt: usize) -> Vec<(bool, u16, u16)> {
        let mut stack = Vec::new();
        let sp = self.gameboy.regs().sp();
        let mut at_sp = true;
        for i in 0..amt {
            if let Some(addr) = sp.checked_add(i as u16 * 2) {
                let val = self.gameboy.read_mem(addr) as u16
                    | (self.gameboy.read_mem(addr + 1) as u16) << 8;
                stack.push((at_sp, addr, val));
                at_sp = false;
            } else {
                break;
            }
        }
        stack
    }

    pub fn registers(&self) -> &Regs {
        self.gameboy.regs()
    }

    pub fn breakpoints(&self) -> &Vec<u16> {
        &self.breakpoints
    }

    pub fn watchpoints(&self) -> Vec<(u16, u8)> {
        self.watchpoints
            .iter()
            .map(|&addr| (addr, self.gameboy.read_mem(addr)))
            .collect::<Vec<(u16, u8)>>()
    }

    pub fn run_command(&mut self) -> bool {
        self.output = match self.input.split(' ').collect::<Vec<&str>>()[..] {
            ["quit" | "q"] => return true,
            ["continue" | "c"] => loop {
                self.gameboy.step();
                if let Some(breakpoint) = self
                    .breakpoints
                    .iter()
                    .position(|bp| *bp == self.gameboy.regs().pc())
                {
                    break format!("Breakpoint {:02} hit", breakpoint);
                }
            },
            ["step" | "s", steps] => match steps.parse::<usize>() {
                Ok(n) => {
                    self.step_n(n);
                    format!("Executed {} instructions", n)
                }
                Err(_) => format!("Invalid number: {}", steps),
            },
            ["step" | "s"] => {
                self.step_n(1);
                "Executed 1 instruction".into()
            }
            ["read" | "r", addr] => match self.parse_addr(addr) {
                Ok(addr) => {
                    let val = self.gameboy.read_mem(addr);
                    format!("Value at address {:#06x}: {:#04x}", addr, val)
                }
                Err(_) => format!("Invalid address: {}", addr),
            },
            ["break" | "b", "add" | "a", addr] => match self.parse_addr(addr) {
                Ok(addr) => {
                    self.breakpoints.push(addr);
                    format!("Inserted breakpoint at {:#06x}", addr)
                }
                Err(_) => format!("Invalid address: {}", addr),
            },
            ["break" | "b", "remove" | "r", idx] => match idx.parse::<usize>() {
                Ok(idx) if idx < self.breakpoints.len() => {
                    let addr = self.breakpoints[idx];
                    self.breakpoints.remove(idx);
                    format!("Removed breakpoint {:02} at {:#06x}", idx, addr)
                }
                _ => format!("Invalid index: {}", idx),
            },
            ["watch" | "w", "add" | "a", addr] => match self.parse_addr(addr) {
                Ok(addr) => {
                    self.watchpoints.push(addr);
                    format!("Inserted watchpoint at {:#06x}", addr)
                }
                Err(_) => format!("Invalid address: {}", addr),
            },
            ["watch" | "w", "remove" | "r", idx] => match idx.parse::<usize>() {
                Ok(idx) if idx < self.watchpoints.len() => {
                    let addr = self.watchpoints[idx];
                    self.watchpoints.remove(idx);
                    format!("Removed watchpoint {:02} at {:#06x}", idx, addr)
                }
                _ => format!("Invalid index: {}", idx),
            },
            ["help" | "h"] => "Commands: quit, continue, step, read, break, watch, help".into(),
            _ => format!("Unknown command: {}", self.input),
        };
        self.input.clear();
        false
    }

    fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.gameboy.step()
        }
    }

    fn parse_addr(&self, input: &str) -> Result<u16, ParseIntError> {
        match input.strip_prefix("0x") {
            Some(hex) => u16::from_str_radix(hex, 16),
            None => input.parse::<u16>(),
        }
    }
}
