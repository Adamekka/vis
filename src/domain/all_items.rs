use super::{Achievement, Game, GameGenre, Genre, Player, PlayerAchievement, PlayerGame};

pub struct AllItems {
    pub players: Vec<Player>,
    pub games: Vec<Game>,
    pub genres: Vec<Genre>,
    pub game_genres: Vec<GameGenre>,
    pub library: Vec<PlayerGame>,
    pub achievements: Vec<Achievement>,
    pub unlocked: Vec<PlayerAchievement>,
}
