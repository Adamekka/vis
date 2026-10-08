//! Maps service requests to validated domain operations and domain results to service models.

use crate::{app_error::AppError, domain};

use super::models;

impl TryFrom<models::Request> for domain::Operation {
    type Error = AppError;

    fn try_from(request: models::Request) -> Result<Self, Self::Error> {
        Ok(match request {
            models::Request::AllItems => Self::AllItems,
            models::Request::AddPlayer { username, email } => {
                Self::AddPlayer(domain::NewPlayer::try_from((username, email))?)
            }
            models::Request::Players => Self::Players,
            models::Request::AddGame {
                title,
                release_date,
            } => Self::AddGame(domain::NewGame::try_from((title, release_date))?),
            models::Request::Games => Self::Games,
            models::Request::AddGenre { name } => Self::AddGenre(domain::NewGenre::try_from(name)?),
            models::Request::Genres => Self::Genres,
            models::Request::TagGame { game_id, genre_id } => {
                Self::TagGame(domain::GameGenre { game_id, genre_id })
            }
            models::Request::AddToLibrary { player_id, game_id } => {
                Self::AddToLibrary { player_id, game_id }
            }
            models::Request::Library { player_id } => Self::Library { player_id },
            models::Request::SetProgress {
                player_id,
                game_id,
                status,
                playtime_minutes,
            } => Self::SetProgress {
                player_id,
                game_id,
                progress: domain::Progress::try_from((status.into(), playtime_minutes))?,
            },
            models::Request::AddAchievement {
                game_id,
                name,
                description,
                points,
            } => Self::AddAchievement {
                game_id,
                achievement: domain::NewAchievement::try_from((name, description, points))?,
            },
            models::Request::Achievements { game_id } => Self::Achievements { game_id },
            models::Request::Unlock {
                player_id,
                achievement_id,
            } => Self::Unlock {
                player_id,
                achievement_id,
            },
            models::Request::Unlocked { player_id } => Self::Unlocked { player_id },
        })
    }
}

impl From<models::Status> for domain::Status {
    fn from(status: models::Status) -> Self {
        match status {
            models::Status::NotStarted => Self::NotStarted,
            models::Status::Playing => Self::Playing,
            models::Status::Completed => Self::Completed,
            models::Status::Abandoned => Self::Abandoned,
        }
    }
}

impl From<domain::Status> for models::Status {
    fn from(status: domain::Status) -> Self {
        match status {
            domain::Status::NotStarted => Self::NotStarted,
            domain::Status::Playing => Self::Playing,
            domain::Status::Completed => Self::Completed,
            domain::Status::Abandoned => Self::Abandoned,
        }
    }
}

impl From<domain::Player> for models::Player {
    fn from(value: domain::Player) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
        }
    }
}

impl From<domain::Game> for models::Game {
    fn from(value: domain::Game) -> Self {
        Self {
            id: value.id,
            title: value.title,
            release_date: value.release_date,
            genres: value.genres,
        }
    }
}

impl From<domain::Genre> for models::Genre {
    fn from(value: domain::Genre) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

impl From<domain::GameGenre> for models::GameGenre {
    fn from(value: domain::GameGenre) -> Self {
        Self {
            game_id: value.game_id,
            genre_id: value.genre_id,
        }
    }
}

impl From<domain::PlayerGame> for models::PlayerGame {
    fn from(value: domain::PlayerGame) -> Self {
        Self {
            player_id: value.player_id,
            game_id: value.game_id,
            title: value.title,
            status: value.status.into(),
            playtime_minutes: value.playtime_minutes,
            added_at: value.added_at,
        }
    }
}

impl From<domain::Achievement> for models::Achievement {
    fn from(value: domain::Achievement) -> Self {
        Self {
            id: value.id,
            game_id: value.game_id,
            name: value.name,
            description: value.description,
            points: value.points,
        }
    }
}

impl From<domain::PlayerAchievement> for models::PlayerAchievement {
    fn from(value: domain::PlayerAchievement) -> Self {
        Self {
            player_id: value.player_id,
            achievement_id: value.achievement_id,
            game_id: value.game_id,
            game_title: value.game_title,
            name: value.name,
            points: value.points,
            unlocked_at: value.unlocked_at,
        }
    }
}

impl From<domain::AllItems> for models::AllItems {
    fn from(value: domain::AllItems) -> Self {
        Self {
            players: value.players.into_iter().map(Into::into).collect(),
            games: value.games.into_iter().map(Into::into).collect(),
            genres: value.genres.into_iter().map(Into::into).collect(),
            game_genres: value.game_genres.into_iter().map(Into::into).collect(),
            library: value.library.into_iter().map(Into::into).collect(),
            achievements: value.achievements.into_iter().map(Into::into).collect(),
            unlocked: value.unlocked.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<domain::Outcome> for models::Response {
    fn from(response: domain::Outcome) -> Self {
        match response {
            domain::Outcome::AllItems(value) => Self::AllItems(value.into()),
            domain::Outcome::Created(value) => Self::Created(value),
            domain::Outcome::Players(value) => {
                Self::Players(value.into_iter().map(Into::into).collect())
            }
            domain::Outcome::Games(value) => {
                Self::Games(value.into_iter().map(Into::into).collect())
            }
            domain::Outcome::Genres(value) => {
                Self::Genres(value.into_iter().map(Into::into).collect())
            }
            domain::Outcome::GenreAssigned => Self::GenreAssigned,
            domain::Outcome::AddedToLibrary => Self::AddedToLibrary,
            domain::Outcome::Library(value) => {
                Self::Library(value.into_iter().map(Into::into).collect())
            }
            domain::Outcome::ProgressUpdated => Self::ProgressUpdated,
            domain::Outcome::Achievements(value) => {
                Self::Achievements(value.into_iter().map(Into::into).collect())
            }
            domain::Outcome::AchievementUnlocked => Self::AchievementUnlocked,
            domain::Outcome::Unlocked(value) => {
                Self::Unlocked(value.into_iter().map(Into::into).collect())
            }
        }
    }
}
