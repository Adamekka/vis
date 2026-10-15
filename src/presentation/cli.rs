use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use crate::{app_error::AppError, service::Service};

use super::command::Command;

#[derive(Parser)]
#[command(version, about = "Manage a game library")]
pub struct Cli {
    #[arg(long, value_enum, default_value = "postgres", global = true)]
    pub storage: Backend,
    #[arg(long, required_if_eq("storage", "json"), global = true)]
    pub file: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Launch,
}

#[derive(Subcommand)]
pub enum Launch {
    /// Open the native game library window
    Gui,
    #[command(flatten)]
    Command(Command),
}

impl Launch {
    pub fn execute(self, mut service: impl Service + 'static) -> Result<(), AppError> {
        match self {
            Self::Gui => super::gui::run(service),
            Self::Command(command) => command.execute(&mut service),
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Backend {
    Postgres,
    Json,
}
