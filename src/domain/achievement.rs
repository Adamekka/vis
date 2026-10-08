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
    pub(super) name: String,
    pub(super) description: String,
    pub(super) points: i32,
}

impl NewAchievement {
    pub fn into_parts(self) -> (String, String, i32) {
        (self.name, self.description, self.points)
    }
}
