use rrr_core::prelude::CoreSettings;
use rrr_render::Renderer;
use rrr_settings::Settings;
use std::collections::VecDeque;

pub mod prelude {
    pub use rrr_render;
}

pub mod key_hit {
    use rrr_input::KeyCode;

    pub struct Action {
        _key: KeyCode,
        _ts: u32,
    }

    pub struct NeedsKeyCode();
    pub struct NeedsTimestamp(KeyCode);

    pub trait BuilderMode {}
    impl BuilderMode for NeedsKeyCode {}
    impl BuilderMode for NeedsTimestamp {}

    #[derive(Debug, Default)]
    pub struct Builder<S: BuilderMode> {
        inner: S,
    }

    impl Builder<NeedsKeyCode> {
        #[must_use]
        pub fn with_key_code(key_code: KeyCode) -> Builder<NeedsTimestamp> {
            Builder {
                inner: NeedsTimestamp(key_code),
            }
        }
    }

    impl Builder<NeedsTimestamp> {
        #[must_use]
        pub fn with_timestamp(self, ts: u32) -> Action {
            Action {
                _key: self.inner.0,
                _ts: ts,
            }
        }
    }
}

pub struct SongID(pub u16);

pub struct RustRustRevolution<S: Mode> {
    _inner: S,
    actions: VecDeque<key_hit::Action>,
    _active_song_id: SongID,
}

pub struct Rendered {
    renderer: Renderer,
    _settings: Settings,
}

pub struct Headless {
    _settings: CoreSettings,
}

pub trait Mode {}
impl Mode for Rendered {}
impl Mode for Headless {}

impl<S: Mode> RustRustRevolution<S> {
    pub fn hit(&mut self, action_builder: key_hit::Builder<key_hit::NeedsTimestamp>) {
        self.actions
            .push_back(action_builder.with_timestamp(u32::MAX));
    }
}

pub struct Unknown {}

pub trait BuilderMode {}
impl BuilderMode for Rendered {}
impl BuilderMode for Headless {}
impl BuilderMode for Unknown {}

#[derive(Debug, Default)]
pub struct RustRustRevolutionBuilder<S: BuilderMode> {
    inner: S,
}

impl RustRustRevolutionBuilder<Unknown> {
    #[must_use]
    pub fn without_renderer() -> RustRustRevolutionBuilder<Headless> {
        RustRustRevolutionBuilder {
            inner: Headless {
                _settings: CoreSettings::default(),
            },
        }
    }

    #[must_use]
    pub fn with_renderer(renderer: Renderer) -> RustRustRevolutionBuilder<Rendered> {
        RustRustRevolutionBuilder {
            inner: Rendered {
                renderer,
                _settings: Settings::default(),
            },
        }
    }
}

impl RustRustRevolutionBuilder<Headless> {
    #[must_use]
    pub fn with_settings(self, settings: CoreSettings) -> Self {
        RustRustRevolutionBuilder {
            inner: Headless {
                _settings: settings,
            },
        }
    }

    #[must_use]
    pub fn build(self, active_song_id: SongID) -> RustRustRevolution<Headless> {
        RustRustRevolution {
            _inner: self.inner,
            actions: VecDeque::with_capacity(usize::from(u8::MAX)),
            _active_song_id: active_song_id,
        }
    }
}

impl RustRustRevolutionBuilder<Rendered> {
    #[must_use]
    pub fn with_settings(self, settings: Settings) -> Self {
        RustRustRevolutionBuilder {
            inner: Rendered {
                renderer: self.inner.renderer,
                _settings: settings,
            },
        }
    }

    #[must_use]
    pub fn build(self, active_song_id: SongID) -> RustRustRevolution<Rendered> {
        RustRustRevolution {
            _inner: self.inner,
            actions: VecDeque::with_capacity(usize::from(u8::MAX)),
            _active_song_id: active_song_id,
        }
    }
}
