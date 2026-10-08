use crate::app_error::AppError;

use super::Status;

pub struct PlayerGame {
    pub player_id: i64,
    pub game_id: i64,
    pub title: String,
    pub status: String,
    pub playtime_minutes: i32,
    pub added_at: String,
}

#[derive(Debug)]
pub struct Progress {
    status: Status,
    playtime_minutes: i32,
}

impl Progress {
    pub fn new(status: Status, playtime_minutes: i32) -> Result<Self, AppError> {
        if playtime_minutes < 0 {
            return Err(AppError::InvalidValue);
        }
        Ok(Self {
            status,
            playtime_minutes,
        })
    }

    pub fn into_parts(self) -> (Status, i32) {
        (self.status, self.playtime_minutes)
    }
}
