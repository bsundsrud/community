use pgx::{FromRow, queryx};
use super::req::{Field, Type};
use postgres::rows::Row;
use postgres::Connection;
use postgres::error::Error;
use postgres_array::Array;
use std::str::FromStr;
mod pgtypes;
pub mod checklist;
use self::pgtypes::{FieldType, RequirementType};

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
                let value = bool::from_str(&req_iter.next().expect("No bool arg")).ok().unwrap_or(false);
                Type::Boolean(value)
            }
            RequirementType::IntRange => {
                let start = u32::from_str(&req_iter.next().expect("No start arg")).ok().unwrap_or(0);
                let end = u32::from_str(&req_iter.next().expect("No end arg")).ok().unwrap_or(0);
                Type::IntRange(start, end)
            }
            RequirementType::IntEquals => {
                let value = u32::from_str(&req_iter.next().expect("No int arg")).ok().unwrap_or(0);
                Type::IntEquals(value)
            }
            RequirementType::StringEquals => Type::StringEquals(req_iter.next().expect("No string arg").clone()),
        };
        RequirementModel {
            id: row.get(0),
            name: row.get(1),
            field_type: Field::from(field_enum),
            req_type: req_type,
        }
    }
}
