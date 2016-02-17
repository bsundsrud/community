use finder::req::{Req, Status};
use user::UserInfo;

#[derive(Debug)]
pub struct Checklist {
    requirements: Vec<ChecklistType>,
}
#[derive(Debug)]
pub enum ChecklistType {
    Or(Box<ChecklistType>, Box<ChecklistType>),
    Requirement(Req),
}

impl ChecklistType {
    pub fn check(&self, info: &UserInfo) -> ChecklistResult {
        match *self {
            ChecklistType::Or(ref req1, ref req2) => {
                let status1 = req1.check(info);
                let status2 = req2.check(info);
                ChecklistResult::Or(Box::new(status1), Box::new(status2))
            }
            ChecklistType::Requirement(ref req) => {
                let status = req.check(info);
                ChecklistResult::Requirement(Result::new(req.name.clone(), status))
            }
        }
    }
}

#[derive(Debug)]
pub struct Result {
    pub name: String,
    pub status: Status,
}

impl Result {
    fn new(name: String, status: Status) -> Result {
        Result {
            name: name,
            status: status,
        }
    }
}
#[derive(Debug)]
pub enum ChecklistResult {
    Or(Box<ChecklistResult>, Box<ChecklistResult>),
    Requirement(Result),
}

impl ChecklistResult {
    pub fn status(&self) -> Status {
        match *self {
            ChecklistResult::Or(ref res1, ref res2) => {
                let s1 = res1.status();
                let s2 = res2.status();
                if s1 == Status::Met || s2 == Status::Met {
                    Status::Met
                } else if s1 == Status::Unknown || s2 == Status::Unknown {
                    Status::Unknown
                } else {
                    Status::NotMet
                }
            }
            ChecklistResult::Requirement(ref req) => req.status,
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

    pub fn add_or_requirements(&mut self, req1: Req, req2: Req) {
        self.add_checklist(ChecklistType::Or(Box::new(ChecklistType::Requirement(req1)),
                                             Box::new(ChecklistType::Requirement(req2))));
    }

    pub fn check(&self, info: &UserInfo) -> Vec<ChecklistResult> {
        let mut results = Vec::new();
        for c in self.requirements.iter() {
            results.push(c.check(info));
        }
        results
    }
}
