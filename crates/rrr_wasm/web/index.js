import init, { initialize, play } from "./bin/rrr_wasm.js";

async function main() {
    await init();

    let canvas = document.body.appendChild(document.createElement("canvas"));
    canvas.setAttribute('class', 'canvas');
    canvas.width = 320;
    canvas.height = 240;

    play(canvas);
}

main();
