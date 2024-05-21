use std::collections::BinaryHeap;

use crate::peripherals::LcdColor;

use super::{
    fifo::{FifoEntry, PixelFifo},
    oam::{Oam, SprPalette, Sprite},
    vram::{IdxType, TileType, Vram},
};

/// The `LCDC` IO register, allowing control of the PPU.
pub struct Lcdc {
    byte: u8,
}

impl Lcdc {
    /// Initializes the `LCDC` register.
    fn new() -> Self {
        Self { byte: 0x91 }
    }
    /// Reads the value of the `LCDC` register.
    pub fn byte(&self) -> u8 {
        self.byte
    }
    /// Writes `val` to the `LCDC` register.
    pub fn set_byte(&mut self, val: u8) {
        self.byte = val
    }
    /// Returns whether the background/window is enabled.
    fn bgw_enabled(&self) -> bool {
        self.byte & 1 != 0
    }
    /// Returns whether sprites are enabled.
    fn spr_enabled(&self) -> bool {
        (self.byte >> 1) & 1 != 0
    }
    /// Returns the current sprite tile type.
    fn spr_tile_type(&self) -> TileType {
        match (self.byte >> 2) & 1 {
            0 => TileType::Single,
            1 => TileType::Double,
            _ => unreachable!(),
        }
    }
    /// Returns the offset of the tile map to use for the background.
    fn bg_map_offset(&self) -> usize {
        match (self.byte >> 3) & 1 {
            0 => 0x0000,
            1 => 0x0400,
            _ => unreachable!(),
        }
    }
    /// Returns the index type to use when retrieving background and window tiles.
    fn bgw_tile_idx_type(&self) -> IdxType {
        match (self.byte >> 4) & 1 {
            0 => IdxType::Signed,
            1 => IdxType::Unsigned,
            _ => unreachable!(),
        }
    }
    /// Returns whether the window is enabled.
    fn win_enabled(&self) -> bool {
        (self.byte >> 5) & 1 != 0
    }
    /// Returns the offset of the tile map to use for the window.
    fn win_map_offset(&self) -> usize {
        match (self.byte >> 6) & 1 {
            0 => 0x0000,
            1 => 0x0400,
            _ => unreachable!(),
        }
    }
    /// Returns whether the LCD is enabled.
    fn lcd_enabled(&self) -> bool {
        (self.byte >> 7) & 1 != 0
    }
}

/// Represents a palette IO register. An 8-bit value consisting of 4 colors of 2 bits each.
pub struct Palette {
    byte: u8,
}

impl Palette {
    /// Initializes a new palette register with the value `val`.
    fn new(val: u8) -> Self {
        Self { byte: val }
    }
    /// Reads the value of the palette register.
    pub fn byte(&self) -> u8 {
        self.byte
    }
    /// Writes `val` to the palette register.
    pub fn set_byte(&mut self, val: u8) {
        self.byte = val
    }
    /// Maps the value `val` to a color in the palette.
    /// `val` should be a value between 1 and 4.
    pub fn color(&self, val: u8) -> LcdColor {
        match (self.byte >> (val * 2)) & 0x03 {
            0 => LcdColor::White,
            1 => LcdColor::LightGray,
            2 => LcdColor::DarkGray,
            3 => LcdColor::Black,
            _ => unreachable!(),
        }
    }
}

/// An enum representing the current fetcher state in the state machine.
pub enum FetchState {
    Index,
    Line0,
    Line1,
    Sleep,
    Push,
}

/// An enum representing the current target between background and window to fetch from.
#[derive(PartialEq, Eq)]
pub enum FetchTarget {
    Bck,
    Win,
}

/// Represents the Pixel Fetcher of the Game Boy.
pub struct PixelFetcher {
    pub state: FetchState,
    pub target: FetchTarget,
    pub window_line: u8,
    pub line_tile: u8,
    pub line_sprites: BinaryHeap<Sprite>,
    pub queued_sprite: Option<Sprite>,
    pub fifo: PixelFifo,

    pub lcdc: Lcdc,

    pub bgp: Palette,
    pub obp0: Palette,
    pub obp1: Palette,

    pub ly: u8,
    pub scx: u8,
    pub scy: u8,
    pub wx: u8,
    pub wy: u8,

