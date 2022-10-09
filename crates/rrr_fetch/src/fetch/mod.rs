mod imp;

use self::playlist::{Song, Stat};
use rrr_types::SongID;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod playlist;
pub use imp::platform::*;

// type BytesFetch = Result<std::option::Option<bytes::Bytes>>;
#[derive(Serialize, Deserialize)]
pub enum BytesFetch {
    Ok(Vec<u8>),
    Wait,
    Err(String),
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

pub fn download_chart(song_id: SongID) -> Fetcher {
    Fetcher::new(song_id)
}
