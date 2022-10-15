mod imp;

use rrr_playlist::{Song, Stat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum FetchProgress {
    Fetching(f32),
    Finished,
    Error(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ManifestPayload {
    pub artists: Vec<Artist>,
    pub charts: Vec<Chart>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistPayload {
    pub songs: Vec<Song>,
    pub stats: HashMap<String, Stat>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub userid: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Chart {
    pub arrows: i32,
    pub artist: i32,
    pub author: i32,
    pub difficulty: i32,
    pub genre: i32,
    pub level: i32,
    pub name: String,
    pub releasedate: String,
    pub style: String,
    pub time: String,
}

#[cfg(target_arch = "wasm32")]
pub mod platform {
    pub use super::imp::platform::*;
}

#[cfg(not(target_arch = "wasm32"))]
pub mod platform {
    pub use super::imp::platform::*;
}
