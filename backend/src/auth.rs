use crate::errors::ServiceError;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub async fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let jwk_url = std::env::var("JWK_URL").expect("JWK_URL must be set");
    let jwk_issuer = std::env::var("JWK_ISSUER").expect("JWK_ISSUER must be set");
    println!("jwk_url:{}", jwk_url);
    println!("jwk_issuer:[{}]", jwk_issuer);

    let jwks = fetch_jwks(jwk_url.as_str())
        .await
        .expect("failed to fetch jwks");

    // issとsub（user_id）をチェック
    // 必須ではなく、カスタマイズできる
    let validations = vec![Validation::Issuer(jwk_issuer), Validation::SubjectPresent];

    let kid = match token_kid(token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return Err(ServiceError::JWKSFetchError),
    };

    println!("kid:{:?}", kid);

    let jwk = jwks.find(&kid).expect("Specified key not found in set");

    println!("jwk:{:?}", jwk);

    let res = validate(token, jwk, validations);
    println!("res.is_ok():{}", res.is_ok());
    Ok(res.is_ok())
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    Ok(val)
}

