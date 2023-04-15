use crate::apu::Apu;
use crate::cartridge::Cartridge;
use crate::cpu::InterruptControl;
use crate::interfaces::GameboyJoypad;
use crate::interfaces::GameboyLcd;
use crate::interfaces::GameboySerial;
use crate::joypad::JoypadController;
use crate::ppu::Ppu;
use crate::serial::SerialController;
use crate::timer::Timer;

/// The bus which handles all reads and writes from/to memory.
/// Also used to access all parts of the Game Boy besides the CPU.
pub struct Bus {
    cart: Cartridge,
    ram: [u8; 0x2000],
    hram: [u8; 0x7f],
    joypad: JoypadController,
    serial: SerialController,
    timer: Timer,
    audio: Apu,
    ppu: Ppu,
    pub interrupts: InterruptControl,
}

impl Bus {
    /// Initializes all the emulated hardware and the memory of the Game Boy.
    /// Also prints information contained in the ROM header.
    pub fn new(
        rom: Vec<u8>,
        lcd: Box<dyn GameboyLcd>,
        joypad: Box<dyn GameboyJoypad>,
        serial: Box<dyn GameboySerial>,
    ) -> Self {
        let cart = Cartridge::new(rom)
            .map_err(|e| log::error!("Failed to parse ROM header: {}", e))
            .unwrap();
        cart.log_header();
        Self {
            cart,
            ram: [0; 0x2000],
            hram: [0; 0x7f],
            joypad: JoypadController::new(joypad),
            serial: SerialController::new(serial),
            timer: Timer::new(),
            audio: Apu::new(),
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
            0xff10..=0xff26 => self.audio.mem[addr as usize - 0xff10],
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
            0xff10..=0xff3f => self.audio.mem[addr as usize - 0xff10] = val,
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
        self.ppu.step(ints);
        self.joypad.step(ints);
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
