use actix_web::{
    App,HttpServer,Responder,HttpResponse,get,
    middleware::{
        Compress,
        Logger,
    },
    web::{
        self,
        Data,
    },
};
use dotenv::dotenv;
use std::{
    env,
    sync::Arc,
};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
     // .envに記述された環境変数の読み込み.
    dotenv().ok();

    // debugと同等以上の重要度を持つログを表示するように設定し、ログを開始する.
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let pool = Arc::new(new_pool()?);
    HttpServer::new(|| {
        App::new()
            .app_data(Data::from(pool.clone()))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                .service(index)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
