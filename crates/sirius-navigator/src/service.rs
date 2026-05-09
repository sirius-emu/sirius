use sirius_repository::Repository;

pub struct NavigatorService {
    _repo: Repository,
}

impl NavigatorService {
    pub fn new(repo: Repository) -> Self {
        Self { _repo: repo }
    }
}
