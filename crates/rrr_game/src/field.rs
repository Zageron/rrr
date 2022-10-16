use rrr_settings::Settings;
use rrr_types::{ReceptorPosition, ScrollDirection};

#[derive(Debug, Default)]
pub struct Field {
    pub start_position: f32,
    pub judge_position: f32,
}

pub fn get_note_start_position(settings: &Settings, field_height: u32, note_height: u32) -> i32 {
    match settings.scroll_direction {
        ScrollDirection::Down => 0i32.saturating_sub_unsigned(note_height),
        ScrollDirection::Up => 0i32.saturating_add_unsigned(field_height),
    }
}

pub fn get_receptor_position(
    settings: &Settings,
    field_height: u32,
    note_height: u32,
) -> ReceptorPosition {
    match settings.scroll_direction {
        ScrollDirection::Up => settings.receptor_position,
        ScrollDirection::Down => field_height - settings.receptor_position - note_height,
    }
}
