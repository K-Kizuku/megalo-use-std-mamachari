use diesel::{RunQueryDsl, QueryDsl};
use diesel::pg::PgConnection;
use uuid::Uuid;

use crate::schema::streams::title;

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

fn get_list_users(conn: &PgConnection) -> Vec<crate::models::User> {
    use crate::schema::users;
    use crate::models::User;
    users::table.load::<User>(conn).expect("Error getting users")
}

pub fn db_sign_in(conn: &PgConnection, id: String) -> bool {
    let user_vec = get_list_users(conn);
    match user_vec
        .iter()
        .find(|&user| user.id == id) {
    Some(_) => return true,
    None => return false,
    };
}

pub fn create_new_stream(conn: &PgConnection, other_title: String, description: String, streamed_by: String) -> crate::models::Stream {
    use crate::schema::streams;
    use crate::models::NewStream;
    let new_stream = NewStream {
        id: Uuid::new_v4(),
        streamed_by,
        title: other_title,
        description,
    };
    diesel::insert_into(streams::table)
        .values(&new_stream)
        .get_result(conn)
        .expect("Error saving new stream")
}

pub fn get_list_streams(conn: &PgConnection) -> Vec<crate::models::Stream> {
    use crate::schema::streams;
    use crate::models::Stream;
    streams::table.load::<Stream>(conn).expect("Error getting streams")
}

pub fn update_stream_flag(conn: &PgConnection, ) {
    use crate::schema::streams;
    use crate::models::Stream;
}
