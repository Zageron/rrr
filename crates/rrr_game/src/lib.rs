use rrr_core::prelude::CoreSettings;
use rrr_render::Renderer;
use rrr_settings::Settings;
use std::collections::VecDeque;

mod builder;
pub use builder::RustRustRevolutionBuilder;

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
    _renderer: Renderer,
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
