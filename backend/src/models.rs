use chrono::{DateTime, Utc};
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
    id: String
}
