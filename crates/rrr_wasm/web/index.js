import init, { initialize, RRRBuilder, Fetcher } from "./bin/rrr_wasm.js";

async function main() {
    await init();

    initialize();

    let canvas = document.body.appendChild(document.createElement("canvas"));
    canvas.setAttribute('class', 'canvas');
    canvas.width = 768;
    canvas.height = 512;

    const params = new URLSearchParams(window.location.search);
    const hash = params.get("hash");
    if (hash == null) {
        throw "Add `?hash=songhash` to the end of the url.";
    }

    // Possible fetch progress reference https://javascript.info/fetch-progress
    var fetcher = await Fetcher.new(`https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id=${hash}&mode=2&type=ChartFFR_music`);
    var value = await fetcher.fetch_js();
    var rrr = await new RRRBuilder().with_canvas(canvas).build(value);
    rrr.run_once();
}

main();
