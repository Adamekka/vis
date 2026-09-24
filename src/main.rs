mod achievement;
mod app_error;
mod cli;
mod command;
mod game;
mod game_genre;
mod genre;
mod player;
mod player_achievement;
mod player_game;
mod status;

use crate::{
    achievement::Achievement, app_error::AppError, cli::Cli, command::Command, game::Game,
    game_genre::GameGenre, genre::Genre, player::Player, player_achievement::PlayerAchievement,
    player_game::PlayerGame,
};
use clap::Parser;
use postgres::{Client, NoTls};
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli.command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            if cfg!(debug_assertions)
                && let AppError::Connection(source) | AppError::Database(source) = &error
            {
                eprintln!("Details: {source:?}");
            }
            ExitCode::FAILURE
        }
    }
}

fn run(command: Command) -> Result<(), AppError> {
    let database_url = env::var("DATABASE_URL").map_err(|_| AppError::Configuration)?;
    if database_url.trim().is_empty() {
        return Err(AppError::Configuration);
    }
    let mut db = Client::connect(&database_url, NoTls).map_err(AppError::Connection)?;

    match command {
        Command::AddPlayer { username, email } => {
            println!("{}", Player::add(&mut db, &username, &email)?);
        }
        Command::Players => {
            let players = Player::list(&mut db)?;
            println!("ID\tUsername\tEmail");
            for player in players {
                println!("{}\t{}\t{}", player.id, player.username, player.email);
            }
        }
        Command::AddGame {
            title,
            release_date,
        } => {
            println!("{}", Game::add(&mut db, &title, &release_date)?);
        }
        Command::Games => {
            let games = Game::list(&mut db)?;
            println!("ID\tTitle\tReleased\tGenres");
            for game in games {
                println!(
                    "{}\t{}\t{}\t{}",
                    game.id,
                    game.title,
                    game.release_date,
                    game.genres.join(", ")
                );
            }
        }
        Command::AddGenre { name } => {
            println!("{}", Genre::add(&mut db, &name)?);
        }
        Command::Genres => {
            let genres = Genre::list(&mut db)?;
            println!("ID\tName");
            for genre in genres {
                println!("{}\t{}", genre.id, genre.name);
            }
        }
        Command::TagGame { game_id, genre_id } => {
            GameGenre { game_id, genre_id }.add(&mut db)?;
            println!("Genre assigned.");
        }
        Command::AddToLibrary { player_id, game_id } => {
            PlayerGame::add(&mut db, player_id, game_id)?;
            println!("Game added to library.");
        }
        Command::Library { player_id } => {
            let library = PlayerGame::list(&mut db, player_id)?;
            println!("Player ID\tGame ID\tTitle\tStatus\tMinutes\tAdded at");
            for entry in library {
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}",
                    entry.player_id,
                    entry.game_id,
                    entry.title,
                    entry.status,
                    entry.playtime_minutes,
                    entry.added_at
                );
            }
        }
        Command::SetProgress {
            player_id,
            game_id,
            status,
            playtime_minutes,
        } => {
            PlayerGame::set_progress(&mut db, player_id, game_id, status, playtime_minutes)?;
            println!("Progress updated.");
        }
        Command::AddAchievement {
            game_id,
            name,
            description,
            points,
        } => {
            println!(
                "{}",
                Achievement::add(&mut db, game_id, &name, &description, points)?
            );
        }
        Command::Achievements { game_id } => {
            let achievements = Achievement::list(&mut db, game_id)?;
            println!("ID\tGame ID\tName\tDescription\tPoints");
            for achievement in achievements {
                println!(
                    "{}\t{}\t{}\t{}\t{}",
                    achievement.id,
                    achievement.game_id,
                    achievement.name,
                    achievement.description,
                    achievement.points
                );
            }
        }
        Command::Unlock {
            player_id,
            achievement_id,
        } => {
            PlayerAchievement::unlock(&mut db, player_id, achievement_id)?;
            println!("Achievement unlocked.");
        }
        Command::Unlocked { player_id } => {
            let achievements = PlayerAchievement::list(&mut db, player_id)?;
            println!("Player ID\tID\tGame ID\tGame\tAchievement\tPoints\tUnlocked at");
            for achievement in achievements {
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    achievement.player_id,
                    achievement.achievement_id,
                    achievement.game_id,
                    achievement.game_title,
                    achievement.name,
                    achievement.points,
                    achievement.unlocked_at
                );
            }
        }
    }
    Ok(())
}
