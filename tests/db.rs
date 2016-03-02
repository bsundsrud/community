use community::models::establish_connection;
use community::finder::db;

#[test]
fn get_requirement() {
    let conn = establish_connection();
    let reqs = db::RequirementModel::get_all_requirements(&conn).unwrap();
    println!("{:?}", reqs);
    assert_eq!(reqs.len(), 2);
    for req in reqs {
        println!("{:?}", req);
    }

}

#[test]
fn get_checklist() {
    let conn = establish_connection();
    let c = db::checklist::ChecklistModel::to_checklist(&conn, 1);
    println!("{:?}", c);
    assert!(false);
}
