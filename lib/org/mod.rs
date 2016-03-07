use finder::checklists::ChecklistModel;

pub struct Org {
    pub name: String,
    pub description: String,
    pub programs: Vec<Program>,
}

pub struct Program {
    pub name: String,
    pub description: String,
    pub requirements: Vec<ChecklistModel>,
}
