use super::{
    Achievement, Action, AllItems, Game, GameGenre, Genre, NewAchievement, NewGame, NewGenre,
    NewPlayer, PlayerAchievement, PlayerLibrary, PlayerListing, Progress,
};

pub enum Operation {
    AllItems,
    AddPlayer(NewPlayer),
    Players,
    AddGame(NewGame),
    AddGameToLibrary {
        player_id: i64,
        game: NewGame,
        genre_ids: Vec<i64>,
        achievements: Vec<NewAchievement>,
    },
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

impl Operation {
    pub fn action(&self) -> Action {
        match self {
            Self::AllItems => Action::AllItems,
            Self::AddPlayer(_) => Action::AddPlayer,
            Self::Players => Action::Players,
            Self::AddGame(_) => Action::AddGame,
            Self::AddGameToLibrary { .. } => Action::AddGameToLibrary,
            Self::Games => Action::Games,
            Self::AddGenre(_) => Action::AddGenre,
            Self::Genres => Action::Genres,
            Self::TagGame(_) => Action::TagGame,
            Self::AddToLibrary { .. } => Action::AddToLibrary,
            Self::Library { .. } => Action::Library,
            Self::SetProgress { .. } => Action::SetProgress,
            Self::AddAchievement { .. } => Action::AddAchievement,
            Self::Achievements { .. } => Action::Achievements,
            Self::Unlock { .. } => Action::Unlock,
            Self::Unlocked { .. } => Action::Unlocked,
        }
    }
}

pub enum Outcome {
    AllItems(AllItems),
    Created(i64),
    GameAddedToLibrary {
        player_id: i64,
        game_id: i64,
        genre_ids: Vec<i64>,
        achievement_ids: Vec<i64>,
    },
    Players(Vec<PlayerListing>),
    Games(Vec<Game>),
    Genres(Vec<Genre>),
    GenreAssigned,
    AddedToLibrary,
    Library(PlayerLibrary),
    ProgressUpdated,
    Achievements(Vec<Achievement>),
    AchievementUnlocked,
    Unlocked(Vec<PlayerAchievement>),
}
