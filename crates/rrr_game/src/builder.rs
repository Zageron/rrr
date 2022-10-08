use crate::{Headless, Rendered, RustRustRevolution, SongID};
use rrr_core::prelude::CoreSettings;
use rrr_render::Renderer;
use rrr_settings::Settings;
use std::collections::VecDeque;

pub struct Unknown {}
pub struct BuildRendered {
    renderer: Renderer,
    settings: Settings,
}
pub struct BuildHeadless {
    settings: CoreSettings,
}

pub trait BuilderMode {}
impl BuilderMode for BuildRendered {}
impl BuilderMode for BuildHeadless {}
impl BuilderMode for Unknown {}

#[derive(Debug, Default)]
pub struct RustRustRevolutionBuilder<S: BuilderMode> {
    inner: S,
}

impl RustRustRevolutionBuilder<Unknown> {
    #[must_use]
    pub fn without_renderer() -> RustRustRevolutionBuilder<BuildHeadless> {
        RustRustRevolutionBuilder {
            inner: BuildHeadless {
                settings: CoreSettings::default(),
            },
        }
    }

    #[must_use]
    pub fn with_renderer(renderer: Renderer) -> RustRustRevolutionBuilder<BuildRendered> {
        RustRustRevolutionBuilder {
            inner: BuildRendered {
                renderer,
                settings: Settings::default(),
            },
        }
    }
}

impl RustRustRevolutionBuilder<BuildHeadless> {
    #[must_use]
    pub fn with_settings(self, settings: CoreSettings) -> Self {
        RustRustRevolutionBuilder {
            inner: BuildHeadless { settings },
        }
    }

    #[must_use]
    pub fn build(self, active_song_id: SongID) -> RustRustRevolution<Headless> {
        RustRustRevolution {
            _inner: Headless {
                _settings: self.inner.settings,
            },
            actions: VecDeque::with_capacity(usize::from(u8::MAX)),
            _active_song_id: active_song_id,
        }
    }
}

impl RustRustRevolutionBuilder<BuildRendered> {
    #[must_use]
    pub fn with_settings(self, settings: Settings) -> Self {
        RustRustRevolutionBuilder {
            inner: BuildRendered {
                renderer: self.inner.renderer,
                settings,
            },
        }
    }

    #[must_use]
    pub fn build(self, active_song_id: SongID) -> RustRustRevolution<Rendered> {
        RustRustRevolution {
            _inner: Rendered {
                _renderer: self.inner.renderer,
                _settings: self.inner.settings,
            },
            actions: VecDeque::with_capacity(usize::from(u8::MAX)),
            _active_song_id: active_song_id,
        }
    }
}
