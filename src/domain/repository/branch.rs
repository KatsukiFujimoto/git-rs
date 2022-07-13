use crate::domain::entity::branch::Branch;

pub trait BranchRepository {
    fn get_branches(&self) -> anyhow::Result<Vec<Branch>>;
}
