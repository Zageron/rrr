use anyhow::Result;
use clap::Parser;
use rrr_config::Config;

#[derive(Debug, Parser)]
pub struct Args {
    /// ID of song to play
    song_id: u16,
}

impl Args {
    pub fn run(&self) -> Result<()> {
        rrr_window::init(Config::default(), self.song_id);
        Ok(())
    }
}
