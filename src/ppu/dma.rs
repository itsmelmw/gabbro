/// Emulates the Direct Memory Access feature of the PPU,
/// which copies a part of memory into OAM.
pub struct Dma {
    pub active: bool,
    source: u16,
    offset: u16,
}

impl Dma {
    /// Initializes a new DMA controller.
    pub fn new() -> Self {
        Self {
            active: false,
            source: 0x00,
            offset: 0x00,
        }
    }

    /// Activates the DMA process.
    pub fn start(&mut self, source: u8) {
        self.active = true;
        self.source = source as u16;
        self.offset = 0x00;
        log::debug!("PPU: DMA started");
    }

    /// Calculates the current source address to copy from.
    pub fn src(&self) -> u16 {
        (self.source << 8) | self.offset
    }

    /// Calculates the current destination address to copy to.
    pub fn dst(&self) -> u16 {
        0xfe00 | self.offset
    }

    /// Updates the state of the DMA after a byte has been copied.
    pub fn step(&mut self) {
        self.offset += 1;
        if self.offset >= 0xa0 {
            self.active = false;
            log::debug!("PPU: DMA ended");
        }
    }
}
