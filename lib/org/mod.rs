use programs::ProgramModel;
use db::PostgresConnection as Connection;
use postgres::error::Error;
use postgres::rows::Row;
use pgx::{queryx, FromRow};

#[derive(Debug)]
pub struct OrgModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub programs: Option<Vec<ProgramModel>>,
    pub branches: Option<Vec<BranchModel>>,
}
impl FromRow for OrgModel {
    fn from_row(row: &Row) -> OrgModel {
        OrgModel {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            programs: None,
            branches: None
        }
    }
}

impl OrgModel {

    pub fn new<S>(name: S, description: Option<String>) -> OrgModel where S: Into<String> {
        OrgModel {
            id: -1,
            name: name.into(),
            description: description,
            programs: None,
            branches: None,
        }
    }

    pub fn list<'a>(conn: &'a Connection) -> Result<Vec<OrgModel>, Error> {
        let stmt = conn.prepare("SELECT id, name, description FROM organizations")?;
        let iter = queryx::<OrgModel>(&stmt, &[])?;
        Ok(iter.collect::<Vec<_>>())
    }

    pub fn get<'a>(conn: &'a Connection, id: i32) -> Result<Option<OrgModel>, Error> {
        let stmt = conn.prepare("SELECT id, name, description FROM organizations WHERE id = $1")?;
        let mut results = queryx::<OrgModel>(&stmt, &[&id])?.collect::<Vec<_>>();
        if results.len() == 0 {
            Ok(None)
        } else {
            Ok(results.pop())
        }
    }

    pub fn create<'a>(self, conn: &'a Connection) -> Result<(), Error> {
        conn.execute("INSERT INTO organizations(name, description) VALUES($1, $2)",
                     &[&self.name, &self.description])?;
        Ok(())
    }

    pub fn update<'a>(&self, conn: &'a Connection) -> Result<(), Error> {
        conn.execute("UPDATE organizations SET name = $1, description = $2 WHERE id = $3",
                     &[&self.name, &self.description, &self.id])?;
        Ok(())
    }

    pub fn delete<'a>(self, conn: &'a Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM organizations WHERE id = $1", &[&self.id])?;
        Ok(())
    }
}
#[derive(Debug)]
pub struct BranchModel {
    pub id: i32,
    pub org_id: i32,
    pub location: Option<String>,
}
