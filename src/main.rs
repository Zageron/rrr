use clap::{Args, Parser, Subcommand};
use rrr_tui;

#[derive(Parser)]
#[command(name = "rrr")]
#[command(author = "Zageron <hello@zageron.ca>")]
#[command(version = "1.0")]
#[command(propagate_version = true)]
#[command(about = "Interface for interacting with RRR.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start playing a chart immediately!
    Play(Play),

    /// Start the user interface.
    Tui,
}

#[derive(Args, Debug)]
struct Play {
    song_id: u16,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Play(id) => {
            println!("Opens the game to play song id {:?}", id);
        }
        Commands::Tui => {
            let _res = rrr_tui::init();
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
