use crate::app_error::AppError;

use super::{
    Achievement, Game, GameGenre, Genre, NewAchievement, NewGame, NewGenre, NewPlayer, Player,
    PlayerAchievement, PlayerGame, Progress,
};

pub trait Storage {
    fn add_player(&mut self, player: NewPlayer) -> Result<i64, AppError>;
    fn players(&mut self) -> Result<Vec<Player>, AppError>;
    fn add_game(&mut self, game: NewGame) -> Result<i64, AppError>;
    fn games(&mut self) -> Result<Vec<Game>, AppError>;
    fn add_genre(&mut self, genre: NewGenre) -> Result<i64, AppError>;
    fn genres(&mut self) -> Result<Vec<Genre>, AppError>;
    fn tag_game(&mut self, tag: GameGenre) -> Result<(), AppError>;
    fn game_genres(&mut self) -> Result<Vec<GameGenre>, AppError>;
    fn add_to_library(
        &mut self,
        player_id: i64,
        game_id: i64,
        progress: Progress,
    ) -> Result<(), AppError>;
    fn owns_game(&mut self, player_id: i64, game_id: i64) -> Result<bool, AppError>;
    fn library(&mut self, player_id: i64) -> Result<Vec<PlayerGame>, AppError>;
    fn set_progress(
        &mut self,
        player_id: i64,
        game_id: i64,
        progress: Progress,
    ) -> Result<(), AppError>;
    fn add_achievement(
        &mut self,
        game_id: i64,
        achievement: NewAchievement,
    ) -> Result<i64, AppError>;
    fn achievements(&mut self, game_id: i64) -> Result<Vec<Achievement>, AppError>;
    fn achievement(&mut self, achievement_id: i64) -> Result<Achievement, AppError>;
    fn unlock(&mut self, player_id: i64, achievement: Achievement) -> Result<(), AppError>;
    fn unlocked(&mut self, player_id: i64) -> Result<Vec<PlayerAchievement>, AppError>;
}
