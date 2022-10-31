import init, { initialize, RRRBuilder, Fetcher } from "./bin/rrr_wasm.js";

var rrr = null;

async function main() {
    await init();

    initialize();

    let canvas = document.body.appendChild(document.createElement("canvas"));
    canvas.setAttribute('class', 'canvas');
    canvas.width = 768;
    canvas.height = 512;

    const params = new URLSearchParams(window.location.search);
    const song_id = params.get("song_id");
    if (song_id == null) {
        throw "Add `?song_id=id` to the end of the url.";
    }

    const response = await (await fetch("https://www.flashflashrevolution.com/game/r3/r3-playlist.v2.php")).json();

    console.log(response);
    const [song] = response.songs.filter(obj => {
        return obj.level == song_id;
    });

    // Possible fetch progress reference https://javascript.info/fetch-progress
    var fetcher = await Fetcher.new(`https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id=${song.previewhash}&mode=2&type=ChartFFR_music`);
    var value = await fetcher.fetch_js();
    rrr = await new RRRBuilder().with_canvas(canvas).build(value);
    canvas.addEventListener("keyup", key_press);
}

function key_press(event) {
    console.log(event);
    if (event.code == "Space") {
        rrr.run_once();
    }
    canvas.removeEventListener("keyup", key_press);
}

main();
