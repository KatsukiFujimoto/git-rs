pub struct Branch {
    pub name: String,
    pub branch_type: BranchType,
}

pub enum BranchType {
    Local,
    Remote,
}
