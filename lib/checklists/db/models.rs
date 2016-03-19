use pgtypes::checklists::ChecklistType as DbChecklistType;
use pgtypes::requirements::{RequirementType, FieldType};
use requirements::{Field, Type, RequirementModel};
use super::super::types::{GroupedResult, ChecklistStatus};
use user::UserInfo;
use pgx::{FromRow, queryx};
use postgres::rows::Row;
use postgres::Connection;
use postgres::error::Error;
use postgres_array::Array;
use std::str::FromStr;

#[derive(Debug)]
pub struct ChecklistRow {
    pub id: i32,
    pub program_id: i32,
    pub checklist_type: DbChecklistType,
    pub checklist_id: Option<i32>,
    pub requirements_id: Option<i32>,
    pub req_name: Option<String>,
    pub req_field: Option<FieldType>,
    pub req_type: Option<RequirementType>,
    pub req_args: Option<Array<String>>,
}

impl FromRow for ChecklistRow {
    fn from_row(row: &Row) -> ChecklistRow {
        ChecklistRow {
            id: row.get(0),
            program_id: row.get(1),
            checklist_type: row.get(2),
            checklist_id: row.get(3),
            requirements_id: row.get(4),
            req_name: row.get(5),
            req_field: row.get(6),
            req_type: row.get(7),
            req_args: row.get(8),
        }
    }
}

pub struct ChecklistModel {
    pub id: i32,
    pub program_id: i32,
    pub parent_checklist_id: Option<i32>,
    pub hierarchy: ChecklistHierarchy,
}

pub enum ChecklistHierarchy {
    Or(Vec<ChecklistModel>),
    And(Vec<ChecklistModel>),
    Requirement(RequirementModel),
}

impl ChecklistHierarchy {
    pub fn check(&self, info: &UserInfo) -> GroupedResult {
        match *self {
            ChecklistHierarchy::Requirement(ref req) => {
                let status = req.check(info);
                GroupedResult::Requirement(ChecklistStatus::new(req.name.clone(), status))
            }
            ChecklistHierarchy::Or(ref v) => {
                GroupedResult::Or(v.iter().map(|r| r.check(info)).collect())
            }
            ChecklistHierarchy::And(ref v) => {
                GroupedResult::And(v.iter().map(|r| r.check(info)).collect())
            }

        }
    }
}

const CHECKLIST_QUERY: &'static str = "select c.id, c.program_id, c.check_type, c.checklist_id, \
                                       c.requirements_id, r.name, r.field, r.req_type, r.req_args \
                                       from checklists c left outer join requirements r on \
                                       c.requirements_id = r.id where c.program_id = $1;";

fn create_req(node: &ChecklistRow) -> RequirementModel {
    let args = node.req_args.as_ref().unwrap();
    let mut req_iter = args.iter();
    let req_type_enum = node.req_type.as_ref().unwrap();
    let field_enum = node.req_field.as_ref().unwrap();
    let req_id = node.requirements_id.unwrap();

    let req_type = match req_type_enum {
        &RequirementType::Boolean => {
            let value = bool::from_str(&req_iter.next().expect("No bool arg"))
                            .ok()
                            .unwrap_or(false);
            Type::Boolean(value)
        }
        &RequirementType::IntRange => {
            let start = u32::from_str(&req_iter.next().expect("No start arg"))
                            .ok()
                            .unwrap_or(0);
            let end = u32::from_str(&req_iter.next().expect("No end arg"))
                          .ok()
                          .unwrap_or(0);
            Type::IntRange(start, end)
        }
        &RequirementType::IntEquals => {
            let value = u32::from_str(&req_iter.next().expect("No int arg"))
                            .ok()
                            .unwrap_or(0);
            Type::IntEquals(value)
        }
        &RequirementType::StringEquals => {
            Type::StringEquals(req_iter.next().expect("No string arg").clone())
        }
    };
    RequirementModel::new(req_id,
                          &node.req_name.as_ref().unwrap(),
                          Field::from(field_enum),
                          req_type)
}

fn create_hierarchy_from_node(node: &ChecklistRow, rows: &Vec<ChecklistRow>) -> ChecklistModel {
    if node.checklist_type == DbChecklistType::Req {
        return ChecklistModel {
            id: node.id,
            program_id: node.program_id,
            parent_checklist_id: node.checklist_id,
            hierarchy: ChecklistHierarchy::Requirement(create_req(node)),
        };
    }
    let mut conditions = Vec::new();
    for row in rows.iter() {
        if let Some(row_chk_id) = row.checklist_id {
            if row_chk_id == node.id {
                conditions.push(create_hierarchy_from_node(row, rows));
            }
        }
    }
    match node.checklist_type {
        DbChecklistType::And => {
            ChecklistModel {
                id: node.id,
                program_id: node.program_id,
                parent_checklist_id: node.checklist_id,
                hierarchy: ChecklistHierarchy::And(conditions),
            }
        }
        DbChecklistType::Or => {
            ChecklistModel {
                id: node.id,
                program_id: node.program_id,
                parent_checklist_id: node.checklist_id,
                hierarchy: ChecklistHierarchy::Or(conditions),
            }
        }
        _ => unreachable!(),
    }
}

impl ChecklistModel {

    pub fn check(&self, info: &UserInfo) -> GroupedResult {
        self.hierarchy.check(info)
    }
    pub fn for_program<'a>(conn: &'a Connection,
                           program_id: i32)
                           -> Result<Vec<ChecklistModel>, Error> {
        let mut ch = Vec::new();
        let stmt = try!(conn.prepare(CHECKLIST_QUERY));
        let iter = try!(queryx::<ChecklistRow>(&stmt, &[&program_id]));
        let rows = iter.collect::<Vec<_>>();
        for row in rows.iter() {
            if let None = row.checklist_id {
                ch.push(create_hierarchy_from_node(&row, &rows));
            }
        }
        Ok(ch)
    }
}
