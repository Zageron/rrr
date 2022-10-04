mod completions;
mod play;
mod tui;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Command {
    /// Start playing a chart immediately!
    Play(play::Args),

    /// Start the user interface.
    Tui(tui::Args),

    /// Generate completions for your shell.
    Completions(completions::Args),
}

impl Command {
    pub fn run(self) -> Result<()> {
        use Command::*;
        match self {
            Play(args) => args.run(),
            Tui(args) => args.run(),
            Completions(args) => args.run(),
        }
    }
}
