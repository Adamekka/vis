//! Decodes named SQL columns into data models before mapping them to domain models.

use postgres::Row;

use crate::app_error::AppError;

use super::models::*;

impl TryFrom<Row> for StoredPlayer {
    type Error = AppError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
        })
    }
}

impl TryFrom<Row> for StoredGenre {
    type Error = AppError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
        })
    }
}

impl TryFrom<Row> for StoredGameGenre {
    type Error = AppError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            game_id: row.try_get("game_id")?,
            genre_id: row.try_get("genre_id")?,
        })
    }
}

impl TryFrom<Row> for StoredAchievement {
    type Error = AppError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            game_id: row.try_get("game_id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            points: row.try_get("points")?,
        })
    }
}

impl TryFrom<Row> for GameView {
    type Error = AppError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            record: StoredGame {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                release_date: row.try_get("release_date")?,
            },
            genres: row.try_get("genres")?,
        })
    }
}

impl TryFrom<Row> for PlayerGameView {
    type Error = AppError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            record: StoredPlayerGame {
                player_id: row.try_get("player_id")?,
                game_id: row.try_get("game_id")?,
                status: StoredStatus::try_from(row.try_get::<_, &str>("status")?)?,
                playtime_minutes: row.try_get("playtime_minutes")?,
                added_at: row.try_get("added_at")?,
            },
            title: row.try_get("title")?,
        })
    }
}

impl TryFrom<Row> for PlayerAchievementView {
    type Error = AppError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            record: StoredPlayerAchievement {
                player_id: row.try_get("player_id")?,
                achievement_id: row.try_get("achievement_id")?,
                game_id: row.try_get("game_id")?,
                unlocked_at: row.try_get("unlocked_at")?,
            },
            game_title: row.try_get("game_title")?,
            name: row.try_get("name")?,
            points: row.try_get("points")?,
        })
    }
}
