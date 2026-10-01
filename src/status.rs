use clap::ValueEnum;

#[derive(Clone, Copy, ValueEnum, serde::Deserialize, serde::Serialize)]
#[value(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
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
