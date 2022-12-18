use actix_web_httpauth::headers;
use actix_web_httpauth::headers::www_authenticate::bearer;
use chrono::NaiveDateTime;
use log::info;
use actix_web::{HttpRequest, HttpResponse, Responder, web, HttpResponseBuilder, get};
use fireauth::FireAuth;
use serde::{Deserialize,Serialize};
use crate::db::establish_connection;
use crate::cruds::{*, self};
use crate::auth::minimal_auth;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewStream {
    title: String,
    description: String
}

// #[derive(Deserialize,Serialize)]
// pub struct Stream {
//     id: String,
//     title: String,
//     description: String,
//     streamed_by: String,
//     created_at: NaiveDateTime,
//     is_streaming: bool
// }

pub async fn start_stream(payload: web::Json<NewStream>, request: HttpRequest) -> impl Responder {
    let local_id = match minimal_auth(&request).await{
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    //let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    //let auth = FireAuth::new(api_key);
    let title = &payload.title;
    let description = &payload.description;
    // let responce = match auth.sign_up_email(&email, &password, true).await {
    //     Ok(response) => response,
    //     Err(_) => return HttpResponse::Unauthorized().finish(),
    // };
    // info!("id_token: {:?}", responce.id_token);
    // info!("email: {:?}", responce.email);
    // info!("local_id: {:?}", responce.local_id);
    // databse function here !
    // save local_id, email, name, description
    let conn = establish_connection();
    let stream = create_new_stream(&conn, title.to_string(), description.to_string(), local_id.to_string());

    // let user_info = match auth.get_user_info(&responce.id_token).await {
    //     Ok(user) => user,
    //     Err(_) => return HttpResponse::Unauthorized().finish(),
    // };
    // info!("password_hash: {:?}", user_info.password_hash);
    HttpResponse::Ok().json(stream)
}

pub async fn get_all_streams(request: HttpRequest) -> HttpResponse {    
    let local_id = match minimal_auth(&request).await{
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    // let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    // let auth = FireAuth::new(api_key);
    
    // let responce = match auth.sign_in_email(&email, &password, true).await {
    //     Ok(response) => response,
    //     Err(_) => return HttpResponse::Unauthorized().finish(),
    // };
    // info!("id_token: {:?}", responce.id_token);
    // info!("email: {:?}", responce.email);
    // databse function here !
    // search local_id -> bool
    // let user_info = match auth.get_user_info(&responce.id_token).await {
    //     Ok(user) => user,
    //     Err(_) => return HttpResponse::Unauthorized().finish(),

    // };
    let conn = establish_connection();
    let streams = get_list_streams(&conn);
    //info!("local_id: {:?}", user_info.local_id);
    HttpResponse::Ok().json(streams)
}

#[get("/api/streams/{id}")]
pub async fn get_one_stream(path: web::Path<String>, request: HttpRequest) -> impl Responder {
    let local_id = match minimal_auth(&request).await{
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    let id = path.into_inner();
    let conn = establish_connection();
    let stream = cruds::get_stream_by_id(&conn, &id);
    HttpResponse::Ok().json(stream)
}

#[derive(Deserialize)]
pub struct UpdateQuery {
    pub key: String,
    pub name: String
}

#[get("/api/streams/update")]
pub async fn update_streaming(query: web::Query<UpdateQuery>) -> impl Responder {
    HttpResponse::Ok().body(format!("key is {}. name is {}.", query.key, query.name))
}