    pub vram: Vram,
    pub oam: Oam,

    pub idx: u8,
    pub low: u8,
    pub high: u8,
}

impl PixelFetcher {
    /// Initializes the Pixel Fetcher.
    pub fn new() -> Self {
        Self {
            state: FetchState::Index,
            target: FetchTarget::Bck,
            window_line: 0,
            line_tile: 0,
            line_sprites: BinaryHeap::new(),
            queued_sprite: None,
            fifo: PixelFifo::new(),

            lcdc: Lcdc::new(),

            bgp: Palette::new(0xfc),
            obp0: Palette::new(0x00),
            obp1: Palette::new(0x00),

            ly: 0x91,
            scx: 0x00,
            scy: 0x00,
            wx: 0x00,
            wy: 0x00,

            vram: Vram::new(),
            oam: Oam::new(),

            idx: 0,
            low: 0,
            high: 0,
        }
    }

    /// Returns the number of pixels drawn on the current scanline.
    pub fn drawn_line(&self) -> usize {
        self.fifo.popped()
    }

    /// Adds the next sprite in OAM to the line sprites if it is on the current scanline
    /// and the line sprites list is not full.
    pub fn fetch_line_sprite(&mut self, dots: usize) {
        if dots % 2 == 1 && self.line_sprites.len() < 10 {
            let sprite = self.oam.sprite(dots / 2);
            if self.ly + 16 >= sprite.y()
                && self.ly + 16 < sprite.y() + self.lcdc.spr_tile_type().height()
            {
                self.line_sprites.push(sprite);
            }
        }
    }

    /// Emulates a step of the Pixel Fetcher. Implemented using a state machine.
    pub fn step(&mut self, dot: usize) {
        // Check if we should change the fetching target to the Window
        if self.target == FetchTarget::Bck
            && self.lcdc.win_enabled()
            && self.drawn_line() as u8 >= self.wx - 7
            && self.ly >= self.wy
        {
            self.fifo.clear();
            self.state = FetchState::Index;
            self.target = FetchTarget::Win;
            self.line_tile = 0;
        }
        // Check if we should change the fetching target to a Sprite
        if self.lcdc.spr_enabled() {
            if let Some(spr) = self.line_sprites.peek() {
                if self.drawn_line() + 8 >= spr.x() as usize {
                    self.queued_sprite = Some(self.line_sprites.pop().unwrap());
                }
            }
        }
        // Every step takes 2 dots, except pushing, which is attempted every dot.
        match self.state {
            FetchState::Index if dot % 2 == 0 => self.fetch_tile_idx(),
            FetchState::Line0 if dot % 2 == 0 => self.fetch_tile_low(),
            FetchState::Line1 if dot % 2 == 0 => self.fetch_tile_high(),
            FetchState::Sleep if dot % 2 == 0 => self.state = FetchState::Push,
            FetchState::Push => self.push_tile_data(),
            _ => (),
        }
    }

    /// Emulates a step of the Pixel Fetcher when it should fetch a tile index.
    fn fetch_tile_idx(&mut self) {
        self.idx = match self.target {
            FetchTarget::Bck => {
                let idx = ((self.scx / 8).wrapping_add(self.line_tile) as usize & 0x1f)
                    + (self.scy.wrapping_add(self.ly) as usize / 8) * 32;
                self.vram.map(self.lcdc.bg_map_offset() + idx)
            }
            FetchTarget::Win => {
                let idx = self.line_tile as usize + (self.window_line as usize / 8) * 32;
                self.vram.map(self.lcdc.win_map_offset() + idx)
            }
        };

        self.state = FetchState::Line0;
    }

    /// Emulates a step of the Pixel Fetcher when it should fetch the low pixel data of a tile.
    fn fetch_tile_low(&mut self) {
        let line_idx = match self.target {
            FetchTarget::Bck => (self.ly.wrapping_add(self.scy)) as usize % 8,
            FetchTarget::Win => self.window_line as usize % 8,
        };
        let line = self.vram.tile_line(
            self.idx,
            line_idx,
            self.lcdc.bgw_tile_idx_type(),
            TileType::Single,
        );
        self.low = line[0];

        self.state = FetchState::Line1;
    }

