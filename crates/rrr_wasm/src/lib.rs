#![allow(unused)]

use anyhow::{self, Result};
use js_sys::Function;
pub use rrr_fetch::platform::Fetcher;
use rrr_fetch::{Chart, FetchProgress};
use rrr_game::{
    hit_action,
    prelude::{Play, RuntimeChart, SongID, Turntable},
    Rendered, RustRustRevolution, RustRustRevolutionBuilder,
};
use rrr_input::KeyCode;
use rrr_record::{record::Record, RecordPressBuilder};
use rrr_render::{Renderer, RendererBuilder};
use rrr_time::Time;
use std::rc::Rc;
use wasm_bindgen::{
    prelude::{wasm_bindgen, *},
    JsValue,
};
use web_sys::HtmlCanvasElement;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{TouchPhase, VirtualKeyCode},
    event_loop::EventLoop,
    window::{self, Window, WindowBuilder},
};

#[cfg(feature = "bench")]
mod benchmark_callback;
#[cfg(feature = "bench")]
use rrr_bench::{BenchmarkData, BenchmarkResults};

#[wasm_bindgen(inspectable)]
#[derive(Debug)]
pub struct RRR {
    #[wasm_bindgen(skip)]
    pub rrr: RustRustRevolution<Rendered, Time>,

    #[wasm_bindgen(skip)]
    pub window: Window,

    #[wasm_bindgen(skip)]
    pub event_loop: EventLoop<()>,
}

