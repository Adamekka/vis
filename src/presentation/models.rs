//! Output models hold display-ready values for the command-line tables.

pub enum Response {
    AllItems(AllItems),
    Created(i64),
    Players(Vec<Player>),
    Games(Vec<Game>),
    Genres(Vec<Genre>),
    GenreAssigned,
    AddedToLibrary,
    Library(Vec<PlayerGame>),
    ProgressUpdated,
    Achievements(Vec<Achievement>),
    AchievementUnlocked,
    Unlocked(Vec<PlayerAchievement>),
}

pub struct Player {
    pub id: i64,
    pub username: String,
    pub email: String,
}

pub struct Game {
    pub id: i64,
    pub title: String,
    pub release_date: String,
    pub genres: String,
}

pub struct Genre {
    pub id: i64,
    pub name: String,
}

pub struct GameGenre {
    pub game_id: i64,
    pub genre_id: i64,
}

pub struct PlayerGame {
    pub player_id: i64,
    pub game_id: i64,
    pub title: String,
    pub status: String,
    pub playtime_minutes: i32,
    pub added_at: String,
}

pub struct Achievement {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub description: String,
    pub points: i32,
}

pub struct PlayerAchievement {
    pub player_id: i64,
    pub achievement_id: i64,
    pub game_id: i64,
    pub game_title: String,
    pub name: String,
    pub points: i32,
    pub unlocked_at: String,
}

pub struct AllItems {
    pub players: Vec<Player>,
    pub games: Vec<Game>,
    pub genres: Vec<Genre>,
    pub game_genres: Vec<GameGenre>,
    pub library: Vec<PlayerGame>,
    pub achievements: Vec<Achievement>,
    pub unlocked: Vec<PlayerAchievement>,
}
