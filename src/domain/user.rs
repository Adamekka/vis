use super::Role;

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    pub role: Role,
}
