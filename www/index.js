import { Emulator } from "skylark-wasm";
import { memory } from "skylark-wasm/skylark_bg";

const PIXEL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const WHITE = "#FFFFFF";
const BLACK = "#000000";

// Construct the universe, and get its width and height.
const emulator = Emulator.new();
const width = emulator.width();
const height = emulator.height();

var romFile = null;
var romLoaded = false;

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("skylark-canvas");
canvas.height = (PIXEL_SIZE ) * height;
canvas.width = (PIXEL_SIZE) * width;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    if (!romLoaded){
        if (romFile){
            emulator.load_rom(romFile);
            romLoaded = true;
        }
        else{
            requestAnimationFrame(renderLoop);
            return;
        }
    }

    emulator.tick(Date.now());

    drawPixels();

    requestAnimationFrame(renderLoop);
};

const getIndex = (x, y) => {
    return x + y * width;
};

const drawPixels = () => {
    const pixelsPtr = emulator.pixels();
    const pixels = new Uint8Array(memory.buffer, pixelsPtr, width * height);

    ctx.beginPath();

    for (let x = 0; x < width; x++) {
        for (let y = 0; y < height; y++) {
            const idx = getIndex(x, y);

            ctx.fillStyle = pixels[idx]
                ? WHITE
                : BLACK;

            ctx.fillRect(
                x * (PIXEL_SIZE),
                y * (PIXEL_SIZE),
                PIXEL_SIZE,
                PIXEL_SIZE
            );
        }
    }

    ctx.stroke();
};

// Register ROM loader
var romInput = document.getElementById('rom-input');
romInput.onchange = e => { 
    var fr = new FileReader();
    fr.onload = () =>  {
      romFile = new Uint8Array(fr.result);
    };

    fr.readAsArrayBuffer(e.target.files[0]);
}

const keyMapping = {};
keyMapping[KeyboardEvent.DOM_VK_1] = 0x1;
keyMapping[KeyboardEvent.DOM_VK_2] = 0x2;
keyMapping[KeyboardEvent.DOM_VK_3] = 0x3;
keyMapping[KeyboardEvent.DOM_VK_4] = 0xc;
keyMapping[KeyboardEvent.DOM_VK_Q] = 0x4;
keyMapping[KeyboardEvent.DOM_VK_W] = 0x5;
keyMapping[KeyboardEvent.DOM_VK_E] = 0x6;
keyMapping[KeyboardEvent.DOM_VK_R] = 0xD;
keyMapping[KeyboardEvent.DOM_VK_A] = 0x7;
keyMapping[KeyboardEvent.DOM_VK_S] = 0x8;
keyMapping[KeyboardEvent.DOM_VK_D] = 0x9;
keyMapping[KeyboardEvent.DOM_VK_F] = 0xE;
keyMapping[KeyboardEvent.DOM_VK_Z] = 0xA;
keyMapping[KeyboardEvent.DOM_VK_X] = 0x0;
keyMapping[KeyboardEvent.DOM_VK_C] = 0xB;
keyMapping[KeyboardEvent.DOM_VK_V] = 0xF;

// Register keyboard events
document.addEventListener('keydown', e => onKeyChange(e, true));
document.addEventListener('keyup', e => onKeyChange(e, false));

function onKeyChange(e, pressed) {
    let mappedKey = keyMapping[e.keyCode];
    if (mappedKey) {
        emulator.key_change(mappedKey, pressed);
    }
}

drawPixels();
requestAnimationFrame(renderLoop);


