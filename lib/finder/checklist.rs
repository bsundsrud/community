use finder::req::{Req, Status};
use user::UserInfo;

#[derive(Debug)]
pub struct Checklist {
    requirements: Vec<ChecklistType>,
}
#[derive(Debug)]
pub enum ChecklistType {
    Or(Vec<ChecklistType>),
    Requirement(Req),
    And(Vec<ChecklistType>),
}

impl ChecklistType {
    pub fn check(&self, info: &UserInfo) -> GroupedResult {
        match *self {
            ChecklistType::Requirement(ref req) => {
                let status = req.check(info);
                GroupedResult::Requirement(ChecklistStatus::new(req.name.clone(), status))
            }
            ChecklistType::Or(ref v) => {
                GroupedResult::Or(v.iter().map(|r| r.check(info)).collect())
            }
            ChecklistType::And(ref v) => {
                GroupedResult::And(v.iter().map(|r| r.check(info)).collect())
            }

        }
    }
}

#[derive(Debug)]
pub struct ChecklistStatus {
    pub name: String,
    pub status: Status,
}

impl ChecklistStatus {
    fn new(name: String, status: Status) -> ChecklistStatus {
        ChecklistStatus {
            name: name,
            status: status,
        }
    }
}
#[derive(Debug)]
pub enum GroupedResult {
    Or(Vec<GroupedResult>),
    Requirement(ChecklistStatus),
    And(Vec<GroupedResult>),
}

impl GroupedResult {
    pub fn status(&self) -> Status {
        match *self {
            GroupedResult::Requirement(ref req) => req.status,
            GroupedResult::Or(ref v) => {
                v.iter().fold(Status::NotMet, |current_status, r| {
                    match r.status() {
                        Status::Met => Status::Met,
                        Status::Unknown if current_status != Status::Met => Status::Unknown,
                        _ => current_status,
                    }
                })
            }
            GroupedResult::And(ref v) => {
                v.iter().fold(Status::Met, |current_status, r| {
                    match (r.status(), current_status) {
                        (Status::NotMet, _) => Status::NotMet,
                        (_, Status::NotMet) => Status::NotMet,
                        (Status::Unknown, _) => Status::Unknown,
                        (_, Status::Unknown) => Status::Unknown,
                        (Status::Met, Status::Met) => Status::Met,
                    }
                })
            }
        }
    }
}

impl Checklist {
    pub fn new() -> Checklist {
        Checklist { requirements: Vec::new() }
    }
    pub fn add_requirement(&mut self, req: Req) {
        self.add_checklist(ChecklistType::Requirement(req));
    }

    pub fn add_checklist(&mut self, chk: ChecklistType) {
        self.requirements.push(chk);
    }

    pub fn add_or_requirements(&mut self, v: Vec<Req>) {
        self.add_checklist(ChecklistType::Or(v.into_iter()
                                              .map(|r| ChecklistType::Requirement(r))
                                              .collect()));
    }

    pub fn add_and_requirements(&mut self, v: Vec<Req>) {
        self.add_checklist(ChecklistType::And(v.into_iter()
                                               .map(|r| ChecklistType::Requirement(r))
                                               .collect()));
    }

    pub fn check(&self, info: &UserInfo) -> Vec<GroupedResult> {
        let mut results = Vec::new();
        for c in self.requirements.iter() {
            results.push(c.check(info));
        }
        results
    }
}
