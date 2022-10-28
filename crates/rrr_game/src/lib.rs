mod builder;
mod field;
pub mod hit_action;

use anyhow::Result;
pub use builder::RustRustRevolutionBuilder;
use rrr_core::{prelude::CoreSettings, Active, Play};
use rrr_noteskin::Noteskin;
use rrr_render::Renderer;
use rrr_settings::Settings;
use rrr_time::{Time, TimeTrait};
use std::collections::VecDeque;

pub mod prelude {
    pub use rrr_core::{
        prelude::{RuntimeChart, RuntimeNote},
        turntable::{self, Turntable},
        Active, Play,
    };
    pub use rrr_render;
    pub use rrr_types::SongID;
}

#[derive(Debug)]
pub struct RustRustRevolution<S: Mode, T: TimeTrait> {
    state: S,
    actions: VecDeque<hit_action::Action>,
    play_state: Play<Active>,
    start_instant: T,
    previous_instant: T,
    current_instant: T,
}

#[derive(Debug)]
pub struct Rendered {
    pub noteskin: Noteskin,
    pub renderer: Renderer,
    pub settings: Settings,
}

#[derive(Debug)]
pub struct Headless {
    pub settings: CoreSettings,
}

pub trait Mode {
    fn core_settings(&self) -> &CoreSettings;
}

impl Mode for Rendered {
    fn core_settings(&self) -> &CoreSettings {
        &self.settings.core
    }
}
impl Mode for Headless {
    fn core_settings(&self) -> &CoreSettings {
        &self.settings
    }
}

impl<S: Mode, T: TimeTrait> RustRustRevolution<S, T> {
    pub fn hit(&mut self, action_builder: hit_action::Builder<hit_action::NeedsTimestamp>) {
        self.actions.push_back(action_builder.build(u32::MAX));
        log::info!("HIT");
    }

    pub fn update(&mut self) {
        self.current_instant = T::now();
        let _delta = self.current_instant.sub(&self.previous_instant);
        let current_progress = (self.start_instant.ms_since() * 1000.) as u32;

        for action in self.actions.drain(..) {
            let core_settings = self.state.core_settings();
            if let Some(direction) = core_settings.key_to_direction_map.get(&action.key) {
                self.play_state.do_action(
                    direction,
                    action.ts,
                    self.state.core_settings().judge_offset,
                );
            }
        }

        self.play_state.tick(current_progress);
    }

    pub fn finish(&mut self) {
        self.previous_instant = self.current_instant;
    }
}

impl RustRustRevolution<Rendered, Time> {
    pub fn height(&self) -> u32 {
        self.state.renderer.height
    }

    pub fn width(&self) -> u32 {
        self.state.renderer.width
    }

    pub fn draw(&mut self) -> Result<()> {
        let renderer = &mut self.state.renderer;

        let chart_progress = self.play_state.progress();
        let settings = &self.state.settings;
        let noteskin = &self.state.noteskin;

        let time_on_screen = settings.scroll_speed;

        let start_position = field::get_note_start_position(
            settings,
            renderer.height,
            self.state.noteskin.note_height.try_into().unwrap(),
        );

        let receptor_position = field::get_receptor_position(
            settings,
            renderer.height,
            self.state.noteskin.note_height.try_into().unwrap(),
        );

        let view = self.play_state.view(
            time_on_screen / 2,
            (time_on_screen as i32 + settings.note_offset) as u32,
        );
        if let Ok(view) = view {
            let temp_view = view.clone();

            // Get the view from the play state.
            // Filter judged notes from the view.
            let filtered_view =
                temp_view.filter(|(_, note)| !self.play_state.judgements().contains_key(note));

            // Fill all of the necessary bits!
            renderer.render_field(
                filtered_view,
                chart_progress,
                start_position,
                receptor_position.0 as i32,
                receptor_position,
                time_on_screen,
                noteskin,
                settings.gap,
                settings.note_offset,
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::f32::EPSILON;

    use rrr_render::field;

    #[test]
    fn test_optimal_scroll_speeds() {
        // Scroll rate of 1x.
        let standard_time_on_screen = 1000u32;
        let scroll_speed_value = 1u32;
        let field_height = 512u32;
        let receptor_offset = 64u32;
        let required_travel_distance: u32 = field_height - receptor_offset;
        let note_height = 64u32;

        let pos_0 = field::get_pos_from_ms(0i64, receptor_offset as f32, field_height as f32, 1000);
        assert!((pos_0 - field_height as f32) < EPSILON);
        let pos_1 =
            field::get_pos_from_ms(166i64, receptor_offset as f32, field_height as f32, 1000);
        // 10 frames at 60fps, field height of 512, minus receptor position of 64, (448 / 60) * 10 px progress.
        assert!(((field_height as f32) - pos_1) - 74.36801 < EPSILON);
    }

    #[test]
    fn test_scroll_rates_x1() {
        let res = field::convert_time_on_screen_from_rate(1.);
        assert_eq!(res, 1000.);
    }

    #[test]
    fn test_scroll_rates_x1_25() {
        let res = field::convert_time_on_screen_from_rate(1.25);
        assert_eq!(res, 800.);
    }

    #[test]
    fn test_scroll_rates_x0_50() {
        let res = field::convert_time_on_screen_from_rate(0.5);
        assert_eq!(res, 2000.);
    }

    #[test]
    fn test_scroll_rates_x2_00() {
        let res = field::convert_time_on_screen_from_rate(2.);
        assert_eq!(res, 500.);
    }
}
