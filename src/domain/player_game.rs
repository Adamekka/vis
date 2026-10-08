use super::Status;

pub struct PlayerGame {
    pub player_id: i64,
    pub game_id: i64,
    pub title: String,
    pub status: Status,
    pub playtime_minutes: i32,
    pub added_at: String,
}

#[derive(Debug)]
pub struct Progress {
    pub(super) status: Status,
    pub(super) playtime_minutes: i32,
}

impl Progress {
    pub fn into_parts(self) -> (Status, i32) {
        (self.status, self.playtime_minutes)
    }
}
