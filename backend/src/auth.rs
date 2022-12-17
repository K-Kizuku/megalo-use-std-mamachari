use log::info;
use actix_web::{HttpRequest, HttpResponse, Responder, web, HttpResponseBuilder};
use fireauth::FireAuth;
use serde::Deserialize;
use crate::db::establish_connection;
use crate::cruds::db_sign_up;

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

pub async fn firebase_signup(payload: web::Json<NewUser>) -> impl Responder{
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
    // databse function here !
    // save local_id, email, name, description
    let conn = establish_connection();
    db_sign_up(
        &conn,
        &responce.local_id,
        &name,
        &responce.email,
        "",
    );

    HttpResponse::Ok().body("OK")
}

pub async fn firebase_signin(payload: web::Json<User>) -> impl Responder{
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
    // databse function here !
    // search local_id -> bool
    HttpResponse::Ok().body("OK")
}

