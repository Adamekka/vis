use serde::{Deserialize, Serialize};

// Persist the normalized records. Titles, genre names, and achievement details in list
// results are joined at read time so the file never contains conflicting copies of them.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StoredPlayer {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StoredGame {
    pub id: i64,
    pub title: String,
    pub release_date: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StoredGenre {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StoredGameGenre {
    pub game_id: i64,
    pub genre_id: i64,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StoredStatus {
    NotStarted,
    Playing,
    Completed,
    Abandoned,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StoredPlayerGame {
    pub player_id: i64,
    pub game_id: i64,
    pub status: StoredStatus,
    pub playtime_minutes: i32,
    pub added_at: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StoredAchievement {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub description: String,
    pub points: i32,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StoredPlayerAchievement {
    pub player_id: i64,
    pub achievement_id: i64,
    pub game_id: i64,
    pub unlocked_at: String,
}

pub struct PlayerValues {
    pub username: String,
    pub email: String,
}

pub struct GameValues {
    pub title: String,
    pub release_date: String,
}

pub struct GenreValues {
    pub name: String,
}

pub struct AchievementValues {
    pub name: String,
    pub description: String,
    pub points: i32,
}

pub struct ProgressValues {
    pub status: StoredStatus,
    pub playtime_minutes: i32,
}

// Read views add joined labels without duplicating them in persisted records.
pub struct GameView {
    pub record: StoredGame,
    pub genres: Vec<String>,
}

pub struct PlayerGameView {
    pub record: StoredPlayerGame,
    pub title: String,
}

pub struct PlayerAchievementView {
    pub record: StoredPlayerAchievement,
    pub game_title: String,
    pub name: String,
    pub points: i32,
}
