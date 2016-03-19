use postgres::types::{ToSql, FromSql, Type, SessionInfo, IsNull};
use postgres::error::Error;
use postgres::Result;
use requirements;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Eq)]
pub enum RequirementType {
    Boolean,
    IntRange,
    IntEquals,
    StringEquals,
}

impl ToSql for RequirementType {
    fn to_sql<W: Write + ?Sized>(&self,
                                 _: &Type,
                                 mut w: &mut W,
                                 _: &SessionInfo)
                                 -> Result<IsNull> {
        let name = match *self {
            RequirementType::Boolean => "boolean",
            RequirementType::IntRange => "int_range",
            RequirementType::IntEquals => "int_equals",
            RequirementType::StringEquals => "string_equals",
        };
        try!(w.write_all(name.as_bytes()));
        try!(w.write(&[0])); // I think the strings are null terminated but can't remember for sure
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "requirement_type"
    }

    to_sql_checked!();
}

impl FromSql for RequirementType {
    fn from_sql<R: Read>(_ty: &Type, raw: &mut R, _ctx: &SessionInfo) -> Result<Self> {
        let mut buf = vec![];
        try!(raw.read_to_end(&mut buf));

        match &*buf {
            b"boolean" => Ok(RequirementType::Boolean),
            b"int_range" => Ok(RequirementType::IntRange),
            b"int_equals" => Ok(RequirementType::IntEquals),
            b"string_equals" => Ok(RequirementType::StringEquals),
            other @ _ => {
                Err(Error::Conversion(format!("unknown `requirement_type` variant: {:?}", other)
                                          .into()))
            }
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "requirement_type"
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FieldType {
    Age,
    County,
    ChildrenCount,
    Income,
    SingleParent,
}

impl ToSql for FieldType {
    fn to_sql<W: Write + ?Sized>(&self,
                                 _: &Type,
                                 mut w: &mut W,
                                 _: &SessionInfo)
                                 -> Result<IsNull> {
        let name = match *self {
            FieldType::Age => "age",
            FieldType::County => "county",
            FieldType::ChildrenCount => "children_count",
            FieldType::Income => "income",
            FieldType::SingleParent => "single_parent",
        };
        try!(w.write_all(name.as_bytes()));
        try!(w.write(&[0])); // I think the strings are null terminated but can't remember for sure
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "field_type"
    }

    to_sql_checked!();
}



impl FromSql for FieldType {
    fn from_sql<R: Read>(_ty: &Type, raw: &mut R, _ctx: &SessionInfo) -> Result<Self> {
        let mut buf = vec![];
        try!(raw.read_to_end(&mut buf));

        match &*buf {
            b"age" => Ok(FieldType::Age),
            b"county" => Ok(FieldType::County),
            b"children_count" => Ok(FieldType::ChildrenCount),
            b"income" => Ok(FieldType::Income),
            b"single_parent" => Ok(FieldType::SingleParent),
            _ => Err(Error::Conversion("unknown `requirement_type` variant".into())),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "requirement_field"
    }
}

impl From<FieldType> for requirements::Field {
    fn from(pgtype: FieldType) -> requirements::Field {
        match pgtype {
            FieldType::Age => requirements::Field::Age,
            FieldType::County => requirements::Field::County,
            FieldType::ChildrenCount => requirements::Field::ChildrenCount,
            FieldType::Income => requirements::Field::Income,
            FieldType::SingleParent => requirements::Field::SingleParent,
        }
    }
}

impl<'a> From<&'a FieldType> for requirements::Field {
    fn from(pgtype: &FieldType) -> requirements::Field {
        match pgtype {
            &FieldType::Age => requirements::Field::Age,
            &FieldType::County => requirements::Field::County,
            &FieldType::ChildrenCount => requirements::Field::ChildrenCount,
            &FieldType::Income => requirements::Field::Income,
            &FieldType::SingleParent => requirements::Field::SingleParent,
        }
    }
}
