use checklists::ChecklistModel;

#[derive(Debug)]
pub struct ProgramModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub org_id: i32,
    pub checklists: Option<Vec<ChecklistModel>>,
}
