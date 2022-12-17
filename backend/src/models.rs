use chrono::{DateTime, Utc,NaiveDateTime};
use serde::Serialize;
// use diesel::{sql_types::Uuid};
use uuid::Uuid;
use crate::schema::*;

#[derive(Debug, Queryable,Identifiable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub description: String,
    //pub created_at: DateTime<Utc>,
    //pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserNewForm<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub description: &'a str,
}

#[derive(Queryable, Clone)]
pub struct LoginUserForm {
    pub id: String
}

#[derive(Debug,Queryable,Serialize)]
pub struct Stream {
    pub id: Uuid,
    pub streamed_by: String,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub is_streaming: bool,
    //pub updated_at: Timestamp,
}

#[derive(Insertable)]
#[table_name = "streams"]
pub struct NewStream {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub streamed_by: String,
    pub created_at: NaiveDateTime,
    pub is_streaming: bool
}
