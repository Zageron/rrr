#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
#[cfg(target_arch = "wasm32")]
pub fn initialize() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().unwrap();
    log::info!("RRR loaded.");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
pub fn play(canvas: Option<HtmlCanvasElement>) {
    wasm_bindgen_futures::spawn_local(run(canvas));
}

#[cfg(target_arch = "wasm32")]
async fn run(canvas: Option<HtmlCanvasElement>) {
    use winit::platform::web::WindowBuilderExtWebSys;

    let canv = canvas.clone().unwrap();

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(canv.width() as f64, canv.height() as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels + Web")
            .with_inner_size(size)
            .with_canvas(canvas)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("WindowBuilder error")
    };

    let window = Rc::new(window);

    let _pixels = {
        let window_size = window.inner_size();

        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());

        Pixels::new_async(canv.width(), canv.height(), surface_texture)
            .await
            .expect("Pixels error")
    };
}
