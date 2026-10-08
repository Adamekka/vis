use clap::Subcommand;

use crate::{
    app_error::AppError,
    domain::{Request, Response, Status, Storage},
};

#[derive(Subcommand)]
pub enum Command {
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
    /// List a player's games and progress
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
    pub fn execute(self, storage: &mut impl Storage) -> Result<(), AppError> {
        let request = match self {
            Self::AddPlayer { username, email } => Request::AddPlayer { username, email },
            Self::Players => Request::Players,
            Self::AddGame {
                title,
                release_date,
            } => Request::AddGame {
                title,
                release_date,
            },
            Self::Games => Request::Games,
            Self::AddGenre { name } => Request::AddGenre { name },
            Self::Genres => Request::Genres,
            Self::TagGame { game_id, genre_id } => Request::TagGame { game_id, genre_id },
            Self::AddToLibrary { player_id, game_id } => {
                Request::AddToLibrary { player_id, game_id }
            }
            Self::Library { player_id } => Request::Library { player_id },
            Self::SetProgress {
                player_id,
                game_id,
                status,
                playtime_minutes,
            } => Request::SetProgress {
                player_id,
                game_id,
                status,
                playtime_minutes,
            },
            Self::AddAchievement {
                game_id,
                name,
                description,
                points,
            } => Request::AddAchievement {
                game_id,
                name,
                description,
                points,
            },
            Self::Achievements { game_id } => Request::Achievements { game_id },
            Self::Unlock {
                player_id,
                achievement_id,
            } => Request::Unlock {
                player_id,
                achievement_id,
            },
            Self::Unlocked { player_id } => Request::Unlocked { player_id },
        };

        match request.execute(storage)? {
            Response::Created(id) => println!("{id}"),
            Response::Players(players) => {
                println!("ID\tUsername\tEmail");
                for player in players {
                    println!("{}\t{}\t{}", player.id, player.username, player.email);
                }
            }
            Response::Games(games) => {
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
            Response::Genres(genres) => {
                println!("ID\tName");
                for genre in genres {
                    println!("{}\t{}", genre.id, genre.name);
                }
            }
            Response::GenreAssigned => println!("Genre assigned."),
            Response::AddedToLibrary => println!("Game added to library."),
            Response::Library(library) => {
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
            Response::ProgressUpdated => println!("Progress updated."),
            Response::Achievements(achievements) => {
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
            Response::AchievementUnlocked => println!("Achievement unlocked."),
            Response::Unlocked(achievements) => {
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
}
