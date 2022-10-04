mod completions;

use clap::CommandFactory;
use clap::{Args, Parser, Subcommand};
use completions::Shell;
use rrr_config::Config;

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

    /// Generate completions for your shell.
    Completions(Shell),
}

#[derive(Args, Debug)]
struct Play {
    song_id: u16,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Play(args) => rrr_window::init(Config::default(), args.song_id),
        Commands::Tui => {
            let _res = rrr_tui::init();
        }
        Commands::Completions(shell) => completions::process(&shell.shell, &mut Cli::command()),
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
