import init, { initialize, RRRBuilder, RRR } from "./bin/rrr_wasm.js";

async function main() {
    await init();

    initialize();

    let canvas = document.body.appendChild(document.createElement("canvas"));
    canvas.setAttribute('class', 'canvas');
    canvas.width = 320;
    canvas.height = 240;

    var rrr = await new RRRBuilder().with_canvas(canvas).build();
    await rrr.run();
}

main();
