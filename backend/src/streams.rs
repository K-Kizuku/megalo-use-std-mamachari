use chrono::NaiveDateTime;
use log::info;
use actix_web::{HttpRequest, HttpResponse, Responder, web, HttpResponseBuilder};
use fireauth::FireAuth;
use serde::{Deserialize,Serialize};
use crate::db::establish_connection;
use crate::cruds::*;

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

pub async fn start_stream(payload: web::Json<NewStream>) -> impl Responder {
    let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    let auth = FireAuth::new(api_key);
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
    let stream = create_new_stream(&conn, title.to_string(), description.to_string(), "1".to_string());

    // let user_info = match auth.get_user_info(&responce.id_token).await {
    //     Ok(user) => user,
    //     Err(_) => return HttpResponse::Unauthorized().finish(),
    // };
    // info!("password_hash: {:?}", user_info.password_hash);
    HttpResponse::Ok().json(stream)
}

pub async fn get_all_streams() -> HttpResponse {
    let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    let auth = FireAuth::new(api_key);
    
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
    let stremas = get_list_streams(&conn);
    //info!("local_id: {:?}", user_info.local_id);
    HttpResponse::Ok().json(stremas)
}

// pub async fn get_one_stream() -> HttpResponse {

// }