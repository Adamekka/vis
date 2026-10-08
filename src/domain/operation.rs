use super::{
    Achievement, AllItems, Game, GameGenre, Genre, NewAchievement, NewGame, NewGenre, NewPlayer,
    Player, PlayerAchievement, PlayerGame, Progress,
};

pub enum Operation {
    AllItems,
    AddPlayer(NewPlayer),
    Players,
    AddGame(NewGame),
    Games,
    AddGenre(NewGenre),
    Genres,
    TagGame(GameGenre),
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
        progress: Progress,
    },
    AddAchievement {
        game_id: i64,
        achievement: NewAchievement,
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

pub enum Outcome {
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
