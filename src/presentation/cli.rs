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
    /// Account to sign in with for a command; the window has its own sign-in form
    #[arg(long, global = true)]
    pub user: Option<String>,
    /// Password for --user; passed as an argument because the login is intentionally naive
    #[arg(long, global = true)]
    pub password: Option<String>,
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
    pub fn execute(
        self,
        mut service: impl Service + 'static,
        user: Option<String>,
        password: Option<String>,
    ) -> Result<(), AppError> {
        match self {
            Self::Gui => {
                if user.is_some() || password.is_some() {
                    return Err(AppError::Configuration(
                        "Sign in from the window. --user and --password apply only to commands.",
                    ));
                }
                super::gui::run(service)
            }
            Self::Command(command) => {
                // Commands run once per process, so each one signs in before it reaches the service.
                let (Some(user), Some(password)) = (user, password) else {
                    return Err(AppError::Configuration(
                        "Sign in with --user and --password.",
                    ));
                };
                service.sign_in(&user, &password)?;
                command.execute(&mut service)
            }
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Backend {
    Postgres,
    Json,
}
