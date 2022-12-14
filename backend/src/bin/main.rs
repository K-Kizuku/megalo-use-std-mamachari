
use actix_web::{
    App,HttpServer,Responder,HttpResponse,get,Error,
    middleware::{
        Compress,
        Logger,
    },
    web::{
        self,
        Data,
    }, HttpRequest,
};
use dotenv::dotenv;

#[macro_use]
extern crate log;
extern crate env_logger as logger;

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
    env,
};

use actix::*;
use actix_files::{Files, NamedFile};
use actix_web_actors::ws::{self, start};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

use log::{Level, logger};

use megalo_use_std_mamachari::{chat_server, schema::streams, streams as st};
use megalo_use_std_mamachari::chat_session;
use megalo_use_std_mamachari::auth;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


// ##### chat ##### //
async fn chat_index() -> impl Responder {
    NamedFile::open_async("./test/chat_test.html").await.unwrap()
}

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<chat_server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        chat_session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}

// ##### firebase ##### //


// ##### main ##### //
#[actix_web::main]
async fn main() -> std::io::Result<()> {
     // .envに記述された環境変数の読み込み.
    dotenv().ok();
    //env::set_var("RUST_LOG", "trace");
    env::set_var("RUST_LOG", "info");
    dotenv::dotenv().ok();
    logger::init();
    let app_state = Arc::new(AtomicUsize::new(0));
    let chat_server = chat_server::ChatServer::new(app_state.clone()).start();
    info!("HTTP Server Started at http://localhost:8000");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .service(web::resource("/api").to(index))
            // chat
            .app_data(web::Data::new(chat_server.clone()))
            .service(web::resource("/chat").to(chat_index))
            .route("/api/chat_count", web::get().to(get_count))
            .route("/api/chat_ws", web::get().to(chat_route))
            //streaming
            .route("/api/streams", web::get().to(st::get_all_streams))
            .route("/api/streams", web::post().to(st::start_stream))
            //.route("/api/streams/modify", web::get().to(st::update_streaming))
            .service(st::update_streaming)
            .service(st::delete_streaming)
            .service(st::get_one_stream)
            // firebase
            .route("/api/signup", web::post().to(auth::firebase_signup))
            .route("/api/signin", web::post().to(auth::firebase_signin))
            .service(Files::new("/test", "./test"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

