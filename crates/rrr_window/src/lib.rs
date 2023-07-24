use anyhow::{anyhow, Result};
use rrr_config::Config;
use rrr_game::{hit_action, Rendered, RustRustRevolution};
use rrr_input::KeyCode;
use rrr_time::Time;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{TouchPhase, VirtualKeyCode},
    event_loop::EventLoop,
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

pub mod prelude {
    pub use winit::{
        event_loop::{EventLoop, EventLoopBuilder},
        platform::run_return::EventLoopExtRunReturn,
    };
}

pub struct Window<'e> {
    event_loop: &'e mut EventLoop<()>,
    pub window: winit::window::Window,
}

impl<'e> Window<'e> {
    pub fn new(config: Config, event_loop: &'e mut EventLoop<()>) -> Result<Window> {
        let size = PhysicalSize::new(config.width, config.height);
        let window = match WindowBuilder::new()
            .with_title("Rust Rust Revolution")
            .with_inner_size(size)
            .with_resizable(false)
            .with_position(PhysicalPosition::new(config.window_x, config.window_y))
            .build(event_loop)
        {
            Ok(window) => window,
            Err(e) => return Err(anyhow!("Unable to create window: {:?}", e)),
        };

        Ok(Self { event_loop, window })
    }

    pub fn run_once(&mut self, rrr: &mut RustRustRevolution<Rendered, Time>) {
        self.window.focus_window();

        let window = &self.window;
        self.event_loop.run_return(move |event, _, control_flow| {
            control_flow.set_poll();

            #[allow(deprecated)]
            match event {
                winit::event::Event::NewEvents(_) => {}

                winit::event::Event::DeviceEvent {
                    device_id: _,
                    event: _,
                } => {}

                winit::event::Event::UserEvent(_) => {}

                winit::event::Event::Suspended => {}

                winit::event::Event::Resumed => {}

                winit::event::Event::MainEventsCleared => {
                    rrr.update();
                    let _ = rrr.draw();
                    window.set_inner_size(PhysicalSize {
                        width: rrr.width(),
                        height: rrr.height(),
                    }); // Is this needed?
                    window.request_redraw();
                }

                winit::event::Event::RedrawRequested(_) => {}

                winit::event::Event::RedrawEventsCleared => {
                    rrr.finish();
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
                                rrr.hit(hit_action::Builder::with_key_code(
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
                    winit::event::WindowEvent::TouchpadMagnify {
                        device_id: _,
                        delta: _,
                        phase: _,
                    } => {}
                    winit::event::WindowEvent::SmartMagnify { device_id: _ } => {}
                    winit::event::WindowEvent::TouchpadRotate {
                        device_id: _,
                        delta: _,
                        phase: _,
                    } => {}
                },
            }
        });
    }
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
