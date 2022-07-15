use crate::domain::entity::branch::Branch;

#[cfg_attr(test, mockall::automock)]
pub trait BranchRepository {
    fn get_branches(&self) -> anyhow::Result<Vec<Branch>>;
    fn delete_branch(&self, branch: Branch) -> anyhow::Result<Branch>;
}
