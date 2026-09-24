use std::fmt;

use postgres::error::SqlState;

pub enum AppError {
    Configuration,
    InvalidDate,
    Connection(postgres::Error),
    Database(postgres::Error),
    NotFound(&'static str),
}

impl From<postgres::Error> for AppError {
    fn from(error: postgres::Error) -> Self {
        Self::Database(error)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidDate => "Invalid release date. Use a valid date in YYYY-MM-DD format.",
            Self::Configuration => {
                "Set DATABASE_URL to your PostgreSQL connection string. See README.md for setup."
            }
            Self::Connection(_) => {
                "Could not connect to PostgreSQL. Check DATABASE_URL and start the database using the setup instructions in README.md."
            }
            Self::NotFound(message) => message,
            Self::Database(error) => match error.code() {
                Some(&SqlState::UNIQUE_VIOLATION) => {
                    "This entry already exists. Use the list commands to find its ID."
                }
                Some(&SqlState::FOREIGN_KEY_VIOLATION) => {
                    "A referenced entry does not exist, or the game is not in the player's library. Check the IDs and library."
                }
                Some(&SqlState::CHECK_VIOLATION | &SqlState::NOT_NULL_VIOLATION) => {
                    "Invalid value. Names, email, and descriptions must not be blank; points and playtime must be nonnegative."
                }
                Some(&SqlState::INVALID_DATETIME_FORMAT | &SqlState::DATETIME_FIELD_OVERFLOW) => {
                    "Invalid release date. Use a valid date in YYYY-MM-DD format."
                }
                Some(&SqlState::UNDEFINED_TABLE) => {
                    "The database schema is missing. Follow the database setup in README.md."
                }
                _ => "Database operation failed. Check the database connection and server logs.",
            },
        };
        f.write_str(message)
    }
}
