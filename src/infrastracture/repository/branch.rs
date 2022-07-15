use super::*;
use crate::domain::{
    entity::branch::{Branch, BranchType},
    repository::branch::BranchRepository,
};
use git2::{Branch as GitBranch, BranchType as GitBranchType};

impl<'repo> BranchRepository for GitRepository<'repo> {
    fn get_branches(&self) -> anyhow::Result<Vec<Branch>> {
        match self.repository.branches(Some(GitBranchType::Local)) {
            // TODO: branches.map()が失敗した場合のハンドリングをする
            Ok(branches) => Ok(branches.map(|x| x.unwrap().into()).collect()),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    fn delete_branch(&self, _branch: Branch) -> anyhow::Result<Branch> {
        todo!()
    }
}

impl<'repo> From<(GitBranch<'repo>, GitBranchType)> for Branch {
    fn from(x: (GitBranch, GitBranchType)) -> Self {
        let (branch, branch_type) = x;

        Self {
            // TODO: branch.name()が失敗した場合のハンドリングをする
            name: branch.name().unwrap().unwrap().to_string(),
            branch_type: branch_type.into(),
        }
    }
}

impl From<GitBranchType> for BranchType {
    fn from(x: GitBranchType) -> BranchType {
        match x {
            GitBranchType::Local => BranchType::Local,
            GitBranchType::Remote => BranchType::Remote,
        }
    }
}
