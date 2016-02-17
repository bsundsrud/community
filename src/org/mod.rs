use finder::checklist::Checklist;

pub struct Org {
    // TODO: probably attach Programs somehow.  Maybe an ORM like diesel would help.
    pub name: String,
    pub description: String,
}

pub struct Program {
    pub name: String,
    pub description: String,
    pub requirements: Checklist,
}
