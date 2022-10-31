use crate::{Headless, Rendered, RustRustRevolution};
use rrr_core::{prelude::CoreSettings, Active, Play, Ready};
use rrr_noteskin::Noteskin;
use rrr_render::Renderer;
use rrr_settings::Settings;
use rrr_time::TimeTrait;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Loaded {}

#[derive(Debug)]
pub struct Unloaded {}

#[derive(Debug)]
pub struct BuildRendered {
    noteskin: Noteskin,
    renderer: Renderer,
    settings: Settings,
}

#[derive(Debug)]
pub struct BuildHeadless {
    settings: CoreSettings,
}

pub trait BuilderMode {}
impl BuilderMode for BuildRendered {}
impl BuilderMode for BuildHeadless {}
impl BuilderMode for Loaded {}
impl BuilderMode for Unloaded {}

#[derive(Debug)]
pub struct RustRustRevolutionBuilder<S: BuilderMode, T: TimeTrait> {
    inner: S,
    play: Play<Ready>,
    marker: std::marker::PhantomData<T>,
}

impl<T: TimeTrait> RustRustRevolutionBuilder<Unloaded, T> {
    #[must_use]
    pub fn with_play(play: Play<Ready>) -> RustRustRevolutionBuilder<Loaded, T> {
        RustRustRevolutionBuilder {
            inner: Loaded {},
            marker: std::marker::PhantomData,
            play,
        }
    }
}

impl<T: TimeTrait> RustRustRevolutionBuilder<Loaded, T> {
    #[must_use]
    pub fn without_renderer(self) -> RustRustRevolutionBuilder<BuildHeadless, T> {
        RustRustRevolutionBuilder {
            inner: BuildHeadless {
                settings: CoreSettings::default(),
            },
            marker: self.marker,
            play: self.play,
        }
    }

    #[must_use]
    pub fn with_renderer(self, renderer: Renderer) -> RustRustRevolutionBuilder<BuildRendered, T> {
        RustRustRevolutionBuilder {
            inner: BuildRendered {
                noteskin: Noteskin::default(),
                renderer,
                settings: Settings::default(),
            },
            marker: self.marker,
            play: self.play,
        }
    }
}

impl<T: TimeTrait> RustRustRevolutionBuilder<BuildHeadless, T> {
    #[must_use]
    pub fn with_settings(self, settings: CoreSettings) -> Self {
        RustRustRevolutionBuilder {
            inner: BuildHeadless { settings },
            marker: self.marker,
            play: self.play,
        }
    }

    #[must_use]
    pub fn build(self, play_state: Play<Active>) -> RustRustRevolution<Headless, T> {
        RustRustRevolution {
            state: Headless {
                settings: self.inner.settings,
            },
            actions: VecDeque::with_capacity(usize::from(u8::MAX)),
            start_instant: T::now(),
            previous_instant: T::now(),
            current_instant: T::now(),
            play_state,
            delta: 0.,
        }
    }
}

impl<T: TimeTrait> RustRustRevolutionBuilder<BuildRendered, T> {
    #[must_use]
    pub fn with_settings(self, settings: Settings) -> Self {
        RustRustRevolutionBuilder {
            inner: BuildRendered {
                noteskin: self.inner.noteskin,
                renderer: self.inner.renderer,
                settings,
            },
            marker: self.marker,
            play: self.play,
        }
    }

    #[must_use]
    pub fn with_noteskin(self, noteskin: Noteskin) -> Self {
        RustRustRevolutionBuilder {
            inner: BuildRendered {
                noteskin,
                renderer: self.inner.renderer,
                settings: self.inner.settings,
            },
            marker: self.marker,
            play: self.play,
        }
    }

    #[must_use]
    pub fn build(self) -> RustRustRevolution<Rendered, T> {
        RustRustRevolution {
            state: Rendered {
                noteskin: self.inner.noteskin,
                renderer: self.inner.renderer,
                settings: self.inner.settings,
            },
            actions: VecDeque::with_capacity(usize::from(u8::MAX)),
            start_instant: T::now(),
            previous_instant: T::now(),
            current_instant: T::now(),
            play_state: self.play.start_with_audio(),
            delta: 0.,
        }
    }
}
