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
    pub fn execute(self, service: &mut impl Service) -> Result<(), AppError> {
        // Use the same tables for individual lists and the combined report.
        fn print_response(response: Response) {
            match response {
                Response::AllItems(items) => {
                    println!("Players");
                    print_response(Response::Players(items.players));
                    println!("\nGames");
                    print_response(Response::Games(items.games));
                    println!("\nGenres");
                    print_response(Response::Genres(items.genres));
                    println!("\nGenre assignments");
                    println!("Game ID\tGenre ID");
                    for tag in items.game_genres {
                        println!("{}\t{}", tag.game_id, tag.genre_id);
                    }
                    println!("\nLibrary entries");
                    print_response(Response::Library(items.library));
                    println!("\nAchievements");
                    print_response(Response::Achievements(items.achievements));
                    println!("\nUnlocked achievements");
                    print_response(Response::Unlocked(items.unlocked));
                }
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
                            game.id, game.title, game.release_date, game.genres
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
        }
        print_response(service.execute(self.into())?.into());
        Ok(())
    }
}
