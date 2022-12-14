use actix_web_httpauth::headers::www_authenticate::bearer;
use log::info;
use actix_web::{HttpRequest, HttpResponse, Responder, web, HttpResponseBuilder};
use fireauth::FireAuth;
use serde::{Deserialize,Serialize};
use crate::db::establish_connection;
use crate::cruds::{db_sign_up, db_sign_in};

#[derive(Deserialize)]
pub struct NewUser {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct User {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct TokenId {
    token: String,
}

pub async fn firebase_signup(payload: web::Json<NewUser>) -> impl Responder {
    let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    let auth = FireAuth::new(api_key);
    let email = &payload.email;
    let password = &payload.password;
    let name = &payload.name;
    let responce = match auth.sign_up_email(&email, &password, true).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    info!("id_token: {:?}", responce.id_token);
    info!("email: {:?}", responce.email);
    info!("local_id: {:?}", responce.local_id);
    // save local_id, email, name, description
    let conn = establish_connection();
    db_sign_up(
        &conn,
        &responce.local_id,
        &name,
        &responce.email,
        "",
    );

    let user_info = match auth.get_user_info(&responce.id_token).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    info!("password_hash: {:?}", user_info.password_hash);
    HttpResponse::Ok().body("OK")
}

pub async fn firebase_signin(payload: web::Json<User>) -> HttpResponse {
    let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    let auth = FireAuth::new(api_key);
    let email = &payload.email;
    let password = &payload.password;
    let responce = match auth.sign_in_email(&email, &password, true).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    info!("id_token: {:?}", responce.id_token);
    info!("email: {:?}", responce.email);
    // search local_id -> bool
    let conn = establish_connection();
    match db_sign_in(&conn, responce.local_id) {
        true => (),
        false => return HttpResponse::Unauthorized().finish(),
    };
    let user_info = match auth.get_user_info(&responce.id_token).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish(),

    };
    info!("local_id: {:?}", user_info.local_id);
    HttpResponse::Ok().json(TokenId{
        token: responce.id_token
    })
}

#[derive(Debug)]
pub enum MinimalAuthErr {
    TokenNotFound,
    UserFirebaseNotFound,
    UserDbNotFound,
}

pub type MinimalAuthResult = Result<String, MinimalAuthErr>;


pub async fn minimal_auth(request: &HttpRequest) -> MinimalAuthResult {
    let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    let auth = FireAuth::new(api_key);
    // Authorization check
    let bearer = match request.headers().get("Authorization") {
        Some(bearer) => bearer,
        None => return Err(MinimalAuthErr::TokenNotFound),
    };
    // user exist check
    let user_local_id = match auth.get_user_info(bearer.to_str().unwrap()).await {
        Ok(user) => user.local_id,
        Err(_) => return Err(MinimalAuthErr::UserFirebaseNotFound),

    };
    let conn = establish_connection();
    match db_sign_in(&conn, user_local_id.clone()) {
        true => (),
        false => return Err(MinimalAuthErr::UserDbNotFound),
    };

    Ok(user_local_id)
}

