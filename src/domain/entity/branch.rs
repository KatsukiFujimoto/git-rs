#[derive(Debug, PartialEq, Clone)]
pub struct Branch {
    pub name: String,
    pub branch_type: BranchType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BranchType {
    Local,
    Remote,
}