#[allow(deprecated)]
#[wasm_bindgen]
impl RRR {
    pub fn run_once(mut self) {
        #[cfg(feature = "bench")]
        let bench = benchmark_callback::BenchmarkCallback {};
        #[cfg(feature = "bench")]
        let mut bench_data = BenchmarkData::default();
        #[cfg(feature = "bench")]
        let mut bench_counter = 0;

        wasm_bindgen_futures::spawn_local(async move {
            self.event_loop.run(move |in_event, _, control_flow| {
                control_flow.set_poll();

                match in_event {
                    winit::event::Event::NewEvents(_) => {}

                    winit::event::Event::DeviceEvent {
                        device_id: _,
                        event: _,
                    } => {}

                    winit::event::Event::UserEvent(_) => {}

                    winit::event::Event::Suspended => {}

                    winit::event::Event::Resumed => {}

                    winit::event::Event::MainEventsCleared => {
                        self.rrr.update();

                        #[cfg(feature = "bench")]
                        {
                            bench_data.add_frame_time(self.rrr.delta as f32);
                            if bench_counter > 60 {
                                bench_counter = 0;
                                unsafe {
                                    bench.run(&bench_data);
                                }
                            } else {
                                bench_counter += 1;
                            }
                        }

                        self.rrr.draw();
                        self.window.set_inner_size(PhysicalSize {
                            width: self.rrr.width(),
                            height: self.rrr.height(),
                        }); // Is this needed?
                        self.window.request_redraw();
                    }

                    winit::event::Event::RedrawRequested(_) => {}

                    winit::event::Event::RedrawEventsCleared => {
                        self.rrr.finish();
                    }

                    winit::event::Event::LoopDestroyed => {}

                    winit::event::Event::WindowEvent {
                        window_id: _,
                        event,
                    } => match event {
                        winit::event::WindowEvent::Resized(_size) => {}
                        winit::event::WindowEvent::Moved(_position) => {}
                        winit::event::WindowEvent::CloseRequested => {
                            control_flow.set_exit();
                        }
                        winit::event::WindowEvent::Destroyed => {}
                        winit::event::WindowEvent::DroppedFile(_) => {}
                        winit::event::WindowEvent::HoveredFile(_) => {}
                        winit::event::WindowEvent::HoveredFileCancelled => {}
                        winit::event::WindowEvent::ReceivedCharacter(_) => {}
                        winit::event::WindowEvent::Focused(_focused) => {}
                        winit::event::WindowEvent::KeyboardInput {
                            device_id: _,
                            input,
                            is_synthetic: _,
                        } => {
                            if let winit::event::ElementState::Pressed = input.state {
                                if let Some(virtual_keycode) = input.virtual_keycode {
                                    self.rrr.hit(hit_action::Builder::with_key_code(
                                        virtual_key_code_to_key_code(virtual_keycode),
                                    ));
                                }
                            }
                        }
                        winit::event::WindowEvent::ModifiersChanged(_) => {}
                        winit::event::WindowEvent::Ime(_) => {}
                        winit::event::WindowEvent::CursorMoved {
                            device_id: _,
                            position: _,
                            modifiers: _,
                        } => {}
                        winit::event::WindowEvent::CursorEntered { device_id: _ } => {}
                        winit::event::WindowEvent::CursorLeft { device_id: _ } => {}
                        winit::event::WindowEvent::MouseWheel {
                            device_id: _,
                            delta: _,
                            phase: _,
                            modifiers: _,
                        } => {}
                        winit::event::WindowEvent::MouseInput {
                            device_id: _,
                            state: _,
                            button: _,
                            modifiers: _,
                        } => {}
                        winit::event::WindowEvent::TouchpadPressure {
                            device_id: _,
                            pressure: _,
                            stage: _,
                        } => {}
                        winit::event::WindowEvent::AxisMotion {
                            device_id: _,
                            axis: _,
                            value: _,
                        } => {}
                        winit::event::WindowEvent::Touch(touch) => {
                            if touch.phase == TouchPhase::Ended {}
                        }
                        winit::event::WindowEvent::ScaleFactorChanged {
                            scale_factor: _,
                            new_inner_size: _,
                        } => {}
                        winit::event::WindowEvent::ThemeChanged(_) => {}
                        winit::event::WindowEvent::Occluded(_) => {}
                    },
                }
            });
        });
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
    pub fn with_benchmarking(self) -> RRRBuilder {
        Self {
            canvas: self.canvas,
        }
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

        let url = format!(
            "https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id={}&mode=2&type=ChartFFR_music",
            "f9b50c8a00667e711ff63ed2cd944f54"
        );

        let mut fetcher = Fetcher::new(url).await;

        assert!(fetcher.is_ok(), "{:?}", fetcher.err());

        let data = fetcher?.fetch().await;

        let record_press = RecordPressBuilder::from_swf(data);
        let record = record_press.press();

        let turntable = Turntable::load(record.unwrap());
        let play = Play::new(turntable);

        let mut rrr =
            RustRustRevolutionBuilder::with_renderer(renderer).build(play.start_with_audio());

        Ok(RRR {
            rrr,
            window,
            event_loop,
        })
    }
}

#[wasm_bindgen]
pub fn initialize() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().unwrap();
    log::info!("RRR loaded.");
}

