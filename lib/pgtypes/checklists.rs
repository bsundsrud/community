use postgres::types::{ToSql, FromSql, Type, SessionInfo, IsNull};
use postgres::error::Error;
use postgres::Result;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Eq)]
pub enum ChecklistType {
    Or,
    And,
    Req,
}

impl FromSql for ChecklistType {
    fn from_sql<R: Read>(_ty: &Type, raw: &mut R, _ctx: &SessionInfo) -> Result<Self> {
        let mut buf = vec![];
        try!(raw.read_to_end(&mut buf));

        match &*buf {
            b"or" => Ok(ChecklistType::Or),
            b"and" => Ok(ChecklistType::And),
            b"req" => Ok(ChecklistType::Req),
            _ => Err(Error::Conversion("unknown `checklist_type` variant".into())),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "checklist_type"
    }
}

impl ToSql for ChecklistType {
    fn to_sql<W: Write + ?Sized>(&self,
                                 _: &Type,
                                 mut w: &mut W,
                                 _: &SessionInfo)
                                 -> Result<IsNull> {
        let name = match *self {
            ChecklistType::Or => "or",
            ChecklistType::And => "and",
            ChecklistType::Req => "req",
        };
        try!(w.write_all(name.as_bytes()));
        try!(w.write(&[0])); // I think the strings are null terminated but can't remember for sure
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "checklist_type"
    }

    to_sql_checked!();
}
