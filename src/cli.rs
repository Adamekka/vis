use clap::Parser;

use crate::command::Command;

#[derive(Parser)]
#[command(version, about = "Manage a game library")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
