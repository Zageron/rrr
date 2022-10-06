#![forbid(unsafe_code)]
#![allow(unused)]

use rrr_render::{Renderer, RendererBuilder};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{self, WindowBuilder},
};

#[wasm_bindgen(inspectable)]
#[derive(Debug)]
pub struct RRR {
    renderer: Renderer,
}

#[wasm_bindgen]
impl RRR {
    #[wasm_bindgen]
    pub async fn run(&self) {
        log::info!("hello runner {:?}", self.renderer)
    }
}

#[wasm_bindgen(inspectable)]
#[derive(Debug, Default)]
pub struct RRRBuilder {
    canvas: Option<HtmlCanvasElement>,
}

#[wasm_bindgen]
impl RRRBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        RRRBuilder::default()
    }

    #[wasm_bindgen]
    pub fn with_canvas(self, canvas: Option<HtmlCanvasElement>) -> RRRBuilder {
        Self { canvas }
    }

    #[wasm_bindgen]
    pub async fn build(self) -> Result<RRR, JsValue> {
        let canv = self.canvas.clone().unwrap();

        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(canv.width() as f64, canv.height() as f64);
            let mut builder = WindowBuilder::new()
                .with_title("Hello Pixels + Web")
                .with_inner_size(size);

            // This is to fix a weird compiler bug.
            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowBuilderExtWebSys;
                builder = builder.with_canvas(self.canvas);
            }

            builder
                .with_min_inner_size(size)
                .build(&event_loop)
                .expect("WindowBuilder error")
        };

        let renderer = if let Ok(renderer) =
            RendererBuilder::new(canv.width(), canv.height(), &window)
                .build()
                .await
        {
            renderer
        } else {
            return Err("Bad canvas.".to_owned().into());
        };

        Ok(RRR { renderer })
    }
}

#[wasm_bindgen]
pub fn initialize() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().unwrap();
    log::info!("RRR loaded.");
}
