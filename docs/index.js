const { lcd_buffer_size, joypad_buffer_size, update_lcd_canvas, update_joypad_buffer } = wasm_bindgen;

async function init() {
    await wasm_bindgen("./pkg/gabbro_web_bg.wasm");

    var lcdShMem = new SharedArrayBuffer(lcd_buffer_size());
    var joypadShMem = new SharedArrayBuffer(joypad_buffer_size());
    var lcdBuffer = new Uint8Array(lcdShMem);
    var joypadBuffer = new Uint8Array(joypadShMem);

    var lcd = document.getElementById("lcd");
    var lcdContext = lcd.getContext("2d");
    lcdContext.imageSmoothingEnabled = false;
    lcdContext.scale(4, 4);

    var fileInput = document.getElementById("rom");
    fileInput.onchange = function (event) {
        var files = fileInput.files;
        if (files.length == 1) {
            var rom_file = files[0];
            var worker = new Worker("./worker.js");

            worker.onmessage = function (event) {
                switch (event.data) {
                    case "worker_ready":
                        var reader = new FileReader();
                        reader.onloadend = function (event) {
                            var rom = new Uint8Array(reader.result);
                            msg = {
                                rom: rom,
                                lcdShMem: lcdShMem,
                                joypadShMem: joypadShMem,
                            };
                            worker.postMessage(msg);
                        }
                        reader.readAsArrayBuffer(rom_file);
                        break;
                    case "update_lcd":
                        update_lcd_canvas(lcd, lcdBuffer);
                        break;
                    default:
                        break;
                }
            }
        }
    };

    window.onkeydown = function (event) {
        update_joypad_buffer(event.code, true, joypadBuffer);
    }
    window.onkeyup = function (event) {
        update_joypad_buffer(event.code, false, joypadBuffer);
    }
}
init();
