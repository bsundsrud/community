
#[derive(Debug)]
pub struct UserInfo {
    pub age: Option<u32>,
    pub county: Option<String>,
    pub child_count: Option<u32>,
    pub child_info: Option<Vec<UserInfo>>,
    pub annual_income: Option<u32>,
    pub single_parent: Option<bool>,
}
