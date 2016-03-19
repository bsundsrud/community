use checklists::ChecklistModel;

pub struct ProgramModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub org_id: i32,
    pub checklists: Vec<ChecklistModel>,
}
