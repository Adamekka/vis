#[derive(Clone)]
pub struct Player {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug)]
pub struct NewPlayer {
    pub(super) username: String,
    pub(super) email: String,
}

impl NewPlayer {
    pub fn into_parts(self) -> (String, String) {
        (self.username, self.email)
    }
}
