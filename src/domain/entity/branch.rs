#[derive(Debug)]
pub struct Branch {
    pub name: String,
    pub branch_type: BranchType,
}

#[derive(Debug)]
pub enum BranchType {
    Local,
    Remote,
}
