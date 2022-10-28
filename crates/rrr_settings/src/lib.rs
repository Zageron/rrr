use rrr_settings_core::CoreSettings;
use rrr_types::ReceptorPosition;
use rrr_types::ScrollDirection;
use serde::{Deserialize, Serialize};
use std::convert::From;

impl From<Settings> for CoreSettings {
    fn from(item: Settings) -> Self {
        item.core
    }
}

/// Stores RRR settings to start charts with.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub core: CoreSettings,
    pub gap: u8,
    pub note_offset: i32,
    pub scroll_speed: u32,
    pub scroll_direction: ScrollDirection,
    pub receptor_position: ReceptorPosition,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            core: Default::default(),
            gap: 8,
            note_offset: -100,
            scroll_speed: 1500,
            scroll_direction: ScrollDirection::default(),
            receptor_position: ReceptorPosition::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rrr_types::ReceptorPosition;

    use super::*;

    #[test]
    fn default_settings() {
        let _settings = Settings::default();
    }

    #[test]
    fn from_settings_to_core() {
        use rrr_settings_core::prelude::*;
        let settings = Settings {
            core: CoreSettings {
                judge_offset: 100,
                key_to_direction_map: HashMap::from([
                    (KeyCode::Left, Direction::Left),
                    (KeyCode::Down, Direction::Down),
                    (KeyCode::Up, Direction::Up),
                    (KeyCode::Right, Direction::Right),
                ]),
            },
            gap: 8,
            note_offset: -100,
            scroll_speed: 1500,
            scroll_direction: ScrollDirection::default(),
            receptor_position: ReceptorPosition::default(),
        };

        let core_settings: CoreSettings = settings.clone().into();
        assert_eq!(settings.core, core_settings);
    }
}
