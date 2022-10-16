mod receptor_position;
pub use receptor_position::ReceptorPosition;

use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub struct SongID(pub u16);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollDirection {
    #[default]
    Up,
    Down,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum Direction {
    Left = 0,
    Down = 1,
    Up = 2,
    Right = 3,
}

impl From<Direction> for u32 {
    fn from(dir: Direction) -> Self {
        if let Ok(res) = dir.try_into() {
            res
        } else {
            u32::MAX
        }
    }
}
