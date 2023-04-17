declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* Starts the Game Boy Emulator. As it runs in an infinite loop,
	* it is recommended to call this inside of a Web Worker.
	* @param {Uint8Array} rom
	* @param {Uint8Array} lcd_buffer
	* @param {Uint8Array} joypad_buffer
	*/
	export function run_gameboy(rom: Uint8Array, lcd_buffer: Uint8Array, joypad_buffer: Uint8Array): void;
	/**
	* Updates the given canvas with the pixel colors stored in the LCD Buffer.
	* @param {HTMLCanvasElement} canvas
	* @param {Uint8Array} lcd_buffer
	*/
	export function update_lcd_canvas(canvas: HTMLCanvasElement, lcd_buffer: Uint8Array): void;
	/**
	* Updates the Joypad Buffer given a KeyboardEvent code and whether it was pressed.
	* @param {string} key_code
	* @param {boolean} pressed
	* @param {Uint8Array} joypad_buffer
	*/
	export function update_joypad_buffer(key_code: string, pressed: boolean, joypad_buffer: Uint8Array): void;
	/**
	* The size (in bytes) of the buffer used to store pixel color values.
	* @returns {number}
	*/
	export function lcd_buffer_size(): number;
	/**
	* The size (in bytes) of the buffer used to store keyboard input.
	* @returns {number}
	*/
	export function joypad_buffer_size(): number;
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run_gameboy: (a: number, b: number, c: number) => void;
  readonly update_lcd_canvas: (a: number, b: number) => void;
  readonly update_joypad_buffer: (a: number, b: number, c: number, d: number) => void;
  readonly lcd_buffer_size: () => number;
  readonly joypad_buffer_size: () => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
