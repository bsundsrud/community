
#[derive(Debug)]
pub struct UserInfo {
    pub age: Option<u32>,
    pub county: Option<String>,
    pub child_count: Option<u32>,
    pub child_info: Option<Vec<UserInfo>>,
    pub annual_income: Option<u32>,
    pub single_parent: Option<bool>,
}

impl UserInfo {
    pub fn new() -> UserInfo {
        UserInfo {
            age: None,
            county: None,
            child_count: None,
            child_info: None,
            annual_income: None,
            single_parent: None,
        }
    }
    pub fn set_age<'a>(&'a mut self, age: u32) -> &'a mut Self {
        self.age = Some(age);
        self
    }

    pub fn set_county<'a>(&'a mut self, county: &str) -> &'a mut Self {
        self.county = Some(county.to_string());
        self
    }

    // TODO: should probably just be updated by changing the child_info field
    pub fn set_child_count<'a>(&'a mut self, count: u32) -> &'a mut Self {
        self.child_count = Some(count);
        self
    }

    pub fn set_annual_income<'a>(&'a mut self, income: u32) -> &'a mut Self {
        self.annual_income = Some(income);
        self
    }

    pub fn set_single_parent<'a>(&'a mut self, value: bool) -> &'a mut Self {
        self.single_parent = Some(value);
        self
    }
}
