use crate::db::schema::{projects, secrets};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = secrets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Secret {
    pub id: String,
    pub name: String,
    pub project_id: String,
    pub config: String,
    pub secret: Vec<u8>,
    pub nonce: Vec<u8>,
    pub created_at: NaiveDateTime,
}
