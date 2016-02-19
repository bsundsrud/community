use community::finder::req::{Req, Field, Type, Status};
use community::user::UserInfo;
use community::finder::checklist::{Checklist, ChecklistType, ChecklistStatus, GroupedResult};

#[test]
fn test_simple_checklist_type() {
    let r = Req::new("", Field::Age, Type::IntRange(15, 30));
    let ch = ChecklistType::Requirement(r);
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
    let r1 = Req::new("", Field::Age, Type::IntRange(15, 30));
    let r2 = Req::new("", Field::SingleParent, Type::Boolean(true));
    let ch = ChecklistType::Or(vec![ChecklistType::Requirement(r1),
                                    ChecklistType::Requirement(r2)]);
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
    let r1 = Req::new("", Field::Age, Type::IntRange(15, 30));
    let r2 = Req::new("", Field::SingleParent, Type::Boolean(true));
    let ch = ChecklistType::And(vec![ChecklistType::Requirement(r1),
                                     ChecklistType::Requirement(r2)]);
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
    use community::finder::checklist::ChecklistType::*;
    let r1 = Req::new("", Field::Age, Type::IntRange(15, 30));
    let r2 = Req::new("", Field::SingleParent, Type::Boolean(false));
    let r3 = Req::new("", Field::SingleParent, Type::Boolean(true));
    let ch = Or(vec![And(vec![Requirement(r1), Requirement(r2)]), Requirement(r3)]);
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
    let mut ch = Checklist::new();
    let r1 = Req::new("", Field::Age, Type::IntRange(15, 30));
    let r2 = Req::new("", Field::SingleParent, Type::Boolean(true));
    let r3 = Req::new("", Field::ChildrenCount, Type::IntRange(1, 3));
    let r4 = Req::new("", Field::County, Type::StringEquals("Dakota".to_string()));
    let r5 = Req::new("",
                      Field::County,
                      Type::StringEquals("Hennepin".to_string()));

    // Age 15-30, Single Parent AND 1-3 children, County Dakota OR Hennepin
    // results will come out in insertion order
    ch.add_requirement(r1);
    ch.add_and_requirements(vec![r2, r3]);
    ch.add_or_requirements(vec![r4, r5]);
    let mut info = UserInfo::new();
    let mut gr = ch.check(&info);
    {
        let mut iter = gr.iter();
        assert_eq!(iter.next().unwrap().status(), Status::Unknown);
        assert_eq!(iter.next().unwrap().status(), Status::Unknown);
        assert_eq!(iter.next().unwrap().status(), Status::Unknown);
        assert!(iter.next().is_none());
    }


    info.set_age(20);
    gr = ch.check(&info);

    {
        let mut iter = gr.iter();
        assert_eq!(iter.next().unwrap().status(), Status::Met);
        assert_eq!(iter.next().unwrap().status(), Status::Unknown);
        assert_eq!(iter.next().unwrap().status(), Status::Unknown);
    }

    info.set_county("Dakota");
    gr = ch.check(&info);
    {
        let mut iter = gr.iter();

        assert_eq!(iter.next().unwrap().status(), Status::Met);
        assert_eq!(iter.next().unwrap().status(), Status::Unknown);
        assert_eq!(iter.next().unwrap().status(), Status::Met);
    }

    info.set_single_parent(false);
    gr = ch.check(&info);
    {
        let mut iter = gr.iter();
        assert_eq!(iter.next().unwrap().status(), Status::Met);
        assert_eq!(iter.next().unwrap().status(), Status::NotMet);
        assert_eq!(iter.next().unwrap().status(), Status::Met);
    }

    info.set_single_parent(true).set_child_count(2);
    gr = ch.check(&info);
    {
        let mut iter = gr.iter();
        assert_eq!(iter.next().unwrap().status(), Status::Met);
        assert_eq!(iter.next().unwrap().status(), Status::Met);
        assert_eq!(iter.next().unwrap().status(), Status::Met);
    }
}
