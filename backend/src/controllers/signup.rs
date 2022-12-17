use crate::errors::ServiceError;
use actix_web::HttpResponse;

pub async fn test() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Created().body("test"))
}
