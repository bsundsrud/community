extern crate community;
use community::user::UserInfo;
use community::finder::req::{Req, Field, Type};
use community::finder::checklist::Checklist;

fn main() {
    let info = UserInfo {
        age: Some(20),
        county: Some("Stearns".to_string()),
        child_info: None,
        child_count: Some(0),
        annual_income: None,
        single_parent: Some(true),
    };

    let r = Req::new("age range", Field::Age, Type::IntRange(10, 30));

    let r1 = Req::new("has 1-3 children",
                      Field::ChildrenCount,
                      Type::IntRange(1, 3));

    let r2 = Req::new("in Dakota county",
                      Field::County,
                      Type::StringEquals(String::from("Dakota")));



    let mut chk = Checklist::new();
    chk.add_or_requirements(vec![r, r1]);
    chk.add_requirement(r2);

    let res = chk.check(&info);
    println!("{:?}", info);
    for r in res {
        println!("{:?} {:?}", r.status(), r);
    }
}
