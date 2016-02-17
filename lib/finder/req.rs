use user::UserInfo;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Met,
    NotMet,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum Field {
    Age,
    County,
    ChildrenCount,
    Income,
    SingleParent,
}

#[derive(Debug)]
pub enum Type {
    Boolean(bool),
    IntRange(u32, u32),
    IntEquals(u32),
    StringEquals(String),
}

#[derive(Debug)]
pub struct Req {
    pub name: String,
    pub field: Field,
    pub req_type: Type,
}

fn check_int_range(val: &Option<u32>, start: u32, end: u32) -> Status {
    if let Some(x) = *val {
        if x >= start && x <= end {
            Status::Met
        } else {
            Status::NotMet
        }
    } else {
        Status::Unknown
    }
}

fn check_int_equals(val: &Option<u32>, other: u32) -> Status {
    if let Some(i) = *val {
        if other == i {
            Status::Met
        } else {
            Status::NotMet
        }
    } else {
        Status::Unknown
    }
}

fn check_boolean(val: &Option<bool>, other: bool) -> Status {
    if let Some(b) = *val {
        if other == b {
            Status::Met
        } else {
            Status::NotMet
        }
    } else {
        Status::Unknown
    }
}

fn check_string_equals(val: &Option<String>, other: &str) -> Status {
    if let Some(ref s) = *val {
        if other == s {
            Status::Met
        } else {
            Status::NotMet
        }
    } else {
        Status::Unknown
    }
}

impl Req {
    pub fn new(name: &str, field: Field, req_type: Type) -> Req {
        Req {
            name: String::from(name),
            field: field,
            req_type: req_type,
        }
    }

    pub fn check(&self, info: &UserInfo) -> Status {
        match self.field {
            Field::Age => {
                match self.req_type {
                    Type::IntRange(start, end) => check_int_range(&info.age, start, end),
                    _ => unimplemented!(),
                }
            }
            Field::County => {
                match self.req_type {
                    Type::StringEquals(ref s) => check_string_equals(&info.county, s),
                    _ => unimplemented!(),
                }
            }
            Field::ChildrenCount => {
                match self.req_type {
                    Type::IntEquals(i) => check_int_equals(&info.child_count, i),
                    Type::IntRange(start, end) => check_int_range(&info.child_count, start, end),
                    _ => unimplemented!(),
                }
            }
            Field::Income => {
                match self.req_type {
                    Type::IntRange(start, end) => check_int_range(&info.annual_income, start, end),
                    _ => unimplemented!(),
                }
            }
            Field::SingleParent => {
                match self.req_type {
                    Type::Boolean(b) => check_boolean(&info.single_parent, b),
                    _ => unimplemented!(),
                }
            }
        }
    }
}
