use crate::errors::ServiceError;
use actix_web::HttpResponse;

pub async fn check() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Created().body("OK"))
}
