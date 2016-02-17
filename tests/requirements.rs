use community::finder::req::{Req, Field, Type, Status};
use community::user::UserInfo;

#[test]
fn test_age_field() {
    let r1 = Req::new("", Field::Age, Type::IntRange(5, 10));
    let mut info = UserInfo::new();

    info.set_age(10);
    assert_eq!(Status::Met, r1.check(&info));

    info.set_age(5);
    assert_eq!(Status::Met, r1.check(&info));

    info.set_age(4);
    assert_eq!(Status::NotMet, r1.check(&info));

    info.set_age(11);
    assert_eq!(Status::NotMet, r1.check(&info));

    info.age = None;
    assert_eq!(Status::Unknown, r1.check(&info));

}

#[test]
fn test_county_field() {
    let r1 = Req::new("", Field::County, Type::StringEquals("TestVal".to_string()));
    let mut info = UserInfo::new();
    assert_eq!(Status::Unknown, r1.check(&info));

    info.set_county("something");
    assert_eq!(Status::NotMet, r1.check(&info));

    info.set_county("TestVal");
    assert_eq!(Status::Met, r1.check(&info));

}

#[test]
fn test_child_count_field() {
    let r1 = Req::new("", Field::ChildrenCount, Type::IntEquals(0));
    let r2 = Req::new("", Field::ChildrenCount, Type::IntRange(1, 3));

    let mut info = UserInfo::new();
    assert_eq!(Status::Unknown, r1.check(&info));
    assert_eq!(Status::Unknown, r2.check(&info));

    info.set_child_count(0);
    assert_eq!(Status::Met, r1.check(&info));
    assert_eq!(Status::NotMet, r2.check(&info));

    info.set_child_count(2);
    assert_eq!(Status::NotMet, r1.check(&info));
    assert_eq!(Status::Met, r2.check(&info));
}

#[test]
fn test_income_field() {
    let r1 = Req::new("", Field::Income, Type::IntRange(10_000, 20_000));
    let mut info = UserInfo::new();

    assert_eq!(Status::Unknown, r1.check(&info));

    info.set_annual_income(5000);
    assert_eq!(Status::NotMet, r1.check(&info));

    info.set_annual_income(15_000);
    assert_eq!(Status::Met, r1.check(&info));

    info.set_annual_income(25_000);
    assert_eq!(Status::NotMet, r1.check(&info));
}

#[test]
fn test_single_parent() {
    let r1 = Req::new("", Field::SingleParent, Type::Boolean(true));
    let mut info = UserInfo::new();

    assert_eq!(Status::Unknown, r1.check(&info));

    info.set_single_parent(true);
    assert_eq!(Status::Met, r1.check(&info));

    info.set_single_parent(false);
    assert_eq!(Status::NotMet, r1.check(&info));
}
