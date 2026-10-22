use clap::Subcommand;

use crate::{app_error::AppError, service::Service};

use super::{models::Response, status::Status};

#[derive(Subcommand)]
pub enum Command {
    /// List all players, games, genres, library entries, achievements, and relationships
    AllItems,
    /// Add a player and print their ID
    AddPlayer { username: String, email: String },
    /// List players
    Players,
    /// Add a game and print its ID; release date uses YYYY-MM-DD
    AddGame { title: String, release_date: String },
    /// List games and their genres
    Games,
    /// Add a genre and print its ID
    AddGenre { name: String },
    /// List genres
    Genres,
    /// Assign a genre to a game; a game can have multiple genres
    TagGame { game_id: i64, genre_id: i64 },
    /// Add a game to a player's library with zero playtime and status not_started
    AddToLibrary { player_id: i64, game_id: i64 },
    /// Show a player's details, games, progress, and locked or unlocked achievements
    Library { player_id: i64 },
    /// Set the status and total playtime of a game in a player's library
    SetProgress {
        player_id: i64,
        game_id: i64,
        #[arg(value_enum)]
        status: Status,
        #[arg(value_parser = clap::value_parser!(i32).range(0..))]
        playtime_minutes: i32,
    },
    /// Add an achievement to a game and print its ID
    AddAchievement {
        game_id: i64,
        name: String,
        description: String,
        #[arg(value_parser = clap::value_parser!(i32).range(0..))]
        points: i32,
    },
    /// List a game's achievements
    Achievements { game_id: i64 },
    /// Unlock an achievement for a player who owns its game
    Unlock { player_id: i64, achievement_id: i64 },
    /// List a player's unlocked achievements
    Unlocked { player_id: i64 },
}

impl Command {
    pub fn execute(self, service: &mut impl Service) -> Result<(), AppError> {
        print!("{}", Response::from(service.execute(self.into())?).text());
        Ok(())
    }
}
