#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoreSettings {
    pub judge_offset: i32,
}

impl Default for CoreSettings {
    #[must_use]
    fn default() -> Self {
        Self { judge_offset: 0 }
    }
}
