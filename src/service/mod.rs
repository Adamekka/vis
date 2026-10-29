mod mapper;
pub mod models;

use crate::{
    app_error::AppError,
    domain::{AllItems, GameGenre, Operation, Outcome, Progress, Status, Storage},
};
use models::{Request, Response};

pub trait Service {
    fn execute(&mut self, request: Request) -> Result<Response, AppError>;
}

pub struct LibraryService<S> {
    storage: S,
}

impl<S: Storage> LibraryService<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    fn execute_operation(&mut self, operation: Operation) -> Result<Outcome, AppError> {
        let storage = &mut self.storage;
        let outcome = match operation {
            Operation::AddGameToLibrary {
                player_id,
                game,
                genre_ids,
                achievements,
            } => {
                // Creation operations return a shared outcome enum; another variant is a dispatcher bug.
                let Outcome::Created(game_id) = self.execute_operation(Operation::AddGame(game))?
                else {
                    unreachable!("AddGame must return the created game ID");
                };
                let mut assigned_genres = Vec::new();
                let mut achievement_ids = Vec::new();
                // Reuse each standalone operation without a transaction so every successful write stays saved.
                let result = (|| {
                    for genre_id in genre_ids {
                        self.execute_operation(Operation::TagGame(GameGenre { game_id, genre_id }))
                            .map_err(|error| (format!("assign genre {genre_id}"), error))?;
                        assigned_genres.push(genre_id);
                    }
                    for (index, achievement) in achievements.into_iter().enumerate() {
                        let Outcome::Created(id) = self
                            .execute_operation(Operation::AddAchievement {
                                game_id,
                                achievement,
                            })
                            .map_err(|error| {
                                (format!("create achievement {}", index + 1), error)
                            })?
                        else {
                            unreachable!("AddAchievement must return the created achievement ID");
                        };
                        achievement_ids.push(id);
                    }
                    self.execute_operation(Operation::AddToLibrary { player_id, game_id })
                        .map_err(|error| {
                            (
                                format!("add the game to player {player_id}'s library"),
                                error,
                            )
                        })?;
                    Ok::<_, (String, AppError)>(())
                })();
                if let Err((step, source)) = result {
                    return Err(AppError::Failure {
                        message: format!(
                            "Could not {step}. {source}\nGame ID {game_id} remains saved. Assigned genre IDs: {assigned_genres:?}. Created achievement IDs: {achievement_ids:?}. The game was not added to the player's library. Use Tag game, Add achievement, and Add to library to continue with these IDs."
                        ),
                        source: Box::new(source),
                    });
                }
                Outcome::GameAddedToLibrary {
                    player_id,
                    game_id,
                    genre_ids: assigned_genres,
                    achievement_ids,
                }
            }
            Operation::AllItems => {
                let players = storage.players()?;
                let games = storage.games()?;
                let genres = storage.genres()?;
                let game_genres = storage.game_genres()?;
                let mut library = Vec::new();
                let mut achievements = Vec::new();
                let mut unlocked = Vec::new();
                for player in &players {
                    library.extend(storage.library(player.id)?.entries);
                    unlocked.extend(storage.unlocked(player.id)?);
                }
                for game in &games {
                    achievements.extend(storage.achievements(game.id)?);
                }
                Outcome::AllItems(AllItems {
                    players,
                    games,
                    genres,
                    game_genres,
                    library,
                    achievements,
                    unlocked,
                })
            }
            Operation::AddPlayer(player) => Outcome::Created(storage.add_player(player)?),
            Operation::Players => Outcome::Players(storage.players()?),
            Operation::AddGame(game) => Outcome::Created(storage.add_game(game)?),
            Operation::Games => Outcome::Games(storage.games()?),
            Operation::AddGenre(genre) => Outcome::Created(storage.add_genre(genre)?),
            Operation::Genres => Outcome::Genres(storage.genres()?),
            Operation::TagGame(tag) => {
                storage.tag_game(tag)?;
                Outcome::GenreAssigned
            }
            Operation::AddToLibrary { player_id, game_id } => {
                storage.add_to_library(
                    player_id,
                    game_id,
                    Progress::try_from((Status::NotStarted, 0))?,
                )?;
                Outcome::AddedToLibrary
            }
            Operation::Library { player_id } => Outcome::Library(storage.library(player_id)?),
            Operation::SetProgress {
                player_id,
                game_id,
                progress,
            } => {
                storage.set_progress(player_id, game_id, progress)?;
                Outcome::ProgressUpdated
            }
            Operation::AddAchievement {
                game_id,
                achievement,
            } => Outcome::Created(storage.add_achievement(game_id, achievement)?),
            Operation::Achievements { game_id } => {
                Outcome::Achievements(storage.achievements(game_id)?)
            }
            Operation::Unlock {
                player_id,
                achievement_id,
            } => {
                let achievement = storage.achievement(achievement_id)?;
                if !storage.owns_game(player_id, achievement.game_id)? {
                    return Err(AppError::InvalidReference);
                }
                // Database and file constraints still protect the write if data changes after this check.
                storage.unlock(player_id, achievement)?;
                Outcome::AchievementUnlocked
            }
            Operation::Unlocked { player_id } => Outcome::Unlocked(storage.unlocked(player_id)?),
        };
        Ok(outcome)
    }
}

impl<S: Storage> Service for LibraryService<S> {
    fn execute(&mut self, request: Request) -> Result<Response, AppError> {
        self.execute_operation(Operation::try_from(request)?)
            .map(Into::into)
    }
}

#[cfg(test)]
mod tests;
