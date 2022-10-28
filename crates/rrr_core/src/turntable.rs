use anyhow::Result;
use btreemultimap::MultiRange;
use rrr_audio::AudioPlayer;
use rrr_chart::RuntimeNote;
use rrr_record::record::Record;
use std::{borrow::BorrowMut, ops::Bound::Included};

#[derive(Debug)]
pub struct Turntable<S: TurntableState> {
    record: Record,
    state: S,
}

#[derive(Debug)]
pub struct Empty {}

#[derive(Debug)]
pub struct Loaded {}

#[derive(Debug)]
pub struct Playing {
    pub progress: u32,
    audio_player: Option<AudioPlayer>,
}

pub trait TurntableState {}
impl TurntableState for Empty {}
impl TurntableState for Loaded {}
impl TurntableState for Playing {}

impl Turntable<Empty> {
    #[must_use]
    pub fn load(record: Record) -> Turntable<Loaded> {
        Turntable {
            record,
            state: Loaded {},
        }
    }
}

impl Turntable<Loaded> {
    /// Start playing the record on the turntable.
    #[must_use]
    pub fn play(self) -> Turntable<Playing> {
        Turntable {
            record: self.record,
            state: Playing {
                progress: 0,
                audio_player: None,
            },
        }
    }

    /// Start playing the record on the turntable.
    ///
    /// # Panics
    ///
    /// If the mp3 data is malformed.
    #[must_use]
    pub fn play_with_audio(self) -> Turntable<Playing> {
        let mut turntable = Turntable {
            record: self.record,
            state: Playing {
                progress: 0,
                audio_player: None,
            },
        };

        if let Ok(audio_player) = AudioPlayer::try_new(turntable.record.mp3.as_slice()) {
            turntable.state.audio_player.replace(audio_player);
        }

        turntable
    }
}

impl Turntable<Playing> {
    #[must_use]
    pub fn stop(self) -> Turntable<Loaded> {
        if let Some(mut player) = self.state.audio_player {
            player.stop();
        }

        Turntable {
            record: self.record,
            state: Loaded {},
        }
    }

    pub fn tick(&mut self, progress: u32) {
        self.state.progress = progress;

        #[allow(clippy::pattern_type_mismatch)]
        if let Some(player) = self.state.audio_player.borrow_mut() {
            player.tick();
        }
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.state.progress >= self.record.duration
    }

    #[must_use]
    pub fn progress(&self) -> u32 {
        self.state.progress
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if it tries to view past the end of the record.
    pub fn view(
        &self,
        look_behind: u32,
        look_ahead: u32,
    ) -> Result<MultiRange<'_, u32, RuntimeNote>> {
        let chart = &self.record.optimized_chart;

        let first = if let Some(first_value) = self.state.progress.checked_sub(look_behind) {
            first_value
        } else {
            self.state.progress
        };

        if let Some(last) = self.state.progress.checked_add(look_ahead) {
            return Ok(chart.range((Included(first), Included(last))));
        }

        Err(anyhow::anyhow!("Tried to look past the end of the chart."))
    }
}
