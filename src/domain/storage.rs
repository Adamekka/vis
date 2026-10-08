use crate::app_error::AppError;

use super::{Achievement, Game, GameGenre, Genre, Player, PlayerAchievement, PlayerGame, Status};

pub trait Storage {
    fn add_player(&mut self, username: &str, email: &str) -> Result<i64, AppError>;
    fn players(&mut self) -> Result<Vec<Player>, AppError>;
    fn add_game(&mut self, title: &str, release_date: &str) -> Result<i64, AppError>;
    fn games(&mut self) -> Result<Vec<Game>, AppError>;
    fn add_genre(&mut self, name: &str) -> Result<i64, AppError>;
    fn genres(&mut self) -> Result<Vec<Genre>, AppError>;
    fn tag_game(&mut self, tag: GameGenre) -> Result<(), AppError>;
    fn add_to_library(&mut self, player_id: i64, game_id: i64) -> Result<(), AppError>;
    fn library(&mut self, player_id: i64) -> Result<Vec<PlayerGame>, AppError>;
    fn set_progress(
        &mut self,
        player_id: i64,
        game_id: i64,
        status: Status,
        playtime_minutes: i32,
    ) -> Result<(), AppError>;
    fn add_achievement(
        &mut self,
        game_id: i64,
        name: &str,
        description: &str,
        points: i32,
    ) -> Result<i64, AppError>;
    fn achievements(&mut self, game_id: i64) -> Result<Vec<Achievement>, AppError>;
    fn unlock(&mut self, player_id: i64, achievement_id: i64) -> Result<(), AppError>;
    fn unlocked(&mut self, player_id: i64) -> Result<Vec<PlayerAchievement>, AppError>;
}
