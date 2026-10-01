pub struct Game {
    pub id: i64,
    pub title: String,
    pub release_date: String,
    pub genres: Vec<String>,
}

impl Game {
    pub fn validate_release_date(release_date: &str) -> Result<(), crate::app_error::AppError> {
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
            || chrono::NaiveDate::parse_from_str(release_date, "%Y-%m-%d").is_err()
        {
            return Err(crate::app_error::AppError::InvalidDate);
        }
        Ok(())
    }
}
