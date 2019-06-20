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
            console.log(romFile);
            emulator.load_rom(romFile);
            romLoaded = true;
        }
        else{
            requestAnimationFrame(renderLoop);
            return;
        }
    }

    emulator.tick();

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


var romInput = document.getElementById('rom-input');
romInput.onchange = e => { 
    var fr = new FileReader();
    fr.onload = () =>  {
      romFile = new Uint8Array(fr.result);
      console.log(romFile);
    };

    fr.readAsArrayBuffer(e.target.files[0]);
}

drawPixels();
requestAnimationFrame(renderLoop);


