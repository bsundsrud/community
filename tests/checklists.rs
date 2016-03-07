use community::finder::requirements::{RequirementModel, Field, Type, Status};
use community::user::UserInfo;
use community::finder::checklists::{ChecklistModel, ChecklistHierarchy};


fn fake_checklist(req: RequirementModel) -> ChecklistModel {
    ChecklistModel {
        id: 1,
        program_id: 1,
        parent_checklist_id: None,
        hierarchy: ChecklistHierarchy::Requirement(req),
    }
}

fn fake_and_from_checklist(ch: Vec<ChecklistModel>) -> ChecklistModel {
    ChecklistModel {
        id: 1,
        program_id: 1,
        parent_checklist_id: None,
        hierarchy: ChecklistHierarchy::And(ch),
    }
}

fn fake_or_from_checklist(ch: Vec<ChecklistModel>) -> ChecklistModel {
    ChecklistModel {
        id: 1,
        program_id: 1,
        parent_checklist_id: None,
        hierarchy: ChecklistHierarchy::Or(ch),
    }
}

#[test]
fn test_simple_checklist_type() {
    let r = RequirementModel::new(1, "", Field::Age, Type::IntRange(15, 30));
    let ch = ChecklistHierarchy::Requirement(r);
    let mut info = UserInfo::new();
    let mut gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown);
    info.set_age(20);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Met);
    info.set_age(40);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::NotMet);
}

#[test]
fn test_simple_or_checklist_type() {
    let r1 = RequirementModel::new(1, "", Field::Age, Type::IntRange(15, 30));
    let r2 = RequirementModel::new(1, "", Field::SingleParent, Type::Boolean(true));
    let ch = ChecklistHierarchy::Or(vec![fake_checklist(r1), fake_checklist(r2)]);
    let mut info = UserInfo::new();
    let mut gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown); // Unknown OR Unknown = Unknown
    info.set_age(40);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown); // Unknown OR NotMet = Unknown
    info.set_age(20);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Met); // Met OR Unknown = Met
    info.set_single_parent(true);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Met); // Met OR Met = Met
    info.set_single_parent(false);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Met); // Met OR NotMet = Met
    info.set_age(40);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::NotMet); // NotMet OR NotMet = NotMet
}

#[test]
fn test_simple_and_checklist_type() {
    let r1 = RequirementModel::new(1, "", Field::Age, Type::IntRange(15, 30));
    let r2 = RequirementModel::new(1, "", Field::SingleParent, Type::Boolean(true));
    let ch = ChecklistHierarchy::And(vec![fake_checklist(r1), fake_checklist(r2)]);
    let mut info = UserInfo::new();
    let mut gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown); // Unknown AND Unknown = Unknown
    info.set_age(40);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::NotMet); // Unknown AND NotMet = NotMet
    info.set_age(20);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown); // Met AND Unknown = Unknown
    info.set_single_parent(true);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Met); // Met AND Met = Met
    info.set_single_parent(false);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::NotMet); // Met AND NotMet = NotMet
    info.set_age(40);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::NotMet); // NotMet AND NotMet = NotMet
}

#[test]
fn test_nested_checklist_types() {
    use community::finder::checklists::ChecklistHierarchy::*;
    let r1 = RequirementModel::new(1, "", Field::Age, Type::IntRange(15, 30));
    let r2 = RequirementModel::new(1, "", Field::SingleParent, Type::Boolean(false));
    let r3 = RequirementModel::new(1, "", Field::SingleParent, Type::Boolean(true));
    let ch = Or(vec![fake_and_from_checklist(vec![fake_checklist(r1), fake_checklist(r2)]),
                     fake_checklist(r3)]);
    let mut info = UserInfo::new();
    let mut gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown); // (U & U) | U = U)
    info.set_age(40);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown); // (N & U) | U = U
    info.set_age(20);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown); // (M & U) | U = U
    info.set_single_parent(true);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Met); // (M & N) | M = M
    info.set_single_parent(false);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Met); // (M & M) | N = M
    info.set_age(40);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::NotMet); // (M & N) | N = N
}

#[test]
fn test_checklists() {
    let r1 = RequirementModel::new(1, "", Field::Age, Type::IntRange(15, 30));
    let r2 = RequirementModel::new(1, "", Field::SingleParent, Type::Boolean(true));
    let r3 = RequirementModel::new(1, "", Field::ChildrenCount, Type::IntRange(1, 3));
    let r4 = RequirementModel::new(1,
                                   "",
                                   Field::County,
                                   Type::StringEquals("Dakota".to_string()));
    let r5 = RequirementModel::new(1,
                                   "",
                                   Field::County,
                                   Type::StringEquals("Hennepin".to_string()));

    // Age 15-30, Single Parent AND 1-3 children, County Dakota OR Hennepin
    // results will come out in insertion order
    let ch1 = fake_checklist(r1);
    let ch2 = fake_and_from_checklist(vec![fake_checklist(r2), fake_checklist(r3)]);
    let ch3 = fake_or_from_checklist(vec![fake_checklist(r4), fake_checklist(r5)]);
    let ch = fake_and_from_checklist(vec![ch1, ch2, ch3]);
    let mut info = UserInfo::new();
    let mut gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown);

    info.set_age(20);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown);

    info.set_county("Dakota");
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Unknown);

    info.set_single_parent(false);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::NotMet);

    info.set_single_parent(true).set_child_count(2);
    gr = ch.check(&info);
    assert_eq!(gr.status(), Status::Met);
}
