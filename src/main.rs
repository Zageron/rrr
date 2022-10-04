mod command;

use anyhow::Result;
use clap::Parser;
use command::Command;

#[derive(Debug, Parser)]
#[command(name = "rrr")]
#[command(author = "Zageron <hello@zageron.ca>")]
#[command(version = "1.0")]
#[command(propagate_version = true)]
#[command(about = "Interface for interacting with RRR.", long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            None => Err(anyhow::anyhow!("Please choose a valid command.")),
            Some(command) => command.run(),
        }
    }
}

fn main() -> Result<()> {
    Cli::parse().run()
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
