//! Service requests and results have their own types so callers do not depend on domain entities.

pub enum Request {
    AllItems,
    AddPlayer {
        username: String,
        email: String,
    },
    Players,
    AddGame {
        title: String,
        release_date: String,
    },
    Games,
    AddGenre {
        name: String,
    },
    Genres,
    TagGame {
        game_id: i64,
        genre_id: i64,
    },
    AddToLibrary {
        player_id: i64,
        game_id: i64,
    },
    Library {
        player_id: i64,
    },
    SetProgress {
        player_id: i64,
        game_id: i64,
        status: Status,
        playtime_minutes: i32,
    },
    AddAchievement {
        game_id: i64,
        name: String,
        description: String,
        points: i32,
    },
    Achievements {
        game_id: i64,
    },
    Unlock {
        player_id: i64,
        achievement_id: i64,
    },
    Unlocked {
        player_id: i64,
    },
}

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

#[derive(Clone, Copy)]
pub enum Status {
    NotStarted,
    Playing,
    Completed,
    Abandoned,
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
    pub genres: Vec<String>,
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
    pub status: Status,
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
