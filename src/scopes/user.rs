use crate::extractors::authentication_token::{AuthenticationToken, Claims};
use actix_web::{HttpResponse, Scope, web};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode-token/{id}", web::get().to(encode_token))
        .route("/decode-token", web::post().to(decode_token))
        .route("/protected", web::get().to(protected))
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    message: String,
    token: String,
}

async fn encode_token(path: web::Path<usize>, secret: web::Data<String>) -> HttpResponse {
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now() + Duration::days(10)).timestamp() as usize;
    let claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();
    HttpResponse::Ok().json(EncodeResponse {
        message: "Successfully created account".to_owned(),
        token,
    })
}

#[derive(Serialize, Deserialize)]
struct DecodeResponse {
    message: String,
    id: usize,
}

#[derive(Serialize, Deserialize)]
struct DecodeBody {
    token: String,
}

async fn decode_token(body: web::Json<DecodeBody>, secret: web::Data<String>) -> HttpResponse {
    let token_result = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match token_result {
        Ok(token) => HttpResponse::Ok().json(DecodeResponse {
            message: "Successfully Logged in.".to_owned(),
            id: token.claims.id,
        }),
        Err(err) => HttpResponse::Unauthorized().json(Response {
            message: err.to_string(),
        }),
    }
}

async fn protected(auth_token: AuthenticationToken) -> HttpResponse {
    println!("{:#?}", auth_token);
    HttpResponse::Ok().json(Response {
        message: "Authorized".to_owned(),
    })
}
