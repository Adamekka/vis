use crate::app_error::AppError;

#[derive(Clone)]
pub struct Player {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug)]
pub struct NewPlayer {
    username: String,
    email: String,
}

impl NewPlayer {
    pub fn new(username: String, email: String) -> Result<Self, AppError> {
        if username.trim().is_empty() || email.trim().is_empty() {
            return Err(AppError::InvalidValue);
        }
        Ok(Self { username, email })
    }

    pub fn into_parts(self) -> (String, String) {
        (self.username, self.email)
    }
}
