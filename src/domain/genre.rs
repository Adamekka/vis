use crate::app_error::AppError;

#[derive(Clone)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct NewGenre {
    name: String,
}

impl NewGenre {
    pub fn new(name: String) -> Result<Self, AppError> {
        if name.trim().is_empty() {
            return Err(AppError::InvalidValue);
        }
        Ok(Self { name })
    }

    pub fn into_name(self) -> String {
        self.name
    }
}
