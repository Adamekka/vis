use std::{cell::RefCell, rc::Rc};

use crate::app_error::AppError;

use super::User;

/// The app-wide signed-in user. It is created once at startup and injected into services; clones share
/// one state, so every holder sees the same sign-in and sign-out.
#[derive(Clone, Default)]
pub struct CurrentUser(Rc<RefCell<Option<User>>>);

impl CurrentUser {
    pub fn user(&self) -> Result<User, AppError> {
        self.0.borrow().clone().ok_or(AppError::SignInRequired)
    }

    pub fn sign_in(&self, user: User) {
        *self.0.borrow_mut() = Some(user);
    }

    pub fn sign_out(&self) {
        *self.0.borrow_mut() = None;
    }
}
