use sirius_repository::Repository;
use sirius_session::SessionManager;

#[derive(Debug, Clone)]
pub struct ServerContext {
    pub sessions: SessionManager,
    pub repository: Repository,
}
