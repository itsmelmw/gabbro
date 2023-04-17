use gabbro::{Gameboy, LCD_HEIGHT, LCD_WIDTH};
use js_sys::{Uint8Array, Uint8ClampedArray};
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
mod interfaces;
use interfaces::{WasmJoypad, WasmLcd};

/// Starts the Game Boy Emulator. As it runs in an infinite loop,
/// it is recommended to call this inside of a Web Worker.
#[wasm_bindgen]
pub fn run_gameboy(rom: Uint8Array, lcd_buffer: Uint8Array, joypad_buffer: Uint8Array) {
    console_error_panic_hook::set_once();
    let rom = rom.to_vec();

    let lcd = Box::new(WasmLcd::new(lcd_buffer));
    let joypad = Box::new(WasmJoypad::new(joypad_buffer));

    let mut gb = Gameboy::builder(rom)
        .attach_lcd(lcd)
        .attach_joypad(joypad)
        .build();
    gb.run();
}

/// Updates the given canvas with the pixel colors stored in the LCD Buffer.
#[wasm_bindgen]
pub fn update_lcd_canvas(canvas: HtmlCanvasElement, lcd_buffer: Uint8Array) {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let mut buffer_copy = [0; 4 * LCD_WIDTH * LCD_HEIGHT];
    Uint8ClampedArray::new(&lcd_buffer.buffer()).copy_to(&mut buffer_copy);
    let image_data =
        ImageData::new_with_u8_clamped_array(Clamped(&buffer_copy), LCD_WIDTH as u32).unwrap();

    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    context
        .draw_image_with_html_canvas_element_and_dw_and_dh(
            &canvas,
            0.0,
            0.0,
            4.0 * LCD_WIDTH as f64,
            4.0 * LCD_HEIGHT as f64,
        )
        .unwrap();
}

/// Updates the Joypad Buffer given a KeyboardEvent code and whether it was pressed.
#[wasm_bindgen]
pub fn update_joypad_buffer(key_code: String, pressed: bool, joypad_buffer: Uint8Array) {
    let pressed = pressed as u8;
    match key_code.as_str() {
        "ArrowRight" => joypad_buffer.set_index(0, pressed),
        "ArrowLeft" => joypad_buffer.set_index(1, pressed),
        "ArrowUp" => joypad_buffer.set_index(2, pressed),
        "ArrowDown" => joypad_buffer.set_index(3, pressed),
        "KeyX" => joypad_buffer.set_index(4, pressed),
        "KeyZ" => joypad_buffer.set_index(5, pressed),
        "Backspace" => joypad_buffer.set_index(6, pressed),
        "Enter" => joypad_buffer.set_index(7, pressed),
        _ => (),
    }
}

/// The size (in bytes) of the buffer used to store pixel color values.
#[wasm_bindgen]
pub fn lcd_buffer_size() -> usize {
    4 * LCD_WIDTH * LCD_HEIGHT
}

/// The size (in bytes) of the buffer used to store keyboard input.
#[wasm_bindgen]
pub fn joypad_buffer_size() -> usize {
    8
}
