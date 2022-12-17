use crate::controllers::health;
use crate::controllers::hoge;
use actix_web::web;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/hoge", web::get().to(hoge::test));
}

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health::check));
}
