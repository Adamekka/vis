//! Role rules: admins may do everything; players may read and manage only their own library.

use crate::app_error::AppError;

use super::{Action, Email, Operation, Player, PlayerListing, Role, User};

impl User {
    /// Whether the role may use this kind of action at all; player-scoped input is checked by `authorize`.
    pub fn may_use(&self, action: Action) -> bool {
        match self.role {
            Role::Admin => true,
            Role::Player { .. } => !matches!(
                action,
                Action::AddPlayer
                    | Action::AddGame
                    | Action::AddGameToLibrary
                    | Action::AddGenre
                    | Action::TagGame
                    | Action::AddAchievement
            ),
        }
    }

    pub fn authorize(&self, operation: &Operation) -> Result<(), AppError> {
        if !self.may_use(operation.action()) {
            return Err(AppError::Forbidden);
        }
        let Role::Player { player_id } = self.role else {
            return Ok(());
        };
        let target = match operation {
            Operation::AddToLibrary { player_id, .. }
            | Operation::Library { player_id }
            | Operation::SetProgress { player_id, .. }
            | Operation::Unlock { player_id, .. }
            | Operation::Unlocked { player_id } => *player_id,
            _ => return Ok(()),
        };
        if target == player_id {
            Ok(())
        } else {
            Err(AppError::Forbidden)
        }
    }

    pub fn view_player(&self, player: Player) -> PlayerListing {
        let visible = match self.role {
            Role::Admin => true,
            Role::Player { player_id } => player_id == player.id,
        };
        PlayerListing {
            id: player.id,
            username: player.username,
            email: if visible {
                Email::Visible(player.email)
            } else {
                Email::Hidden
            },
        }
    }
}
