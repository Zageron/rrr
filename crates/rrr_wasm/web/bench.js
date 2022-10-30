import init, { initialize, RRRBuilder, Bencher } from "./bin/rrr_wasm.js";

class BenchUiElements {
    constructor() {
        this.avg_frame_time_element = document.body.querySelector("#avg_frame_time");
        this.one_percent_frame_time_element = document.body.querySelector("#one_percent_frame_time");
        this.tenth_percent_frame_time_element = document.body.querySelector("#tenth_percent_frame_time");
    }
}

var bencher = null;
var bencher_ui_elements = null;

async function main() {
    await init();

    initialize();

    var canvas = document.body.appendChild(document.createElement("canvas"));
    canvas.setAttribute('class', 'canvas');
    canvas.width = 768;
    canvas.height = 512;

    var rrr = await new RRRBuilder().with_canvas(canvas).build();
    bencher = new Bencher(performance.now());
    bencher_ui_elements = new BenchUiElements();
    rrr.run_once(() => { update_benchmark_ui(bencher, bencher_ui_elements); });
}

export function update_benchmark_ui(bencher, ui_elements) {
    bencher.update(performance.now());
    var data = bencher.current_data();
    ui_elements.avg_frame_time_element.textContent = (1000 / data.avg_frame_time).toFixed(3) + "fps";
    ui_elements.one_percent_frame_time_element.textContent = (1000 / data.one_percent_frame_time).toFixed(3) + "fps";
    ui_elements.tenth_percent_frame_time_element.textContent = (1000 / data.tenth_percent_frame_time).toFixed(3) + "fps";
}

main();
