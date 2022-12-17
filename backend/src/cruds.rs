use chrono::Utc;
use diesel::{RunQueryDsl, QueryDsl};
use diesel::pg::PgConnection;
use uuid::Uuid;
use diesel::prelude::*;


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

pub fn create_new_stream(conn: &PgConnection, other_title: String, description: String, streamed_by: String) -> crate::models::Stream {
    use crate::schema::streams;
    use crate::models::NewStream;
    let new_stream = NewStream {
        id: Uuid::new_v4(),
        streamed_by,
        title: other_title,
        description,
        created_at: Utc::now().naive_utc(),
        is_streaming: false
    };
    diesel::insert_into(streams::table)
        .values(&new_stream)
        .get_result(conn)
        .expect("Error saving new stream")
}

pub fn get_list_streams(conn: &PgConnection) -> Vec<crate::models::Stream> {
    // use crate::schema::streams;
    use crate::models::Stream;
    use crate::schema::streams::dsl::*;
    streams.filter(is_streaming.eq_all(true)).load::<Stream>(conn).expect("Error failed all streams")
}

// pub fn update_stream_flag(conn: &PgConnection, id: String) {
//     use crate::schema::streams::dsl::*;
//     use crate::models::Stream;
//     let stream = streams.filter(is_streaming.eq_all(true))
// }

pub fn get_stream_by_id(conn: &PgConnection, sid: &str) -> crate::models::Stream {
    use crate::models::Stream;
    use crate::schema::streams::dsl::*;
    let uuid = Uuid::parse_str(sid).unwrap();
    streams.find(uuid).first::<Stream>(conn).expect("Error stream by id")
}