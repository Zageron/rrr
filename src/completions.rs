use clap::{Args, Command};
use clap_complete::{
    generate,
    shells::{self, Bash, Elvish, Fish, PowerShell, Zsh},
};
use std::io;

#[derive(Args, Debug)]
pub struct Shell {
    pub shell: shells::Shell,
}

pub fn process(shell: &shells::Shell, cmd: &mut Command) {
    match shell {
        shells::Shell::Bash => generate(Bash, cmd, "rrr", &mut io::stdout()),
        shells::Shell::Elvish => generate(Elvish, cmd, "rrr", &mut io::stdout()),
        shells::Shell::Fish => generate(Fish, cmd, "rrr", &mut io::stdout()),
        shells::Shell::PowerShell => generate(PowerShell, cmd, "rrr", &mut io::stdout()),
        shells::Shell::Zsh => generate(Zsh, cmd, "rrr", &mut io::stdout()),
        _ => panic!("No completion support for this shell."),
    };
}
