use rrr_input::KeyCode;
use rrr_types::Direction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod prelude {
    pub use rrr_input::KeyCode;
    pub use rrr_types::Direction;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreSettings {
    pub judge_offset: i8,
    pub key_to_direction_map: HashMap<KeyCode, Direction>,
}

impl Default for CoreSettings {
    fn default() -> Self {
        Self {
            judge_offset: 0,
            key_to_direction_map: HashMap::from([
                (KeyCode::Left, Direction::Left),
                (KeyCode::Down, Direction::Down),
                (KeyCode::Up, Direction::Up),
                (KeyCode::Right, Direction::Right),
            ]),
        }
    }
}
