
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
