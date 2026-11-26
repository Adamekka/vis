/// A player as shown to the signed-in user, who may not be allowed to see the email address.
pub struct PlayerListing {
    pub id: i64,
    pub username: String,
    pub email: Email,
}

pub enum Email {
    Visible(String),
    Hidden,
}