fn virtual_key_code_to_key_code(value: VirtualKeyCode) -> KeyCode {
    match value {
        VirtualKeyCode::Key1 => KeyCode::Key1,
        VirtualKeyCode::Key2 => KeyCode::Key2,
        VirtualKeyCode::Key3 => KeyCode::Key3,
        VirtualKeyCode::Key4 => KeyCode::Key4,
        VirtualKeyCode::Key5 => KeyCode::Key5,
        VirtualKeyCode::Key6 => KeyCode::Key6,
        VirtualKeyCode::Key7 => KeyCode::Key7,
        VirtualKeyCode::Key8 => KeyCode::Key8,
        VirtualKeyCode::Key9 => KeyCode::Key9,
        VirtualKeyCode::Key0 => KeyCode::Key0,
        VirtualKeyCode::A => KeyCode::A,
        VirtualKeyCode::B => KeyCode::B,
        VirtualKeyCode::C => KeyCode::C,
        VirtualKeyCode::D => KeyCode::D,
        VirtualKeyCode::E => KeyCode::E,
        VirtualKeyCode::F => KeyCode::F,
        VirtualKeyCode::G => KeyCode::G,
        VirtualKeyCode::H => KeyCode::H,
        VirtualKeyCode::I => KeyCode::I,
        VirtualKeyCode::J => KeyCode::J,
        VirtualKeyCode::K => KeyCode::K,
        VirtualKeyCode::L => KeyCode::L,
        VirtualKeyCode::M => KeyCode::M,
        VirtualKeyCode::N => KeyCode::N,
        VirtualKeyCode::O => KeyCode::O,
        VirtualKeyCode::P => KeyCode::P,
        VirtualKeyCode::Q => KeyCode::Q,
        VirtualKeyCode::R => KeyCode::R,
        VirtualKeyCode::S => KeyCode::S,
        VirtualKeyCode::T => KeyCode::T,
        VirtualKeyCode::U => KeyCode::U,
        VirtualKeyCode::V => KeyCode::V,
        VirtualKeyCode::W => KeyCode::W,
        VirtualKeyCode::X => KeyCode::X,
        VirtualKeyCode::Y => KeyCode::Y,
        VirtualKeyCode::Z => KeyCode::Z,
        VirtualKeyCode::Escape => KeyCode::Escape,
        VirtualKeyCode::F1 => KeyCode::F1,
        VirtualKeyCode::F2 => KeyCode::F2,
        VirtualKeyCode::F3 => KeyCode::F3,
        VirtualKeyCode::F4 => KeyCode::F4,
        VirtualKeyCode::F5 => KeyCode::F5,
        VirtualKeyCode::F6 => KeyCode::F6,
        VirtualKeyCode::F7 => KeyCode::F7,
        VirtualKeyCode::F8 => KeyCode::F8,
        VirtualKeyCode::F9 => KeyCode::F9,
        VirtualKeyCode::F10 => KeyCode::F10,
        VirtualKeyCode::F11 => KeyCode::F11,
        VirtualKeyCode::F12 => KeyCode::F12,
        VirtualKeyCode::F13 => KeyCode::F13,
        VirtualKeyCode::F14 => KeyCode::F14,
        VirtualKeyCode::F15 => KeyCode::F15,
        VirtualKeyCode::F16 => KeyCode::F16,
        VirtualKeyCode::F17 => KeyCode::F17,
        VirtualKeyCode::F18 => KeyCode::F18,
        VirtualKeyCode::F19 => KeyCode::F19,
        VirtualKeyCode::F20 => KeyCode::F20,
        VirtualKeyCode::F21 => KeyCode::F21,
        VirtualKeyCode::F22 => KeyCode::F22,
        VirtualKeyCode::F23 => KeyCode::F23,
        VirtualKeyCode::F24 => KeyCode::F24,
        VirtualKeyCode::Snapshot => KeyCode::Snapshot,
        VirtualKeyCode::Scroll => KeyCode::Scroll,
        VirtualKeyCode::Pause => KeyCode::Pause,
        VirtualKeyCode::Insert => KeyCode::Insert,
        VirtualKeyCode::Home => KeyCode::Home,
        VirtualKeyCode::Delete => KeyCode::Delete,
        VirtualKeyCode::End => KeyCode::End,
        VirtualKeyCode::PageDown => KeyCode::PageDown,
        VirtualKeyCode::PageUp => KeyCode::PageUp,
        VirtualKeyCode::Left => KeyCode::Left,
        VirtualKeyCode::Up => KeyCode::Up,
        VirtualKeyCode::Right => KeyCode::Right,
        VirtualKeyCode::Down => KeyCode::Down,
        VirtualKeyCode::Back => KeyCode::Back,
        VirtualKeyCode::Return => KeyCode::Return,
        VirtualKeyCode::Space => KeyCode::Space,
        VirtualKeyCode::Compose => KeyCode::Compose,
        VirtualKeyCode::Caret => KeyCode::Caret,
        VirtualKeyCode::Numlock => KeyCode::Numlock,
        VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
        VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
        VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
        VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
        VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
        VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
        VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
        VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
        VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
        VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
        VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
        VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
        VirtualKeyCode::NumpadAdd => KeyCode::NumpadAdd,
        VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
        VirtualKeyCode::Apps => KeyCode::Apps,
        VirtualKeyCode::Asterisk => KeyCode::Asterisk,
        VirtualKeyCode::Plus => KeyCode::Plus,
        VirtualKeyCode::At => KeyCode::At,
        VirtualKeyCode::Ax => KeyCode::Ax,
        VirtualKeyCode::Backslash => KeyCode::Backslash,
        VirtualKeyCode::Calculator => KeyCode::Calculator,
        VirtualKeyCode::Capital => KeyCode::Capital,
        VirtualKeyCode::Colon => KeyCode::Colon,
        VirtualKeyCode::Comma => KeyCode::Comma,
        VirtualKeyCode::Convert => KeyCode::Convert,
        VirtualKeyCode::NumpadDecimal => KeyCode::NumpadDecimal,
        VirtualKeyCode::NumpadDivide => KeyCode::NumpadDivide,
        VirtualKeyCode::Equals => KeyCode::Equals,
        VirtualKeyCode::Grave => KeyCode::Grave,
        VirtualKeyCode::Kana => KeyCode::Kana,
        VirtualKeyCode::Kanji => KeyCode::Kanji,
        VirtualKeyCode::LAlt => KeyCode::LAlt,
        VirtualKeyCode::LBracket => KeyCode::LBracket,
        VirtualKeyCode::LControl => KeyCode::LControl,
        VirtualKeyCode::LShift => KeyCode::LShift,
        VirtualKeyCode::LWin => KeyCode::LWin,
        VirtualKeyCode::Mail => KeyCode::Mail,
        VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
        VirtualKeyCode::MediaStop => KeyCode::MediaStop,
        VirtualKeyCode::Minus => KeyCode::Minus,
        VirtualKeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
        VirtualKeyCode::Mute => KeyCode::Mute,
        VirtualKeyCode::MyComputer => KeyCode::MyComputer,
        VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
        VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
        VirtualKeyCode::NextTrack => KeyCode::NextTrack,
        VirtualKeyCode::NoConvert => KeyCode::NoConvert,
        VirtualKeyCode::NumpadComma => KeyCode::NumpadComma,
        VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
        VirtualKeyCode::NumpadEquals => KeyCode::NumpadEquals,
        VirtualKeyCode::Period => KeyCode::Period,
        VirtualKeyCode::PlayPause => KeyCode::PlayPause,
        VirtualKeyCode::Power => KeyCode::Power,
        VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
        VirtualKeyCode::RAlt => KeyCode::RAlt,
        VirtualKeyCode::RBracket => KeyCode::RBracket,
        VirtualKeyCode::RControl => KeyCode::RControl,
        VirtualKeyCode::RShift => KeyCode::RShift,
        VirtualKeyCode::RWin => KeyCode::RWin,
        VirtualKeyCode::Semicolon => KeyCode::Semicolon,
        VirtualKeyCode::Slash => KeyCode::Slash,
        VirtualKeyCode::Sleep => KeyCode::Sleep,
        VirtualKeyCode::Stop => KeyCode::Stop,
        VirtualKeyCode::NumpadSubtract => KeyCode::NumpadSubtract,
        VirtualKeyCode::Sysrq => KeyCode::Sysrq,
        VirtualKeyCode::Tab => KeyCode::Tab,
        VirtualKeyCode::Underline => KeyCode::Underline,
        VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
        VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
        VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
        VirtualKeyCode::Wake => KeyCode::Wake,
        VirtualKeyCode::WebBack => KeyCode::WebBack,
        VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
        VirtualKeyCode::WebForward => KeyCode::WebForward,
        VirtualKeyCode::WebHome => KeyCode::WebHome,
        VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
        VirtualKeyCode::WebSearch => KeyCode::WebSearch,
        VirtualKeyCode::WebStop => KeyCode::WebStop,
        VirtualKeyCode::Yen => KeyCode::Yen,
        VirtualKeyCode::Copy => KeyCode::Copy,
        VirtualKeyCode::Paste => KeyCode::Paste,
        VirtualKeyCode::Cut => KeyCode::Cut,
        VirtualKeyCode::OEM102 => KeyCode::OEM102,
    }
}
