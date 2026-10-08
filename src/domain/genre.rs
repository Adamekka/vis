#[derive(Clone)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct NewGenre {
    pub(super) name: String,
}

impl NewGenre {
    pub fn into_name(self) -> String {
        self.name
    }
}
