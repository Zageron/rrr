// How is construction and ownership going to happen here.
// Do I want to rely on rrr_fetch? Probably not.

// Load playlist.
//  Cache Layer
//  Fetch Layer
//  Parse Layer (optional)
// Construct playlist struct.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Song {
    pub level: i64,
    pub genre: i64,
    pub name: String,
    pub author: String,
    pub author_url: String,
    pub stepauthor: String,
    pub difficulty: i64,
    pub style: String,
    pub time: String,
    pub order: i64,
    pub note_count: i64,
    pub nps_min: i64,
    pub nps_max: i64,
    pub data_nps: String,
    pub hash_load: String,
    pub swf_end_delay: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub total_length: i64,
    pub eff_length: i64,
    pub chord_count: i64,
    pub avg_nps: f64,
    pub first_delay: i64,
    pub last_delay: i64,
    pub note_delays: Vec<i64>,
    pub hand_bias: i64,
    pub jumps: Vec<i64>,
    pub color_jumps: Vec<i64>,
    pub framers: Vec<i64>,
    pub density: Vec<Vec<Density>>,
    pub camel_jacks: i64,
    pub color_total: ColorTotal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorTotal {
    pub red: Vec<f64>,
    pub blue: Vec<f64>,
    pub purple: Vec<f64>,
    pub yellow: Vec<f64>,
    pub pink: Vec<f64>,
    pub orange: Vec<f64>,
    pub cyan: Vec<f64>,
    pub green: Vec<f64>,
    pub white: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Density {
    Integer(i64),
    String(String),
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FFRPlaylist {
    pub songs: Vec<Song>,
    pub stats: HashMap<String, Stat>,
}

pub trait Playlist {}
impl Playlist for FFRPlaylist {}
