use serde::{Deserialize, Serialize};

use uuid::Uuid;

pub fn write<'a, T>(_id: Uuid, _data: T) -> bool
where
    T: Serialize + Deserialize<'a>,
{
    false
}

pub fn read<'a, T>(_id: Uuid, _data: T) -> bool
where
    T: Serialize + Deserialize<'a>,
{
    false
}
