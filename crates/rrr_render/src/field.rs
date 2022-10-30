use rrr_chart::{NoteColor, RuntimeNote};
use rrr_graphics::sprites;
use rrr_math::lerp::Lerp;
use rrr_noteskin::Noteskin;
use rrr_types::Direction;

/// Standard time on screen is hard-coded to 1000 milliseconds.
/// Use this function with a user set multiplier to get actual time on screen..
/// # Examples
///
/// ```
/// use rrr_render::field::convert_time_on_screen_from_rate;
///
/// let time_on_screen = convert_time_on_screen_from_rate(0.5);
/// assert_eq!(time_on_screen, 2000.);
/// ```
pub fn convert_time_on_screen_from_rate(scroll_rate: f32) -> f32 {
    const STANDARD_TIME_ON_SCREEN: f32 = 1000.0;
    if scroll_rate != 0. {
        STANDARD_TIME_ON_SCREEN / scroll_rate
    } else {
        STANDARD_TIME_ON_SCREEN
    }
}

pub fn get_pos_from_ms(
    ms: i64,
    end_position: f32,
    start_position: f32,
    time_on_screen: u32,
) -> f32 {
    start_position.lerp(end_position, ms as f32 / time_on_screen as f32)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_notes<'a>(
    view: impl IntoIterator<Item = (&'a u32, &'a RuntimeNote)>,
    time_on_screen: u32,
    ms_chart_progress: u32,
    ms_note_render_offset: i32,
    offset: f32,
    frame: &mut [u8],
    noteskin: &Noteskin,
    lane_gap: u8,
    screen_width: u32,
    screen_height: u32,
    start_position: i32,
    end_position: i32,
) {
    let end_position = end_position as f64;

    for (&ms_when_note_at_receptor, note) in view {
        // Calculate "time_on_screen" as from off-screen to receptor, and then continue on with the lerp. (lerp can fall off)
        // Rendering should carry on past the zero point but it should arrive at 0 at the receptor point rather than the beginning of the screen.

        let note_progress = (ms_when_note_at_receptor as f32 + ms_note_render_offset as f32)
            - ms_chart_progress as f32;
        let normalized = note_progress / time_on_screen as f32;
        let position = end_position.lerp(start_position as f64, normalized.into());

        let lane_offset = noteskin.note_width.saturating_add(lane_gap as usize) as f32;

        let lane_index = match note.direction {
            Direction::Left => -1.5,
            Direction::Down => -0.5,
            Direction::Up => 0.5,
            Direction::Right => 1.5,
        };
        let x = offset + (lane_offset * lane_index);
        let y = position as f32;
        sprites::blit(
            frame,
            screen_width,
            screen_height,
            x,
            y,
            &note.direction,
            &noteskin.get_note(note.color),
        );
    }
}

pub(crate) fn draw_receptors(
    noteskin: &Noteskin,
    frame: &mut [u8],
    offset: f32,
    receptor_y: f32,
    gap: u8,
    screen_width: u32,
    screen_height: u32,
) {
    let receptor_skin = noteskin.get_note(NoteColor::Receptor);
    let lane_offset = noteskin.note_width.saturating_add(gap as usize) as f32;
    sprites::blit(
        frame,
        screen_width,
        screen_height,
        offset + (lane_offset * -1.5),
        receptor_y,
        &Direction::Left,
        &receptor_skin,
    );
    sprites::blit(
        frame,
        screen_width,
        screen_height,
        offset + (lane_offset * -0.5),
        receptor_y,
        &Direction::Down,
        &receptor_skin,
    );
    sprites::blit(
        frame,
        screen_width,
        screen_height,
        offset + (lane_offset * 0.5),
        receptor_y,
        &Direction::Up,
        &receptor_skin,
    );
    sprites::blit(
        frame,
        screen_width,
        screen_height,
        offset + (lane_offset * 1.5),
        receptor_y,
        &Direction::Right,
        &receptor_skin,
    );
}

pub(crate) fn clear(frame: &mut [u8]) {
    frame.fill(0);
}
