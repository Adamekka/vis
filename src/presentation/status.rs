use clap::{ValueEnum, builder::PossibleValue};

use crate::domain::Status;

// Implement the CLI adapter here so the domain enum has no dependency on Clap.
impl ValueEnum for Status {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::NotStarted,
            Self::Playing,
            Self::Completed,
            Self::Abandoned,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(match self {
            Self::NotStarted => "not_started",
            Self::Playing => "playing",
            Self::Completed => "completed",
            Self::Abandoned => "abandoned",
        }))
    }
}
