#![deny(rust_2018_idioms)]
#![warn(
    elided_lifetimes_in_paths,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    variant_size_differences,
    clippy::all,
    clippy::cargo,
    clippy::pedantic,
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::pattern_type_mismatch,
    clippy::print_stdout,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::shadow_reuse,
    clippy::todo,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads
)]
#![allow(clippy::module_name_repetitions, clippy::multiple_crate_versions)]
#![forbid(unsafe_code)]

pub mod actions;
pub mod field;
pub mod judge;
pub mod record;
pub mod turntable;

use self::{
    actions::NoteAction,
    field::Field,
    judge::{Judge, JudgeWindow, Judgement},
    turntable::Turntable,
};
use anyhow::Result;
use btreemultimap::{BTreeMultiMap, MultiRange};
use rrr_chart::{NoteDirection, RuntimeNote};
use rrr_settings_core::CoreSettings;
use std::{borrow::BorrowMut, collections::HashSet};

#[derive(Debug, Default, Clone)]
pub struct JudgementReport {
    pub amazings: usize,
    pub perfects: usize,
    pub goods: usize,
    pub averages: usize,
    pub misses: usize,
    pub boos: usize,
}

pub struct Play<S: PlayState> {
    field: Field,
    state: S,
    settings: CoreSettings,
}

impl<S: PlayState> Play<S> {
    pub fn settings(&self) -> &CoreSettings {
        &self.settings
    }
}

pub struct Ready {
    turntable: Turntable<turntable::Loaded>,
}

pub struct Active {
    turntable: Turntable<turntable::Playing>,
    actions: BTreeMultiMap<RuntimeNote, NoteAction>,
    judge: Judge,
    misses: HashSet<RuntimeNote>,
    judgement_report: JudgementReport,
}

pub struct Concluded {
    turntable: Turntable<turntable::Loaded>,
    actions: BTreeMultiMap<RuntimeNote, NoteAction>,
    _judgement_report: JudgementReport,
}

pub trait PlayState {}
impl PlayState for Ready {}
impl PlayState for Active {}
impl PlayState for Concluded {}

impl Play<Ready> {
    #[must_use]
    pub fn new(turntable: Turntable<turntable::Loaded>, field: Field) -> Self {
        Self {
            state: Ready { turntable },
            settings: CoreSettings::default(),
            field,
        }
    }

    #[must_use]
    pub fn with_settings(self, settings: CoreSettings) -> Self {
        Self {
            field: self.field,
            state: self.state,
            settings,
        }
    }

    #[must_use]
    pub fn with_field(self, field: Field) -> Self {
        Self {
            field,
            state: self.state,
            settings: self.settings,
        }
    }

    #[must_use]
    pub fn start_with_audio(self) -> Play<Active> {
        Play {
            field: self.field,
            state: Active {
                turntable: self.state.turntable.play_with_audio(),
                actions: BTreeMultiMap::default(),
                judge: Judge::new(),
                misses: HashSet::<RuntimeNote>::new(),
                judgement_report: JudgementReport::default(),
            },
            settings: self.settings,
        }
    }

    #[must_use]
    pub fn start(self) -> Play<Active> {
        Play {
            field: self.field,
            state: Active {
                turntable: self.state.turntable.play(),
                actions: BTreeMultiMap::default(),
                judge: Judge::new(),
                misses: HashSet::<RuntimeNote>::new(),
                judgement_report: JudgementReport::default(),
            },
            settings: self.settings,
        }
    }
}

impl Play<Active> {
    #[must_use]
    pub fn stop(self) -> Play<Ready> {
        Play {
            field: self.field,
            state: Ready {
                turntable: self.state.turntable.stop(),
            },
            settings: self.settings,
        }
    }

    #[must_use]
    pub fn finish(self) -> Play<Concluded> {
        Play {
            field: self.field,
            state: Concluded {
                turntable: self.state.turntable.stop(),
                actions: self.state.actions,
                _judgement_report: self.state.judgement_report,
            },
            settings: self.settings,
        }
    }

