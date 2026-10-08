mod app_error;
mod data;
mod domain;
mod presentation;
mod service;

use std::{env, error::Error, process::ExitCode};

use clap::Parser;

use crate::{
    app_error::AppError,
    data::{JsonStorage, PostgresStorage},
    presentation::{Backend, Cli},
    service::LibraryService,
};

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            if cfg!(debug_assertions)
                && let Some(source) = error.source()
            {
                eprintln!("Details: {source:?}");
            }
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<(), AppError> {
    match cli.storage {
        Backend::Postgres => {
            if cli.file.is_some() {
                return Err(AppError::Configuration("Use --file with --storage json."));
            }
            let database_url = env::var("DATABASE_URL").map_err(|_| {
                AppError::Configuration("Set DATABASE_URL to your PostgreSQL connection string.")
            })?;
            if database_url.trim().is_empty() {
                return Err(AppError::Configuration(
                    "Set DATABASE_URL to your PostgreSQL connection string.",
                ));
            }
            let storage = PostgresStorage::connect(&database_url)?;
            cli.command.execute(&mut LibraryService::new(storage))
        }
        Backend::Json => {
            // Clap requires a path in JSON mode, so absence is a parser invariant violation.
            let path = cli.file.expect("clap requires --file for JSON storage");
            let storage = JsonStorage::open(&path)?;
            cli.command.execute(&mut LibraryService::new(storage))
        }
    }
}
