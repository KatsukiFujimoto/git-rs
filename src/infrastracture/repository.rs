use git2::Repository;

pub mod branch;

pub struct GitRepository<'repo> {
    git_repo: &'repo Repository,
}

impl<'repo> GitRepository<'repo> {
    pub fn new(git_repo: &'repo Repository) -> Self {
        Self { git_repo }
    }
}
