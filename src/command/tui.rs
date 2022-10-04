use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {}

impl Args {
    pub fn run(&self) -> Result<()> {
        rrr_tui::init()
    }
}
