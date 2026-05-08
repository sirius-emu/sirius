//! Authentication and handshake logic.

use sirius_error::AuthError;
use sirius_repository::Repository;
use sirius_repository::models::User;

pub async fn authenticate(
    ticket: String,
    repo: &Repository,
) -> Result<User, AuthError> {
    let user = repo
        .users
        .find_by_auth_ticket(&ticket)
        .await
        .map_err(|_| AuthError::InvalidTicket)?;

    repo.users
        .consume_auth_ticket(user.id)
        .await
        .map_err(|_| AuthError::InternalError)?;

    Ok(user)
}
