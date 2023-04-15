/// Represents a line of 8 pixels of a tile in memory.
pub type TileLine = [u8; 2];
/// Represents an 8x8 tile in memory.
pub type Tile = [TileLine; 8];
/// Represents an 8x16 tile in memory.
pub type DoubleTile = [TileLine; 16];

/// An enum representing the selected method of reading tiles from VRAM.
pub enum IdxType {
    Signed = 0,
    Unsigned = 1,
}

/// An enum representing whether an 8x8 or 8x16 tile should be read from VRAM.
pub enum TileType {
    Single = 0,
    Double = 1,
}

impl TileType {
    /// Returns the tile height represented by the type.
    pub fn height(&self) -> u8 {
        match self {
            TileType::Single => 8,
            TileType::Double => 16,
        }
    }
}

/// Represents the Video RAM of the Game Boy.
pub union Vram {
    bytes: [u8; 0x2000],
    tiles: [Tile; 0x180],
    double_tiles: [DoubleTile; 0xc0],
}

impl Vram {
    /// Initializes the VRAM.
    pub fn new() -> Self {
        Self { bytes: [0; 0x2000] }
    }
    /// Reads the value at `addr` in VRAM.
    pub fn read(&self, addr: u16) -> u8 {
        unsafe { self.bytes[addr as usize] }
    }
    /// Writes `val` to `addr` in VRAM.
    pub fn write(&mut self, addr: u16, val: u8) {
        unsafe { self.bytes[addr as usize] = val }
    }
    /// Reads the `idx`th tile index in the tile map.
    /// `idx` should already have the correct offset applied.
    pub fn map(&self, idx: usize) -> u8 {
        unsafe { self.bytes[0x1800 + idx] }
    }
    /// Reads an 8-pixel line of a tile from VRAM.
    pub fn tile_line(
        &self,
        idx: u8,
        line: usize,
        idx_type: IdxType,
        tile_type: TileType,
    ) -> TileLine {
        let tile_idx = match idx_type {
            IdxType::Signed => ((idx as i8) as usize).wrapping_add(256),
            IdxType::Unsigned => idx as usize,
        };
        unsafe {
            match tile_type {
                TileType::Single => self.tiles[tile_idx][line],
                TileType::Double => self.double_tiles[tile_idx / 2][line],
            }
        }
    }
}
