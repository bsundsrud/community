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

impl Req {
    pub fn new(name: &str, field: Field, req_type: Type) -> Req {
        Req {
            name: String::from(name),
            field: field,
            req_type: req_type,
        }
    }

    pub fn check(&self, info: &UserInfo) -> Status {
        fields!(self,
            Field::Age => requirements!(self,
                Type::IntRange(start, end) => int_range!(info.age, start, end)
            ),
            Field::County => requirements!(self,
                Type::StringEquals(ref s) => string_equals!(info.county, s)
            ),
            Field::ChildrenCount => requirements!(self,
                Type::IntEquals(i) => int_equals!(info.child_count, i),
                Type::IntRange(start, end) => int_range!(info.child_count, start, end)
            ),
            Field::Income => requirements!(self,
                Type::IntRange(start, end) => int_range!(info.annual_income, start, end)
            ),
            Field::SingleParent => requirements!(self,
                Type::Boolean(b) => boolean!(info.single_parent, b)
            )
        )
    }
}
