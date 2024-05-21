use gabbro::peripherals::{ButtonState, Joypad, Lcd, Serial, LcdColor};
use js_sys::{Atomics, Date, Int32Array, SharedArrayBuffer, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn postMessage(message: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn clear();
}

const FRAME_TIME: f64 = 17.;

pub struct WasmJoypad {
    buffer: Uint8Array,
}

impl WasmJoypad {
    pub fn new(buffer: Uint8Array) -> Self {
        Self { buffer }
    }
}

impl Joypad for WasmJoypad {
    fn get_button_state(&mut self) -> ButtonState {
        ButtonState {
            right: self.buffer.get_index(0) != 0,
            left: self.buffer.get_index(1) != 0,
            up: self.buffer.get_index(2) != 0,
            down: self.buffer.get_index(3) != 0,
            a: self.buffer.get_index(4) != 0,
            b: self.buffer.get_index(5) != 0,
            select: self.buffer.get_index(6) != 0,
            start: self.buffer.get_index(7) != 0,
        }
    }
}

pub struct WasmLcd {
    buffer: Uint8Array,
    ptr: u32,
    sleep_buffer: Int32Array,
    prev_time: f64,
}

impl WasmLcd {
    pub fn new(buffer: Uint8Array) -> Self {
        Self {
            buffer,
            ptr: 0,
            sleep_buffer: Int32Array::new(&SharedArrayBuffer::new(4)),
            prev_time: Date::now(),
        }
    }
}

impl WasmLcd {
    fn sleep(&mut self, time: f64) {
        Atomics::wait_with_timeout(&self.sleep_buffer, 0, 0, time).unwrap();
    }
}

impl Lcd for WasmLcd {
    fn push_pixel(&mut self, color: LcdColor) {
        let val = match color {
            LcdColor::White => [0xff, 0xff, 0xff, 0xff],
            LcdColor::LightGray => [0xaa, 0xaa, 0xaa, 0xff],
            LcdColor::DarkGray => [0x55, 0x55, 0x55, 0xff],
            LcdColor::Black => [0x00, 0x00, 0x00, 0xff],
        };
        for i in val {
            self.buffer.set_index(self.ptr, i);
            self.ptr += 1;
        }
    }

    fn frame_ready(&mut self) {
        postMessage("update_lcd");
        self.ptr = 0;

        let cur_time = Date::now();
        let time_diff = cur_time - self.prev_time;
        if time_diff <= FRAME_TIME {
            self.sleep(FRAME_TIME - time_diff);
        }
        self.prev_time = cur_time;
    }
}

pub struct WasmBlarggSerial {
    buffer: String,
}
impl WasmBlarggSerial {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
}
impl Serial for WasmBlarggSerial {
    fn transfer(&mut self, val: u8) {
        self.buffer.push(val as char);
        clear();
        log(&self.buffer);
    }
}
