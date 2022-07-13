pub mod branch;

#[cfg(test)]
pub mod fixtures {
    use super::branch::{Branch, BranchType};

    pub fn branch_sample() -> Branch {
        Branch {
            name: "sample_branch".into(),
            branch_type: BranchType::Local,
        }
    }
}
