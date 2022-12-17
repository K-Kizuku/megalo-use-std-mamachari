use diesel::RunQueryDsl;
use diesel::pg::PgConnection;

pub fn db_sign_up<'a>(conn: &PgConnection, id: &'a str, name: &'a str, email: &'a str, description: &'a str) -> crate::models::User {
    use crate::schema::users;
    use crate::models::UserNewForm;
    let new_user = UserNewForm {
        id,
        name,
        email,
        description,
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}
