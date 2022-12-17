use crate::controllers::health;
use crate::controllers::signup;
use actix_web::web;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/signup", web::get().to(signup::test));
}

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health::check));
}
