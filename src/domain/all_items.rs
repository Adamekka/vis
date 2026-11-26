use super::{Achievement, Game, GameGenre, Genre, PlayerAchievement, PlayerGame, PlayerListing};

pub struct AllItems {
    pub players: Vec<PlayerListing>,
    pub games: Vec<Game>,
    pub genres: Vec<Genre>,
    pub game_genres: Vec<GameGenre>,
    pub library: Vec<PlayerGame>,
    pub achievements: Vec<Achievement>,
    pub unlocked: Vec<PlayerAchievement>,
}
