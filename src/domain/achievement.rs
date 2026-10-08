use crate::app_error::AppError;

#[derive(Clone)]
pub struct Achievement {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub description: String,
    pub points: i32,
}

#[derive(Debug)]
pub struct NewAchievement {
    name: String,
    description: String,
    points: i32,
}

impl NewAchievement {
    pub fn new(name: String, description: String, points: i32) -> Result<Self, AppError> {
        if name.trim().is_empty() || description.trim().is_empty() || points < 0 {
            return Err(AppError::InvalidValue);
        }
        Ok(Self {
            name,
            description,
            points,
        })
    }

    pub fn into_parts(self) -> (String, String, i32) {
        (self.name, self.description, self.points)
    }
}
