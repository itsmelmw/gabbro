use std::cmp::Ordering;

/// An enum representing a sprite palette register.
pub enum SprPalette {
    Obp0 = 0,
    Obp1 = 1,
}

/// Representation of a sprite in OAM.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Sprite {
    y: u8,
    x: u8,
    tile: u8,
    flags: u8,
}

impl Sprite {
    /// Reads the X coordinate of the sprite.
    pub fn x(&self) -> u8 {
        self.x
    }
    /// Reads the Y coordinate of the sprite.
    pub fn y(&self) -> u8 {
        self.y
    }
    /// Reads the index of the tile used by the sprite.
    pub fn tile(&self) -> u8 {
        self.tile
    }
    /// Returns the sprite palette register that should be used when drawing the sprite.
    pub fn palette(&self) -> SprPalette {
        match (self.flags >> 4) & 1 {
            0 => SprPalette::Obp0,
            1 => SprPalette::Obp1,
            _ => unreachable!(),
        }
    }
    /// Returns whether the sprite should be flipped on the X-axis when drawing the sprite.
    pub fn flip_x(&self) -> bool {
        (self.flags >> 5) & 1 != 0
    }
    /// Returns whether the sprite should be flipped on the Y-axis when drawing the sprite.
    pub fn flip_y(&self) -> bool {
        (self.flags >> 6) & 1 != 0
    }
    /// Returns whether the sprite should be drawn underneath parts of the background.
    pub fn under_bg(&self) -> bool {
        (self.flags >> 7) & 1 != 0
    }
}

impl PartialEq for Sprite {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x)
    }
}

impl Eq for Sprite {}

impl PartialOrd for Sprite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sprite {
    fn cmp(&self, other: &Self) -> Ordering {
        // Other way around intentionally, so that the BinaryHeap is a min heap.
        other.x.cmp(&self.x)
    }
}

/// Represents the Object Attribute Memory of the Game Boy.
pub union Oam {
    bytes: [u8; 0xa0],
    sprites: [Sprite; 40],
}

impl Oam {
    /// Initializes the OAM.
    pub fn new() -> Self {
        Self { bytes: [0; 0xa0] }
    }
    /// Reads the value at `addr` inside OAM.
    pub fn read(&self, addr: u16) -> u8 {
        unsafe { self.bytes[addr as usize] }
    }
    /// Writes `val` to `addr` inside OAM.
    pub fn write(&mut self, addr: u16, val: u8) {
        unsafe { self.bytes[addr as usize] = val }
    }
    /// Reads the `idx`th sprite from OAM.
    pub fn sprite(&self, idx: usize) -> Sprite {
        unsafe { self.sprites[idx] }
    }
}
