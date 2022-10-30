use crate::{Headless, Rendered, RustRustRevolution};
use rrr_core::{prelude::CoreSettings, Active, Play};
use rrr_noteskin::Noteskin;
use rrr_render::Renderer;
use rrr_settings::Settings;
use rrr_time::TimeTrait;
use std::collections::VecDeque;

pub struct Unknown {}
pub struct BuildRendered {
    noteskin: Noteskin,
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
pub struct RustRustRevolutionBuilder<S: BuilderMode, T: TimeTrait> {
    inner: S,
    marker: std::marker::PhantomData<T>,
}

impl<T: TimeTrait> RustRustRevolutionBuilder<Unknown, T> {
    #[must_use]
    pub fn without_renderer() -> RustRustRevolutionBuilder<BuildHeadless, T> {
        RustRustRevolutionBuilder {
            inner: BuildHeadless {
                settings: CoreSettings::default(),
            },
            marker: std::marker::PhantomData,
        }
    }

    #[must_use]
    pub fn with_renderer(renderer: Renderer) -> RustRustRevolutionBuilder<BuildRendered, T> {
        RustRustRevolutionBuilder {
            inner: BuildRendered {
                noteskin: Noteskin::default(),
                renderer,
                settings: Settings::default(),
            },
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: TimeTrait> RustRustRevolutionBuilder<BuildHeadless, T> {
    #[must_use]
    pub fn with_settings(self, settings: CoreSettings) -> Self {
        RustRustRevolutionBuilder {
            inner: BuildHeadless { settings },
            marker: std::marker::PhantomData,
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
            marker: std::marker::PhantomData,
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
            marker: std::marker::PhantomData,
        }
    }

    #[must_use]
    pub fn build(self, play_state: Play<Active>) -> RustRustRevolution<Rendered, T> {
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
            play_state,
            delta: 0.,
        }
    }
}
