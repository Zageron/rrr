#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Window configuration
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub window_x: u32,
    pub window_y: u32,
}

impl Default for Config {
    #[must_use]
    fn default() -> Self {
        Self {
            width: 768,
            height: 512,
            window_x: 0,
            window_y: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let _config = Config::default();
    }
}
