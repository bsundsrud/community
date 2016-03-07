use finder::checklists::ChecklistModel;

pub struct ProgramModel {
    id: i32,
    name: String,
    description: Option<String>,
    org_id: i32,
    checklists: Vec<ChecklistModel>,
}
