/// Operation kinds without their input, so callers can ask what a role may do before filling in a form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    AllItems,
    AddPlayer,
    Players,
    AddGame,
    AddGameToLibrary,
    Games,
    AddGenre,
    Genres,
    TagGame,
    AddToLibrary,
    Library,
    SetProgress,
    AddAchievement,
    Achievements,
    Unlock,
    Unlocked,
}

impl Action {
    // Rust cannot enumerate enum variants without a macro crate; the GUI test checks this list covers every command.
    pub const ALL: [Self; 16] = [
        Self::AllItems,
        Self::AddPlayer,
        Self::Players,
        Self::AddGame,
        Self::AddGameToLibrary,
        Self::Games,
        Self::AddGenre,
        Self::Genres,
        Self::TagGame,
        Self::AddToLibrary,
        Self::Library,
        Self::SetProgress,
        Self::AddAchievement,
        Self::Achievements,
        Self::Unlock,
        Self::Unlocked,
    ];
}
