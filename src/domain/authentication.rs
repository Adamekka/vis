use crate::app_error::AppError;

use super::{Role, User};

const ACCOUNTS: [(&str, &str, Role); 3] = [
    ("admin", "admin", Role::Admin),
    ("alex", "alex", Role::Player { player_id: 1 }),
    ("sam", "sam", Role::Player { player_id: 2 }),
];

pub fn authenticate(username: &str, password: &str) -> Result<User, AppError> {
    ACCOUNTS
        .iter()
        .find(|(name, secret, _)| *name == username && *secret == password)
        .map(|&(name, _, role)| User {
            username: name.to_owned(),
            role,
        })
        .ok_or(AppError::InvalidCredentials)
}
