import init, { initialize, play } from "./bin/rrr_wasm.js";

async function main() {
    await init();
    const shadow = document.body.attachShadow({ mode: 'open' });

    let canvas = shadow.appendChild(document.createElement("canvas"));
    canvas.setAttribute('class', 'canvas');
    canvas.width = 320;
    canvas.height = 240;

    shadow.appendChild(document.createElement('p'));
    play(canvas);
}

main();
