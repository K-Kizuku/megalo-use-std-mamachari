use anyhow::Result;
use diesel::RunQueryDsl;
use crate::models::*;
use crate::schema::*;
use diesel::pg::PgConnection;

pub fn sign_up(conn: &mut PgConnection, id: String, email: String, name: String)-> Result<User> {
    let mut new_user = UserNewForm{id:id, name:name, email:email, description:"".to_string()};
    Ok(diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user"))
}