use requirements::Status;

#[derive(Debug)]
pub struct ChecklistStatus {
    pub name: String,
    pub status: Status,
}

impl ChecklistStatus {
    pub fn new(name: String, status: Status) -> ChecklistStatus {
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
