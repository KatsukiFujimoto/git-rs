use crate::domain::{entity::branch::Branch, repository::branch::BranchRepository};

pub struct GetAllBranches<'a> {
    repo: &'a dyn BranchRepository,
}

impl<'a> GetAllBranches<'a> {
    pub fn new(repo: &'a dyn BranchRepository) -> Self {
        Self { repo }
    }

    pub fn run(&self) -> anyhow::Result<Vec<Branch>> {
        todo!()
    }
}
