use crate::domain::{entity::branch::Branch, repository::branch::BranchRepository};

pub struct GetAllBranches<'a> {
    repo: &'a dyn BranchRepository,
}

impl<'a> GetAllBranches<'a> {
    pub fn new(repo: &'a dyn BranchRepository) -> Self {
        Self { repo }
    }

    pub fn run(&self) -> anyhow::Result<Vec<Branch>> {
        self.repo.get_branches()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        entity::fixtures::branch_sample, repository::branch::MockBranchRepository,
    };
    use assert_matches::assert_matches;

    #[test]
    pub fn test_run() {
        let mut repo = MockBranchRepository::new();
        repo.expect_get_branches()
            .times(1)
            .returning(|| Ok(vec![branch_sample(), branch_sample()]));
        let usecase = GetAllBranches::new(&repo);
        let branches = usecase.run();
        assert_matches!(branches, Ok(x) => {
            assert_matches!(x[..], [_, _]);
        })
    }
}
