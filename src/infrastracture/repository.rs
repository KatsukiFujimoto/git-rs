use git2::Repository;

pub mod branch;

pub struct GitRepository<'repo> {
    repository: &'repo Repository,
}

impl<'repo> GitRepository<'repo> {
    pub fn new(repository: &'repo Repository) -> Self {
        Self { repository }
    }
}
