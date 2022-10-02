use rrr_settings_core::CoreSettings;
use std::convert::From;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

impl From<Settings> for CoreSettings {
    fn from(item: Settings) -> Self {
        item.core
    }
}

/// Stores RRR settings to start charts with.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    pub core: CoreSettings,
    pub note_offset: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let _settings = Settings::default();
    }

    #[test]
    fn from_settings_to_core() {
        let settings = Settings {
            core: CoreSettings { judge_offset: 100 },
            note_offset: -100,
        };

        let core_settings: CoreSettings = settings.into();
        assert_eq!(settings.core, core_settings);
    }
}
