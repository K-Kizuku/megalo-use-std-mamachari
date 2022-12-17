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
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws::{self, start};

use log::{Level, logger};

use megalo_use_std_mamachari::chat_server;
use megalo_use_std_mamachari::chat_session;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env::set_var("RUST_LOG", "trace");
    env::set_var("RUST_LOG", "info");
    logger::init();
    let app_state = Arc::new(AtomicUsize::new(0));
    let chat_server = chat_server::ChatServer::new(app_state.clone()).start();
    info!("HTTP Server Started at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .service(web::resource("/").to(index))
            .service(web::resource("/chat").to(chat_index))
            .route("/chat_count", web::get().to(get_count))
            .route("/chat_ws", web::get().to(chat_route))
            .service(Files::new("/test", "./test"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chat_room_test() {
        
    }
}
