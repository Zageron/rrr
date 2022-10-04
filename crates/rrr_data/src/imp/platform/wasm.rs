use serde::{Deserialize, Serialize};

use uuid::Uuid;

pub fn write<'a, T>(id: Uuid, data: T) -> bool
where
    T: Serialize + Deserialize<'a>,
{
    false
}

pub fn read<'a, T>(id: Uuid, data: T) -> bool
where
    T: Serialize + Deserialize<'a>,
{
    false
}
