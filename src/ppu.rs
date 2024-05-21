pub mod dma;
pub mod fetcher;
pub mod fifo;
pub mod oam;
pub mod vram;
use crate::{
    cpu::interrupts::IntReg,
    peripherals::Lcd,
    ppu::{
        dma::Dma,
        fetcher::{FetchTarget, PixelFetcher},
    },
};

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;

pub const OAM_DOTS: usize = 80;
pub const LINE_DOTS: usize = 456;
pub const FRAME_LINES: u8 = 154;

/// An enum representing the current PPU state in the state machine.
enum PpuMode {
    Oam = 0x02,
    Draw = 0x03,
    Hblank = 0x00,
    Vblank = 0x01,
}

/// The `STAT` IO register, representing the current state of the PPU.
pub struct Stat {
    byte: u8,
}

impl Stat {
    /// Initializes a new `STAT` register.
    fn new() -> Self {
        Self { byte: 0x81 }
    }
    /// Reads the value of the `STAT` register.
    pub fn byte(&self) -> u8 {
        self.byte
    }
    /// Writes `val` to the `STAT` register.
    pub fn set_byte(&mut self, val: u8) {
        self.byte = val
    }
    /// Reads the current PPU mode.
    fn mode(&self) -> PpuMode {
        match self.byte & 3 {
            0 => PpuMode::Hblank,
            1 => PpuMode::Vblank,
            2 => PpuMode::Oam,
            3 => PpuMode::Draw,
            _ => unreachable!(),
        }
    }
    /// Sets the current PPU mode.
    fn set_mode(&mut self, mode: PpuMode) {
        self.byte = (self.byte & !3) | (mode as u8)
    }
    /// Sets the LY-compare bit, when `LY == LYC`/
    fn set_lyc(&mut self) {
        self.byte |= 1 << 2
    }
    /// Resets the LY-compare bit, when `LY != LYC`.
    fn reset_lyc(&mut self) {
        self.byte &= !(1 << 2)
    }
    /// Returns whether the LCDSTAT interrupt should be triggered on HBLANK.
    fn hblank_enabled(&self) -> bool {
        (self.byte >> 3) & 1 != 0
    }
    /// Returns whether the LCDSTAT interrupt should be triggered on VBLANK.
    fn vblank_enabled(&self) -> bool {
        (self.byte >> 4) & 1 != 0
    }
    /// Returns whether the LCDSTAT interrupt should be triggered when starting to read from OAM.
    fn oam_enabled(&self) -> bool {
        (self.byte >> 5) & 1 != 0
    }
    /// Returns whether the LCDSTAT interrupt should be triggered when `LY == LYC`.
    fn lyc_enabled(&self) -> bool {
        (self.byte >> 6) & 1 != 0
    }
}

/// Emulates the Game Boy Pixel Processing Unit.
pub struct Ppu<L>
where
    L: Lcd,
{
    pub stat: Stat,
    pub lyc: u8,

    pub dma: Dma,

    lcd: L,
    pub fetcher: PixelFetcher,

    line_dots: usize,
}

impl<L> Ppu<L>
where
    L: Lcd,
{
    /// Initializes a new PPU.
    pub fn new(lcd: L) -> Self {
        Self {
            stat: Stat::new(),
            lyc: 0x00,

            dma: Dma::new(),

            lcd,
            fetcher: PixelFetcher::new(),

            line_dots: 0,
        }
    }

    /// Emulates a machine cycle of the PPU. Implemented using a state machine.
    /// May request the VBLANK and/or LCDSTAT interrupt.
    pub fn step(&mut self, ints: &mut IntReg) {
        for _ in 0..4 {
            self.line_dots += 1;
            match self.stat.mode() {
                PpuMode::Oam => self.mode_oam(),
                PpuMode::Draw => self.mode_draw(ints),
                PpuMode::Hblank => self.mode_hblank(ints),
                PpuMode::Vblank => self.mode_vblank(ints),
            };
        }
    }

    /// Emulates a machine cycle of the PPU when it is in OAM mode.
    /// Fetches the 10 first sprites that will be drawn at the current scanline.
    fn mode_oam(&mut self) {
        self.fetcher.fetch_line_sprite(self.line_dots);
        if self.line_dots >= OAM_DOTS {
            self.fetcher.start_line();
            self.stat.set_mode(PpuMode::Draw);
        }
    }

    /// Emulates a machine cycle of the PPU when it is in DRAW mode.
    /// Pushes pixels to the LCD to draw.
    /// May request the HBLANK LCDSTAT interrupt.
    fn mode_draw(&mut self, ints: &mut IntReg) {
        self.fifo_step();
        if self.fetcher.drawn_line() >= LCD_WIDTH {
            if let FetchTarget::Win = self.fetcher.target {
                self.fetcher.window_line += 1;
            }
            self.stat.set_mode(PpuMode::Hblank);
            if self.stat.hblank_enabled() {
                ints.irq_lcdstat();
            }
        }
    }

    /// Emulates a machine cycle of the PPU when it is in HBLANK mode.
    /// May request the OAM or VBLANK LCDSTAT interrupt.
    fn mode_hblank(&mut self, ints: &mut IntReg) {
        if self.line_dots >= LINE_DOTS {
            self.line_dots = 0;
            self.inc_ly(ints);
            if self.fetcher.ly >= LCD_HEIGHT as u8 {
                self.lcd.frame_ready();
                self.fetcher.end_frame();
                ints.irq_vblank();
                self.stat.set_mode(PpuMode::Vblank);
                if self.stat.vblank_enabled() {
                    ints.irq_lcdstat()
                }
            } else {
                self.fetcher.end_line();
                self.stat.set_mode(PpuMode::Oam);
                if self.stat.oam_enabled() {
                    ints.irq_lcdstat();
                }
            }
        }
    }

    /// Emulates a machine cycle of the PPU when it is in VBLANK mode.
    /// May request the OAM LCDSTAT interrupt.
    fn mode_vblank(&mut self, ints: &mut IntReg) {
        if self.line_dots >= LINE_DOTS {
            self.line_dots = 0;
            self.inc_ly(ints);
            if self.fetcher.ly >= FRAME_LINES {
                self.fetcher.ly = 0;
                self.stat.set_mode(PpuMode::Oam);
                if self.stat.oam_enabled() {
                    ints.irq_lcdstat();
                }
            }
        }
    }

    /// Increments the `LY` register, which stores the current scanline the PPU is drawing.
    /// Also updates the LYC bit of the `STAT` register, and may request the LYC LCDSTAT interrupt.
    fn inc_ly(&mut self, ints: &mut IntReg) {
        self.fetcher.ly += 1;
        if self.fetcher.ly == self.lyc {
            self.stat.set_lyc();
            if self.stat.lyc_enabled() {
                ints.irq_lcdstat();
            }
        } else {
            self.stat.reset_lyc();
        }
    }

    /// Emulates a step of the pixel fifo during DRAW mode.
    fn fifo_step(&mut self) {
        // Advance the state of the fetcher
        self.fetcher.step(self.line_dots);
        // Push a pixel to the LCD
        self.draw_pixel();
    }

    /// Pushes a pixel to the LCD if available.
    fn draw_pixel(&mut self) {
        if let Some(lcdcolor) = self.fetcher.pop_pixel() {
            self.lcd.push_pixel(lcdcolor);
        }
    }
}
