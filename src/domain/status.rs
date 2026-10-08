#[derive(Clone, Copy, Debug)]
pub enum Status {
    NotStarted,
    Playing,
    Completed,
    Abandoned,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NotStarted => "not_started",
            Self::Playing => "playing",
            Self::Completed => "completed",
            Self::Abandoned => "abandoned",
        })
    }
}
