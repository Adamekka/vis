//! Fallible mappings validate raw input before constructing domain write models.

use crate::app_error::AppError;

use super::{NewAchievement, NewGame, NewGenre, NewPlayer, Progress, Status};

impl TryFrom<(String, String)> for NewPlayer {
    type Error = AppError;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        let (username, email) = value;
        if username.trim().is_empty() || email.trim().is_empty() {
            return Err(AppError::InvalidValue);
        }
        Ok(Self { username, email })
    }
}

impl TryFrom<(String, String)> for NewGame {
    type Error = AppError;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        let (title, release_date) = value;
        if title.trim().is_empty() {
            return Err(AppError::InvalidValue);
        }
        // PostgreSQL has no year zero, while chrono accepts it.
        if release_date.len() != 10
            || release_date.starts_with("0000")
            || !release_date.bytes().enumerate().all(|(index, byte)| {
                if index == 4 || index == 7 {
                    byte == b'-'
                } else {
                    byte.is_ascii_digit()
                }
            })
            || chrono::NaiveDate::parse_from_str(&release_date, "%Y-%m-%d").is_err()
        {
            return Err(AppError::InvalidDate);
        }
        Ok(Self {
            title,
            release_date,
        })
    }
}

impl TryFrom<String> for NewGenre {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let name = value;
        if name.trim().is_empty() {
            return Err(AppError::InvalidValue);
        }
        Ok(Self { name })
    }
}

impl TryFrom<(String, String, i32)> for NewAchievement {
    type Error = AppError;

    fn try_from(value: (String, String, i32)) -> Result<Self, Self::Error> {
        let (name, description, points) = value;
        if name.trim().is_empty() || description.trim().is_empty() || points < 0 {
            return Err(AppError::InvalidValue);
        }
        Ok(Self {
            name,
            description,
            points,
        })
    }
}

impl TryFrom<(Status, i32)> for Progress {
    type Error = AppError;

    fn try_from(value: (Status, i32)) -> Result<Self, Self::Error> {
        let (status, playtime_minutes) = value;
        if playtime_minutes < 0 {
            return Err(AppError::InvalidValue);
        }
        Ok(Self {
            status,
            playtime_minutes,
        })
    }
}
