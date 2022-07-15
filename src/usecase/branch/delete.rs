use crate::domain::{entity::branch::Branch, repository::branch::BranchRepository};

pub struct DeleteBranch<'a> {
    repo: &'a dyn BranchRepository,
}

impl<'a> DeleteBranch<'a> {
    pub fn new(repo: &'a dyn BranchRepository) -> Self {
        Self { repo }
    }

    pub fn run(&self, branch: Branch) -> anyhow::Result<Branch> {
        self.repo.delete_branch(branch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        entity::fixtures::branch_sample, repository::branch::MockBranchRepository,
    };
    use assert_matches::assert_matches;
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_run() {
        let branch = branch_sample();
        let mut repo = MockBranchRepository::new();
        repo.expect_delete_branch()
            .times(1)
            .with(eq(branch.clone()))
            .returning(|x| Ok(x));
        let usecase = DeleteBranch::new(&repo);
        assert_matches!(usecase.run(branch.clone()), Ok(x) => { assert_eq!(x, branch) })
    }
}
