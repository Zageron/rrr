mod builder;
pub mod hit_action;

pub use builder::RustRustRevolutionBuilder;
use rrr_core::prelude::CoreSettings;
use rrr_render::Renderer;
use rrr_settings::Settings;
use rrr_types::SongID;
use std::collections::VecDeque;

pub mod prelude {
    pub use rrr_render;
    pub use rrr_types::SongID;
}

#[derive(Debug)]
pub struct RustRustRevolution<S: Mode> {
    _inner: S,
    actions: VecDeque<hit_action::Action>,
    _active_song_id: SongID,
}

#[derive(Debug)]
pub struct Rendered {
    _renderer: Renderer,
    _settings: Settings,
}

#[derive(Debug)]
pub struct Headless {
    _settings: CoreSettings,
}

pub trait Mode {}

impl Mode for Rendered {}
impl Mode for Headless {}

impl<S: Mode> RustRustRevolution<S> {
    pub fn hit(&mut self, action_builder: hit_action::Builder<hit_action::NeedsTimestamp>) {
        self.actions
            .push_back(action_builder.with_timestamp(u32::MAX));
    }
}