    /// Temporary function giving a view directly into the playing turntable.
    ///
    /// Remove this after we create the `ChartDriver`.
    /// # Errors
    /// Turntable could slice into an invalid set of notes.
    pub fn view(
        &self,
        look_behind: u32,
        look_ahead: u32,
    ) -> Result<MultiRange<'_, u32, RuntimeNote>> {
        self.state.turntable.view(look_behind, look_ahead)
    }

    #[must_use]
    pub fn progress(&self) -> u32 {
        self.state.turntable.progress()
    }

    #[must_use]
    pub fn missed_notes(&self) -> &HashSet<RuntimeNote> {
        &self.state.misses
    }

    #[must_use]
    pub fn actions(&self) -> &BTreeMultiMap<RuntimeNote, NoteAction> {
        &self.state.actions
    }

    pub fn tick(&mut self, progress: u32) {
        self.state.turntable.tick(progress);
        self.check_miss();
    }

    fn check_miss(&mut self) {
        const MISS_OFFSET: u32 = 250;

        let song_progress = self.progress();

        if let Ok(view) = self.state.turntable.view(MISS_OFFSET.saturating_mul(2), 0) {
            #[allow(clippy::pattern_type_mismatch)]
            let mapped_notes = view
                .filter(|(&ts, note)| {
                    song_progress >= ts.saturating_add(MISS_OFFSET)
                        && !self.state.misses.contains(note)
                        && !self.state.judge.judgements.contains_key(note)
                })
                .map(|(_, note)| note.clone());

            let misses = mapped_notes.collect::<HashSet<RuntimeNote>>();
            self.state.judgement_report.misses = self
                .state
                .judgement_report
                .misses
                .saturating_add(misses.len());

            self.state.misses.extend(misses);
        }
    }

    #[must_use]
    pub fn field(&self) -> &Field {
        &self.field
    }

    #[must_use]
    pub fn judgement_results(&self) -> &JudgementReport {
        &self.state.judgement_report
    }

    #[must_use]
    pub fn judgements(&self) -> &Judgement {
        &self.state.judge.judgements
    }

    pub fn do_action(&mut self, direction: &NoteDirection, ts: u32, offset: i8) {
        if let Ok(mut view_result) = self.state.turntable.view(
            120_u32.saturating_add(u32::from(offset.unsigned_abs())),
            120_u32.saturating_add(u32::from(offset.unsigned_abs())),
        ) {
            #[allow(clippy::pattern_type_mismatch)]
            if let Some((_, closest_note)) =
                view_result.find(|(_, note)| self.determine_judgable(note, direction))
            {
                if let Ok(judgement_result) = self.state.judge.judge(ts, closest_note) {
                    if let Some(judgement) = judgement_result {
                        self.append_to_judgement_report(judgement);
                    } else {
                        self.state.judgement_report.boos =
                            self.state.judgement_report.boos.saturating_add(1);
                    }
                }
            } else {
                self.state.judgement_report.boos =
                    self.state.judgement_report.boos.saturating_add(1);
            }
        }
    }

    fn determine_judgable(&self, note: &RuntimeNote, direction: &NoteDirection) -> bool {
        let is_judged = self.state.actions.contains_key(note);
        let is_same_direction = *direction == note.direction;
        !is_judged && is_same_direction
    }

    fn append_to_judgement_report(&mut self, judgement: JudgeWindow) {
        let report = self.state.judgement_report.borrow_mut();
        match judgement {
            JudgeWindow(-118) => report.averages = report.averages.saturating_add(1),
            JudgeWindow(-85) => report.goods = report.goods.saturating_add(1),
            JudgeWindow(-51) => report.perfects = report.perfects.saturating_add(1),
            JudgeWindow(-18) => report.amazings = report.amazings.saturating_add(1),
            JudgeWindow(17) => report.perfects = report.perfects.saturating_add(1),
            JudgeWindow(50 | 84) => report.goods = report.goods.saturating_add(1),
            JudgeWindow(117) => report.averages = report.averages.saturating_add(1),
            _ => (),
        }
    }
}

impl Play<Concluded> {
    #[must_use]
    pub fn actions(&self) -> &BTreeMultiMap<RuntimeNote, NoteAction> {
        &self.state.actions
    }

    #[must_use]
    pub fn finalize(self) -> Play<Ready> {
        Play {
            field: self.field,
            state: Ready {
                turntable: self.state.turntable,
            },
            settings: self.settings,
        }
    }
}

pub trait Difference {
    #[must_use]
    fn abs_diff(self, right: &u32) -> Self;

    #[must_use]
    fn diff(self, right: &u32) -> u32;
}

impl Difference for u32 {
    fn abs_diff(self, right: &u32) -> u32 {
        if self < *right {
            (*right).saturating_sub(self)
        } else {
            self.saturating_sub(*right)
        }
    }

    fn diff(self, right: &u32) -> u32 {
        self.saturating_sub(*right)
    }
}

// tests
#[cfg(test)]
mod tests {
    use rrr_math::lerp::Lerp;

    fn screen_pos_to_lerp_time() -> f64 {
        (-64.).inv_lerp(720., 64.)
    }

    fn lerp_time_to_screen_pos() -> f64 {
        (-64.).lerp(720., screen_pos_to_lerp_time()).round()
    }

    fn ms_time_from_screen_height_time_on_screen_and_position() -> f64 {
        let start_position = 720.;
        let end_position = -64.;
        let time_on_screen = 3000.;
        let judge_position = 64.;

        let normalized_note_progress = end_position.inv_lerp(start_position, judge_position);
        let ms: f64 = normalized_note_progress * time_on_screen;

        log::info!("normalized: {}", ms);
        ms.round()
    }

    #[test]
    fn test_screen_space_to_judgement_zero() {
        assert!(screen_pos_to_lerp_time() - 0.163_265_306_122_448_97 <= f64::EPSILON);
    }

    #[test]
    fn test_screen_lerp_time_to_screen_space() {
        assert!((lerp_time_to_screen_pos() - 64.).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ms_time_from_screen_height_time_on_screen_and_position() {
        assert!(
            (ms_time_from_screen_height_time_on_screen_and_position() - 490.0).abs() < f64::EPSILON
        );
    }
}

// What I should really be doing is determining exactly what ratio is between this zero point and the note.
// So if a note has a ms timestamp of 2000, and the zero point is at 2000,
// how many milliseconds is the is the receptor before that. Ex. (2000 - 120)
