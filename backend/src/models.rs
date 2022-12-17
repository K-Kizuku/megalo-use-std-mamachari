use chrono::{DateTime, Utc};
use diesel::{prelude::*, sql_types::Timestamp};
use uuid::Uuid;
use crate::schema::{
    users
};

#[derive(Queryable,Identifiable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub description: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserNewForm {
    pub name: String,
    pub email: String,
    pub description: String,
    pub id: String
}

#[derive(Queryable, Clone)]
pub struct LoginUserForm {
    id: String
}