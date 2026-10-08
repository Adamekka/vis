pub struct Game {
    pub id: i64,
    pub title: String,
    pub release_date: String,
    pub genres: Vec<String>,
}

#[derive(Debug)]
pub struct NewGame {
    pub(super) title: String,
    pub(super) release_date: String,
}

impl NewGame {
    pub fn into_parts(self) -> (String, String) {
        (self.title, self.release_date)
    }
}
