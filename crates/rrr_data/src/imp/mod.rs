use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
mod platform {
    mod wasm;
    pub use self::wasm::*;
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    mod native;
    pub use self::native::*;
}

pub fn write<'a, T>(id: Uuid, data: T) -> bool
where
    T: Serialize + Deserialize<'a>,
{
    platform::write(id, data)
}

pub fn read<'a, T>(id: Uuid, data: T) -> bool
where
    T: Serialize + Deserialize<'a>,
{
    platform::read(id, data)
}
