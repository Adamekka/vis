use crate::app_error::AppError;

use super::{
    Achievement, Game, GameGenre, Genre, NewAchievement, NewGame, NewGenre, NewPlayer, Player,
    PlayerAchievement, PlayerGame, Progress, Status, Storage,
};

pub enum Request {
    AddPlayer {
        username: String,
        email: String,
    },
    Players,
    AddGame {
        title: String,
        release_date: String,
    },
    Games,
    AddGenre {
        name: String,
    },
    Genres,
    TagGame {
        game_id: i64,
        genre_id: i64,
    },
    AddToLibrary {
        player_id: i64,
        game_id: i64,
    },
    Library {
        player_id: i64,
    },
    SetProgress {
        player_id: i64,
        game_id: i64,
        status: Status,
        playtime_minutes: i32,
    },
    AddAchievement {
        game_id: i64,
        name: String,
        description: String,
        points: i32,
    },
    Achievements {
        game_id: i64,
    },
    Unlock {
        player_id: i64,
        achievement_id: i64,
    },
    Unlocked {
        player_id: i64,
    },
}

pub enum Response {
    Created(i64),
    Players(Vec<Player>),
    Games(Vec<Game>),
    Genres(Vec<Genre>),
    GenreAssigned,
    AddedToLibrary,
    Library(Vec<PlayerGame>),
    ProgressUpdated,
    Achievements(Vec<Achievement>),
    AchievementUnlocked,
    Unlocked(Vec<PlayerAchievement>),
}

impl Request {
    pub fn execute(self, storage: &mut impl Storage) -> Result<Response, AppError> {
        match self {
            Self::AddPlayer { username, email } => Ok(Response::Created(
                storage.add_player(NewPlayer::new(username, email)?)?,
            )),
            Self::Players => Ok(Response::Players(storage.players()?)),
            Self::AddGame {
                title,
                release_date,
            } => Ok(Response::Created(
                storage.add_game(NewGame::new(title, release_date)?)?,
            )),
            Self::Games => Ok(Response::Games(storage.games()?)),
            Self::AddGenre { name } => {
                Ok(Response::Created(storage.add_genre(NewGenre::new(name)?)?))
            }
            Self::Genres => Ok(Response::Genres(storage.genres()?)),
            Self::TagGame { game_id, genre_id } => {
                storage.tag_game(GameGenre { game_id, genre_id })?;
                Ok(Response::GenreAssigned)
            }
            Self::AddToLibrary { player_id, game_id } => {
                storage.add_to_library(
                    player_id,
                    game_id,
                    Progress::new(Status::NotStarted, 0)?,
                )?;
                Ok(Response::AddedToLibrary)
            }
            Self::Library { player_id } => Ok(Response::Library(storage.library(player_id)?)),
            Self::SetProgress {
                player_id,
                game_id,
                status,
                playtime_minutes,
            } => {
                storage.set_progress(
                    player_id,
                    game_id,
                    Progress::new(status, playtime_minutes)?,
                )?;
                Ok(Response::ProgressUpdated)
            }
            Self::AddAchievement {
                game_id,
                name,
                description,
                points,
            } => Ok(Response::Created(storage.add_achievement(
                game_id,
                NewAchievement::new(name, description, points)?,
            )?)),
            Self::Achievements { game_id } => {
                Ok(Response::Achievements(storage.achievements(game_id)?))
            }
            Self::Unlock {
                player_id,
                achievement_id,
            } => {
                let achievement = storage.achievement(achievement_id)?;
                if !storage.owns_game(player_id, achievement.game_id)? {
                    return Err(AppError::InvalidReference);
                }
                // Database and file constraints still protect the write if data changes after this check.
                storage.unlock(player_id, achievement)?;
                Ok(Response::AchievementUnlocked)
            }
            Self::Unlocked { player_id } => Ok(Response::Unlocked(storage.unlocked(player_id)?)),
        }
    }
}
