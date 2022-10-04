use std::io;

use crate::Cli;
use anyhow::Result;
use clap::{CommandFactory, Parser};

#[derive(Debug, Parser)]
pub struct Args {
    /// shell to generate completions for
    #[arg(value_enum)]
    pub shell: clap_complete::Shell,
}

impl Args {
    pub fn run(&self) -> Result<()> {
        clap_complete::generate(self.shell, &mut Cli::command(), "rrr", &mut io::stdout());
        Ok(())
    }
}
