use std::{error::Error, fmt};

#[derive(Debug)]
pub enum AppError {
    Configuration(&'static str),
    InvalidDate,
    AlreadyExists,
    InvalidReference,
    InvalidValue,
    InvalidFile(&'static str),
    IdExhausted,
    NotFound(&'static str),
    Failure {
        message: String,
        source: Box<dyn Error + Send + Sync>,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Configuration(message) | Self::NotFound(message) => message,
            Self::InvalidDate => "Invalid release date. Use a valid date in YYYY-MM-DD format.",
            Self::AlreadyExists => {
                "This entry already exists. Use the list commands to find its ID."
            }
            Self::InvalidReference => {
                "A referenced entry does not exist, or the game is not in the player's library. Check the IDs and library."
            }
            Self::InvalidValue => {
                "Invalid value. Names, email, and descriptions must not be blank; points and playtime must be nonnegative."
            }
            Self::InvalidFile(reason) => return write!(f, "Invalid library file. {reason}"),
            Self::IdExhausted => "No more IDs are available. Choose another library file.",
            Self::Failure { message, .. } => message,
        };
        f.write_str(message)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Failure { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}
