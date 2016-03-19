use pgx::{FromRow, queryx};
use super::super::types::{Field, Type, Status};
use postgres::rows::Row;
use postgres::Connection;
use postgres::error::Error;
use postgres_array::Array;
use std::str::FromStr;
use user::UserInfo;
use pgtypes::requirements::{FieldType, RequirementType};

#[derive(Debug)]
pub struct RequirementModel {
    pub id: i32,
    pub name: String,
    pub field_type: Field,
    pub req_type: Type,
}

impl RequirementModel {
    pub fn get_all_requirements<'a>(conn: &'a Connection) -> Result<Vec<RequirementModel>, Error> {
        let stmt = try!(conn.prepare("select id, name, field,req_type, req_args from \
                                      requirements;"));
        let iter = try!(queryx::<RequirementModel>(&stmt, &[]));
        Ok(iter.collect::<Vec<_>>())
    }
}

impl FromRow for RequirementModel {
    fn from_row(row: &Row) -> RequirementModel {
        let req_args: Array<String> = row.get(4);
        let req_type_enum: RequirementType = row.get(3);
        let field_enum: FieldType = row.get(2);
        let mut req_iter = req_args.into_iter();
        let req_type = match req_type_enum {
            RequirementType::Boolean => {
                let value = bool::from_str(&req_iter.next().expect("No bool arg"))
                                .ok()
                                .unwrap_or(false);
                Type::Boolean(value)
            }
            RequirementType::IntRange => {
                let start = u32::from_str(&req_iter.next().expect("No start arg"))
                                .ok()
                                .unwrap_or(0);
                let end = u32::from_str(&req_iter.next().expect("No end arg")).ok().unwrap_or(0);
                Type::IntRange(start, end)
            }
            RequirementType::IntEquals => {
                let value = u32::from_str(&req_iter.next().expect("No int arg")).ok().unwrap_or(0);
                Type::IntEquals(value)
            }
            RequirementType::StringEquals => {
                Type::StringEquals(req_iter.next().expect("No string arg").clone())
            }
        };
        let req_name: String = row.get(1);
        RequirementModel::new(row.get(0), &req_name, Field::from(field_enum), req_type)
    }
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

impl RequirementModel {
    pub fn new(id: i32, name: &str, field: Field, req_type: Type) -> RequirementModel {
        RequirementModel {
            id: id,
            name: String::from(name),
            field_type: field,
            req_type: req_type,
        }
    }

    pub fn check(&self, info: &UserInfo) -> Status {
        match self.field_type {
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
