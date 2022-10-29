import init, { initialize, RRRBuilder, BenchmarkResults } from "./bin/rrr_wasm.js";

async function main() {
    await init();

    initialize();

    let canvas = document.body.appendChild(document.createElement("canvas"));
    canvas.setAttribute('class', 'canvas');
    canvas.width = 768;
    canvas.height = 512;

    var rrr = await new RRRBuilder().with_canvas(canvas).build();
    rrr.run_once();
}

export function update_benchmark_ui(data) {
    console.log("benchmark data: " + data);
}

main();
