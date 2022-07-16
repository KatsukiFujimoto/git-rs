#[derive(Debug, PartialEq, Clone)]
pub struct Branch {
    pub name: String,
    pub branch_type: BranchType,
    pub current: bool,
}

impl Branch {
    pub fn branch_type(&self) -> String {
        self.branch_type.to_string()
    }
}

#[derive(Debug, PartialEq, Clone, strum_macros::Display)]
pub enum BranchType {
    Local,
    Remote,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::fixtures::branch_sample;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_branch_type() {
        let branch_local = Branch {
            branch_type: BranchType::Local,
            ..branch_sample()
        };
        let branch_remote = Branch {
            branch_type: BranchType::Remote,
            ..branch_sample()
        };
        assert_eq!(branch_local.branch_type(), "Local".to_string());
        assert_eq!(branch_remote.branch_type(), "Remote".to_string());
    }
}
