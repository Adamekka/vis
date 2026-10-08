use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use super::command::Command;

#[derive(Parser)]
#[command(version, about = "Manage a game library")]
pub struct Cli {
    #[arg(long, value_enum, default_value = "postgres", global = true)]
    pub storage: Backend,
    #[arg(long, required_if_eq("storage", "json"), global = true)]
    pub file: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Backend {
    Postgres,
    Json,
}
