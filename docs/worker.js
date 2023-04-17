importScripts("./pkg/gabbro_web.js");
const { run_gameboy } = wasm_bindgen;

async function init() {
    await wasm_bindgen("./pkg/gabbro_web_bg.wasm");

    self.onmessage = function (event) {
        var rom = event.data.rom;
        var lcdShMem = event.data.lcdShMem;
        var joypadShMem = event.data.joypadShMem;
        var lcdBuffer = new Uint8Array(lcdShMem);
        var joypadBuffer = new Uint8Array(joypadShMem);
        run_gameboy(rom, lcdBuffer, joypadBuffer);
    };
    postMessage("worker_ready");
}
init();