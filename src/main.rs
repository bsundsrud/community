extern crate community;
use community::user::UserInfo;
use community::finder::requirements::{RequirementModel, Field, Type};
use community::finder::checklists::{ChecklistModel, ChecklistHierarchy};

fn main() {
    let info = UserInfo {
        age: Some(20),
        county: Some("Stearns".to_string()),
        child_info: None,
        child_count: Some(0),
        annual_income: None,
        single_parent: Some(true),
    };

    let r = RequirementModel::new(1, "age range", Field::Age, Type::IntRange(10, 30));

    let r1 = RequirementModel::new(2,
                                   "has 1-3 children",
                                   Field::ChildrenCount,
                                   Type::IntRange(1, 3));

    let r2 = RequirementModel::new(3,
                                   "in Dakota county",
                                   Field::County,
                                   Type::StringEquals(String::from("Dakota")));



    let chk = ChecklistModel {
        id: 1,
        program_id: 1,
        parent_checklist_id: None,
        hierarchy: ChecklistHierarchy::Or(vec![ChecklistModel {
                                                   id: 2,
                                                   program_id: 1,
                                                   parent_checklist_id: Some(1),
                                                   hierarchy: ChecklistHierarchy::Requirement(r),
                                               },
                                               ChecklistModel {
                                                   id: 3,
                                                   program_id: 1,
                                                   parent_checklist_id: Some(1),
                                                   hierarchy: ChecklistHierarchy::Requirement(r1),
                                               }]),
    };
    let chk2 = ChecklistModel {
        id: 4,
        program_id: 1,
        parent_checklist_id: None,
        hierarchy: ChecklistHierarchy::Requirement(r2),
    };

    let res = chk.check(&info);
    println!("{:?}", info);
    println!("{:?} {:?}", res.status(), res);
    let res2 = chk2.check(&info);
    println!("{:?} {:?}", res2.status(), res2);
}