    /// Emulates a step of the Pixel Fetcher when it should fetch the high pixel data of a tile.
    fn fetch_tile_high(&mut self) {
        let line_idx = match self.target {
            FetchTarget::Bck => (self.ly.wrapping_add(self.scy)) as usize % 8,
            FetchTarget::Win => self.window_line as usize % 8,
        };
        let line = self.vram.tile_line(
            self.idx,
            line_idx,
            self.lcdc.bgw_tile_idx_type(),
            TileType::Single,
        );
        self.high = line[1];

        self.state = FetchState::Sleep;
    }

    /// Emulates a step of the Pixel Fetcher when it should push the tile data to the FIFO.
    fn push_tile_data(&mut self) {
        if let Some(sprite) = self.queued_sprite {
            self.push_sprite_data(sprite);
            self.queued_sprite = None;
        }
        if self.fifo.push_bgw(self.low, self.high) {
            self.line_tile += 1;
            self.state = FetchState::Index;
        }
    }

    /// Pushes sprite data to the Pixel FIFO. Currently done in a single Pixel Fetcher step.
    // TODO split in T-cycles.
    fn push_sprite_data(&mut self, sprite: Sprite) {
        let spr_line = (self.ly + 16 - sprite.y()) as usize;
        let line_idx = match sprite.flip_y() {
            true => (self.lcdc.spr_tile_type().height() as usize - 1) - spr_line,
            false => spr_line,
        };
        let line = self.vram.tile_line(
            sprite.tile(),
            line_idx,
            IdxType::Unsigned,
            self.lcdc.spr_tile_type(),
        );
        let (low, high) = match sprite.flip_x() {
            true => (line[0].reverse_bits(), line[1].reverse_bits()),
            false => (line[0], line[1]),
        };
        let discard = if sprite.x() < 8 { 7 - sprite.x() } else { 0 };
        self.fifo.push_spr(
            low << discard,
            high << discard,
            sprite.palette(),
            sprite.under_bg(),
        );
    }

    /// Returns the correct palette register according to `palette`.
    fn get_palette(&self, palette: SprPalette) -> &Palette {
        match palette {
            SprPalette::Obp0 => &self.obp0,
            SprPalette::Obp1 => &self.obp1,
        }
    }

    /// Pops a pixel from the Pixel FIFO, and returns the correct pixel color to display.
    pub fn pop_pixel(&mut self) -> Option<LcdColor> {
        if self.queued_sprite.is_none() {
            if let Some(entry) = self.fifo.pop() {
                return Some(match entry {
                    _ if !self.lcdc.lcd_enabled() => LcdColor::White,
                    // When there is no sprite, or it is transparent
                    // Then draw the background color if it is enabled
                    FifoEntry {
                        spr_c: 0,
                        bgw_c: color,
                        ..
                    } if self.lcdc.bgw_enabled() => self.bgp.color(color),
                    // When there is a sprite without the under-bg bit
                    // Then draw the sprite color
                    FifoEntry {
                        spr_under: false,
                        spr_c: color,
                        spr_pal: palette,
                        ..
                    } => self.get_palette(palette).color(color),
                    // When there is a sprite with the bit set, but the sprite has priority
                    // Then draw the sprite color
                    FifoEntry {
                        bgw_c: 0,
                        spr_under: true,
                        spr_c: color,
                        spr_pal: palette,
                    } => self.get_palette(palette).color(color),
                    // When there is a sprite with the bit set, and the background has priority
                    // Then draw the background color if it is enabled
                    FifoEntry {
                        spr_under: true,
                        bgw_c: color,
                        ..
                    } if self.lcdc.bgw_enabled() => self.bgp.color(color),
                    // Else, draw white
                    _ => LcdColor::White,
                });
            }
        }
        None
    }

    /// Resets the Pixel Fetcher for at the start of a scanline.
    pub fn start_line(&mut self) {
        self.state = FetchState::Index;
        self.target = FetchTarget::Bck;
        self.line_tile = 0;
        self.fifo.reset(self.scx as usize % 8);
    }

    /// Resets the Pixel Fetcher for at the end of a scanline.
    pub fn end_line(&mut self) {
        self.line_sprites.clear();
    }

    /// Resets the Pixel Fetcher for at the end of a frame.
    pub fn end_frame(&mut self) {
        self.end_line();
        self.window_line = 0;
    }
}
