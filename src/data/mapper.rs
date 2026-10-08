//! Maps persisted records and write values across the data/domain boundary.

use crate::{app_error::AppError, domain};

use super::models::*;

impl From<StoredPlayer> for domain::Player {
    fn from(value: StoredPlayer) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
        }
    }
}

impl From<StoredGenre> for domain::Genre {
    fn from(value: StoredGenre) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

impl From<StoredGameGenre> for domain::GameGenre {
    fn from(value: StoredGameGenre) -> Self {
        Self {
            game_id: value.game_id,
            genre_id: value.genre_id,
        }
    }
}

impl From<StoredAchievement> for domain::Achievement {
    fn from(value: StoredAchievement) -> Self {
        Self {
            id: value.id,
            game_id: value.game_id,
            name: value.name,
            description: value.description,
            points: value.points,
        }
    }
}

impl From<GameView> for domain::Game {
    fn from(value: GameView) -> Self {
        Self {
            id: value.record.id,
            title: value.record.title,
            release_date: value.record.release_date,
            genres: value.genres,
        }
    }
}

impl From<PlayerGameView> for domain::PlayerGame {
    fn from(value: PlayerGameView) -> Self {
        Self {
            player_id: value.record.player_id,
            game_id: value.record.game_id,
            status: value.record.status.into(),
            playtime_minutes: value.record.playtime_minutes,
            added_at: value.record.added_at,
            title: value.title,
        }
    }
}

impl From<PlayerAchievementView> for domain::PlayerAchievement {
    fn from(value: PlayerAchievementView) -> Self {
        Self {
            player_id: value.record.player_id,
            achievement_id: value.record.achievement_id,
            game_id: value.record.game_id,
            unlocked_at: value.record.unlocked_at,
            game_title: value.game_title,
            name: value.name,
            points: value.points,
        }
    }
}

impl From<domain::NewPlayer> for PlayerValues {
    fn from(value: domain::NewPlayer) -> Self {
        let (username, email) = value.into_parts();
        Self { username, email }
    }
}

impl From<domain::NewGame> for GameValues {
    fn from(value: domain::NewGame) -> Self {
        let (title, release_date) = value.into_parts();
        Self {
            title,
            release_date,
        }
    }
}

impl From<domain::NewGenre> for GenreValues {
    fn from(value: domain::NewGenre) -> Self {
        let name = value.into_name();
        Self { name }
    }
}

impl From<domain::NewAchievement> for AchievementValues {
    fn from(value: domain::NewAchievement) -> Self {
        let (name, description, points) = value.into_parts();
        Self {
            name,
            description,
            points,
        }
    }
}

impl From<domain::Progress> for ProgressValues {
    fn from(value: domain::Progress) -> Self {
        let (status, playtime_minutes) = value.into_parts();
        Self {
            status: status.into(),
            playtime_minutes,
        }
    }
}

impl From<domain::GameGenre> for StoredGameGenre {
    fn from(value: domain::GameGenre) -> Self {
        Self {
            game_id: value.game_id,
            genre_id: value.genre_id,
        }
    }
}

impl From<domain::Achievement> for StoredAchievement {
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

impl From<domain::Status> for StoredStatus {
    fn from(status: domain::Status) -> Self {
        match status {
            domain::Status::NotStarted => Self::NotStarted,
            domain::Status::Playing => Self::Playing,
            domain::Status::Completed => Self::Completed,
            domain::Status::Abandoned => Self::Abandoned,
        }
    }
}

impl From<StoredStatus> for domain::Status {
    fn from(status: StoredStatus) -> Self {
        match status {
            StoredStatus::NotStarted => Self::NotStarted,
            StoredStatus::Playing => Self::Playing,
            StoredStatus::Completed => Self::Completed,
            StoredStatus::Abandoned => Self::Abandoned,
        }
    }
}

impl StoredStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Playing => "playing",
            Self::Completed => "completed",
            Self::Abandoned => "abandoned",
        }
    }
}

impl TryFrom<&str> for StoredStatus {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "not_started" => Ok(Self::NotStarted),
            "playing" => Ok(Self::Playing),
            "completed" => Ok(Self::Completed),
            "abandoned" => Ok(Self::Abandoned),
            _ => Err(AppError::InvalidValue),
        }
    }
}
