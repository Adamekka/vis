use crate::app_error::AppError;

pub struct Game {
    pub id: i64,
    pub title: String,
    pub release_date: String,
    pub genres: Vec<String>,
}

#[derive(Debug)]
pub struct NewGame {
    title: String,
    release_date: String,
}

impl NewGame {
    pub fn new(title: String, release_date: String) -> Result<Self, AppError> {
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

    pub fn into_parts(self) -> (String, String) {
        (self.title, self.release_date)
    }
}
