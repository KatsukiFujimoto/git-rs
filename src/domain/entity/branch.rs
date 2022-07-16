#[derive(Debug, PartialEq, Clone)]
pub struct Branch {
    pub name: String,
    pub branch_type: BranchType,
}

impl Branch {
    pub fn branch_type(&self) -> String {
        match self.branch_type {
            BranchType::Local => "local".into(),
            BranchType::Remote => "remote".into(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
        assert_eq!(branch_local.branch_type(), "local".to_string());
        assert_eq!(branch_remote.branch_type(), "remote".to_string());
    }
}
