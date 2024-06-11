use crate::{
    apu::Apu,
    cartridge::Cartridge,
    cpu::interrupts::InterruptControl,
    joypad::JoypadController,
    peripherals::{Cable, Joypad, Lcd, Speaker},
    ppu::Ppu,
    serial::SerialController,
    timer::Timer,
};

/// The bus which handles all reads and writes from/to memory.
/// Also used to access all parts of the Game Boy besides the CPU.
pub struct Bus<L, S, J, C>
where
    L: Lcd,
    S: Speaker,
    J: Joypad,
    C: Cable,
{
    cart: Cartridge,
    ram: [u8; 0x2000],
    hram: [u8; 0x7f],
    joypad: JoypadController<J>,
    serial: SerialController<C>,
    timer: Timer,
    apu: Apu<S>,
    ppu: Ppu<L>,
    pub interrupts: InterruptControl,
}

impl<L, S, J, C> Bus<L, S, J, C>
where
    L: Lcd,
    S: Speaker,
    J: Joypad,
    C: Cable,
{
    /// Initializes all the emulated hardware and the memory of the Game Boy.
    /// Also prints information contained in the ROM header.
    pub fn new(rom: Vec<u8>, lcd: L, speaker: S, joypad: J, cable: C) -> Self {
        let cart = Cartridge::new(rom)
            .map_err(|e| log::error!("Failed to parse ROM header: {}", e))
            .unwrap();
        cart.log_header();
        Self {
            cart,
            ram: [0; 0x2000],
            hram: [0; 0x7f],
            joypad: JoypadController::new(joypad),
            serial: SerialController::new(cable),
            timer: Timer::new(),
            apu: Apu::new(speaker),
            ppu: Ppu::new(lcd),
            interrupts: InterruptControl::new(),
        }
    }

    /// Reads a value from the memory mapped at `addr`.
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // ROM
            0x0000..=0x7fff => self.cart.mbc.read_rom(addr),
            // Video RAM
            0x8000..=0x9fff => self.ppu.fetcher.vram.read(addr - 0x8000),
            // External Working RAM
            0xa000..=0xbfff => self.cart.mbc.read_ram(addr - 0xa000),
            // Working RAM (+ Echo)
            0xc000..=0xdfff => self.ram[addr as usize - 0xc000],
            0xe000..=0xfdff => {
                log::debug!("Read from echo RAM at address {:#06x}", addr);
                self.ram[addr as usize - 0xe000]
            }
            // Object Attribute Memory
            0xfe00..=0xfe9f => self.ppu.fetcher.oam.read(addr - 0xfe00),
            // IO Registers
            // Joypad
            0xff00 => self.joypad.p1.byte(),
            // Serial
            0xff01 => self.serial.sb,
            0xff02 => self.serial.sc,
            // Timer
            0xff04 => (self.timer.div >> 8) as u8,
            0xff05 => self.timer.tima,
            0xff06 => self.timer.tma,
            0xff07 => self.timer.tac.byte(),
            // APU
            0xff10 => self.apu.ch1.nrx0,
            0xff11 => self.apu.ch1.nrx1,
            0xff12 => self.apu.ch1.nrx2,
            0xff13 => self.apu.ch1.nrx3,
            0xff14 => self.apu.ch1.nrx4,
            0xff16 => self.apu.ch2.nrx1,
            0xff17 => self.apu.ch2.nrx2,
            0xff18 => self.apu.ch2.nrx3,
            0xff19 => self.apu.ch2.nrx4,
            0xff1a => self.apu.ch3.nrx0,
            0xff1b => self.apu.ch3.nrx1,
            0xff1c => self.apu.ch3.nrx2,
            0xff1d => self.apu.ch3.nrx3,
            0xff1e => self.apu.ch3.nrx4,
            0xff20 => self.apu.ch4.nrx1,
            0xff21 => self.apu.ch4.nrx2,
            0xff22 => self.apu.ch4.nrx3,
            0xff23 => self.apu.ch4.nrx4,
            0xff24 => self.apu.master.volume,
            0xff25 => self.apu.master.panning,
            0xff26 => self.apu.master.control,
            0xff30..=0xff3f => self.apu.ch3.waveform[addr as usize - 0xff30],
            // PPU
            0xff40 => self.ppu.fetcher.lcdc.byte(),
            0xff41 => self.ppu.stat.byte(),
            0xff42 => self.ppu.fetcher.scy,
            0xff43 => self.ppu.fetcher.scx,
            0xff44 => self.ppu.fetcher.ly,
            0xff45 => self.ppu.lyc,
            0xff46 => 0xff,
            0xff47 => self.ppu.fetcher.bgp.byte(),
            0xff48 => self.ppu.fetcher.obp0.byte(),
            0xff49 => self.ppu.fetcher.obp1.byte(),
            0xff4a => self.ppu.fetcher.wy,
            0xff4b => self.ppu.fetcher.wx,
            // High RAM
            0xff80..=0xfffe => self.hram[addr as usize - 0xff80],
            // Interrupts
            0xff0f => self.interrupts.flags.byte(),
            0xffff => self.interrupts.enable.byte(),
            _ => {
                log::debug!("Read from unmapped address {:#06x}", addr);
                0xff
            }
        }
    }

    /// Writes the value `val` to the memory mapped at `addr`.
    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            // ROM
            0x0000..=0x7fff => self.cart.mbc.write_rom(addr, val),
            // Video RAM
            0x8000..=0x9fff => self.ppu.fetcher.vram.write(addr - 0x8000, val),
            // External Working RAM
            0xa000..=0xbfff => self.cart.mbc.write_ram(addr - 0xa000, val),
            // Working RAM (+ Echo)
            0xc000..=0xdfff => self.ram[addr as usize - 0xc000] = val,
            0xe000..=0xfdff => {
                self.ram[addr as usize - 0xe000] = val;
                log::debug!("Write to echo RAM at address {:#06x}", addr);
            }
            // Object Attribute Memory
            0xfe00..=0xfe9f => self.ppu.fetcher.oam.write(addr - 0xfe00, val),
            // IO Registers
            // Joypad
            0xff00 => self.joypad.p1.set_byte(val),
            // Serial
            0xff01 => self.serial.sb = val,
            0xff02 => self.serial.sc = val,
            // Timer
            0xff04 => self.timer.div = 0,
            0xff05 => self.timer.tima = val,
            0xff06 => self.timer.tma = val,
            0xff07 => self.timer.tac.set_byte(val),
            // APU
            0xff10 => self.apu.ch1.nrx0 = val,
            0xff11 => self.apu.ch1.nrx1 = val,
            0xff12 => self.apu.ch1.nrx2 = val,
            0xff13 => self.apu.ch1.nrx3 = val,
            0xff14 => {
                self.apu.ch1.nrx4 = val;
                if (val >> 7) & 1 != 0 {
                    self.apu.ch1.start();
                }
            }
            0xff16 => self.apu.ch2.nrx1 = val,
            0xff17 => self.apu.ch2.nrx2 = val,
            0xff18 => self.apu.ch2.nrx3 = val,
            0xff19 => {
                self.apu.ch2.nrx4 = val;
                if (val >> 7) & 1 != 0 {
                    self.apu.ch2.start();
                }
            }
            0xff1a => self.apu.ch3.nrx0 = val,
            0xff1b => self.apu.ch3.nrx1 = val,
            0xff1c => self.apu.ch3.nrx2 = val,
            0xff1d => self.apu.ch3.nrx3 = val,
            0xff1e => {
                self.apu.ch3.nrx4 = val;
                if (val >> 7) & 1 != 0 {
                    self.apu.ch3.start();
                }
            }
            0xff20 => self.apu.ch4.nrx1 = val,
            0xff21 => self.apu.ch4.nrx2 = val,
            0xff22 => self.apu.ch4.nrx3 = val,
            0xff23 => {
                self.apu.ch4.nrx4 = val;
                if (val >> 7) & 1 != 0 {
                    self.apu.ch4.start();
                }
            }
            0xff24 => self.apu.master.volume = val,
            0xff25 => self.apu.master.panning = val,
            0xff26 => self.apu.master.control = val,
            0xff30..=0xff3f => self.apu.ch3.waveform[addr as usize - 0xff30] = val,
            // PPU
            0xff40 => self.ppu.fetcher.lcdc.set_byte(val),
            0xff41 => self.ppu.stat.set_byte(val),
            0xff42 => self.ppu.fetcher.scy = val,
            0xff43 => self.ppu.fetcher.scx = val,
            0xff44 => self.ppu.fetcher.ly = val,
            0xff45 => self.ppu.lyc = val,
            0xff46 => self.ppu.dma.start(val),
            0xff47 => self.ppu.fetcher.bgp.set_byte(val),
            0xff48 => self.ppu.fetcher.obp0.set_byte(val),
            0xff49 => self.ppu.fetcher.obp1.set_byte(val),
            0xff4a => self.ppu.fetcher.wy = val,
            0xff4b => self.ppu.fetcher.wx = val,
            // High RAM
            0xff80..=0xfffe => self.hram[addr as usize - 0xff80] = val,
            // Interrupts
            0xff0f => self.interrupts.flags.set_byte(val),
            0xffff => self.interrupts.enable.set_byte(val),
            _ => log::debug!("Write to unmapped address {:#06x}", addr),
        }
    }

    /// Emulates a machine cycle for all parts of the Game Boy that are stored in the [`Bus`].
    /// This does not include the CPU.
    pub fn io_step(&mut self) {
        self.dma_step();

        let ints = &mut self.interrupts.flags;
        self.joypad.step(ints);
        self.ppu.step(ints);
        self.apu.step();
        self.serial.step(ints);
        self.timer.step(ints);
    }

    /// Performs a step of the Direct Memory Access feature of the PPU when active.
    fn dma_step(&mut self) {
        if self.ppu.dma.active {
            let val = self.read(self.ppu.dma.src());
            self.write(self.ppu.dma.dst(), val);
            self.ppu.dma.step();
        }
    }
}
