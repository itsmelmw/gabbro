use super::oam::SprPalette;

/// A bit queue used for Background and Window pixel information.
pub struct BgwBitQueue {
    queue: u8,
    len: usize,
}

impl BgwBitQueue {
    /// Initializes a new bit queue for background and window.
    pub fn new() -> Self {
        Self { queue: 0, len: 0 }
    }
    /// Fills the bit queue with `val`.
    pub fn set_byte(&mut self, val: u8) {
        self.queue = val;
        self.len = 8;
    }
    /// Pops a bit from the bit queue.
    pub fn pop(&mut self) -> u8 {
        if self.is_empty() {
            log::error!("Attempt to pop from an empty bit queue.");
            panic!("Aborting");
        }
        self.len -= 1;
        let val = (self.queue >> 7) & 1;
        self.queue <<= 1;
        val
    }
    /// Returns whether the bit queue is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Resets the bit queue to 0.
    pub fn clear(&mut self) {
        self.queue = 0;
        self.len = 0;
    }
}

/// A bit queue used for Sprite pixel information.
pub struct SprBitQueue {
    queue: u8,
}

impl SprBitQueue {
    /// Initializes a new bitqueue for sprites.
    pub fn new() -> Self {
        Self { queue: 0 }
    }
    /// Performs a bitwise or on the queue with `val`.
    pub fn or_byte(&mut self, val: u8) {
        self.queue |= val;
    }
    /// Reads the value of the bit queue.
    pub fn byte(&self) -> u8 {
        self.queue
    }
    /// Pops a value from the bit queue.
    pub fn pop(&mut self) -> u8 {
        let val = (self.queue >> 7) & 1;
        self.queue <<= 1;
        val
    }
    /// Resets the bit queue to 0.
    pub fn clear(&mut self) {
        self.queue = 0;
    }
}

/// Represents an entry in the pixel FIFO.
pub struct FifoEntry {
    pub bgw_c: u8,
    pub spr_c: u8,
    pub spr_pal: SprPalette,
    pub spr_under: bool,
}

/// Represents the Pixel FIFO of the Game Boy.
/// Consists of 6 bit queues containing background, window and sprite information.
pub struct PixelFifo {
    bgw0: BgwBitQueue,
    bgw1: BgwBitQueue,
    spr0: SprBitQueue,
    spr1: SprBitQueue,
    palt: SprBitQueue,
    prio: SprBitQueue,

    discard_bgw: usize,
    popped: usize,
}

impl PixelFifo {
    /// Initializes the Pixel FIFO.
    pub fn new() -> Self {
        Self {
            bgw0: BgwBitQueue::new(),
            bgw1: BgwBitQueue::new(),
            spr0: SprBitQueue::new(),
            spr1: SprBitQueue::new(),
            palt: SprBitQueue::new(),
            prio: SprBitQueue::new(),
            discard_bgw: 0,
            popped: 0,
        }
    }

    /// Returns whether the background/window FIFO's are empty.
    fn bgw_empty(&self) -> bool {
        self.bgw0.is_empty()
    }

    /// Returns how many times the Pixel FIFO has been popped from since the last call to `reset`.
    pub fn popped(&self) -> usize {
        self.popped
    }

    /// Resets the Pixel FIFO.
    pub fn reset(&mut self, discard_bgw: usize) {
        self.clear();
        self.discard_bgw = discard_bgw;
        self.popped = 0;
    }

    /// Clears the bit queues of the Pixel FIFO.
    pub fn clear(&mut self) {
        self.bgw0.clear();
        self.bgw1.clear();
        self.spr0.clear();
        self.spr1.clear();
        self.palt.clear();
        self.prio.clear();
    }

    /// Pushes background/window pixel data onto the Pixel FIFO.
    pub fn push_bgw(&mut self, low: u8, high: u8) -> bool {
        if !self.bgw_empty() {
            return false;
        }
        self.bgw0.set_byte(low);
        self.bgw1.set_byte(high);
        true
    }

    /// Pushes sprite pixel data onto the Pixel FIFO.
    pub fn push_spr(&mut self, low: u8, high: u8, palette: SprPalette, under_bg: bool) {
        let mask = !(self.spr0.byte() | self.spr1.byte());
        let palettes = match palette {
            SprPalette::Obp0 => 0x00,
            SprPalette::Obp1 => 0xff,
        };
        let priorities = match under_bg {
            false => 0x00,
            true => 0xff,
        };
        self.spr0.or_byte(low & mask);
        self.spr1.or_byte(high & mask);
        self.palt.or_byte(palettes & mask);
        self.prio.or_byte(priorities & mask);
    }

    /// Pops a pixel from the Pixel FIFO if possible.
    pub fn pop(&mut self) -> Option<FifoEntry> {
        if self.bgw_empty() {
            return None;
        }
        let bgw_c = (self.bgw1.pop() << 1) | self.bgw0.pop();
        if self.discard_bgw > 0 {
            self.discard_bgw -= 1;
            return None;
        }
        let spr_c = (self.spr1.pop() << 1) | self.spr0.pop();
        let spr_pal = match self.palt.pop() {
            0 => SprPalette::Obp0,
            1 => SprPalette::Obp1,
            _ => unreachable!(),
        };
        let spr_under = self.prio.pop() != 0;
        self.popped += 1;

        Some(FifoEntry {
            bgw_c,
            spr_c,
            spr_pal,
            spr_under,
        })
    }
}
