use actix_web::{
    Error as ActixWebError, FromRequest, HttpRequest, dev::Payload, error::ErrorUnauthorized,
    http::header::HeaderValue, web,
};
use jsonwebtoken::{
    Algorithm, DecodingKey, TokenData, Validation, decode, errors::Error as JwtError,
};
use serde::{Deserialize, Serialize};
use std::future::{Ready, ready};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationToken {
    id: usize,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        let authorization_header_option = req.headers().get(actix_web::http::header::AUTHORIZATION);

        if authorization_header_option.is_none() {
            return ready(Err(ErrorUnauthorized("No Authentication token provided")));
        }

        let authentication_token = authorization_header_option
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string();
        let secret: &str = &req.app_data::<web::Data<String>>().unwrap();

        let token_result = decode::<Claims>(
            &authentication_token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match token_result {
            Ok(token) => ready(Ok(AuthenticationToken {
                id: token.claims.id,
            })),
            Err(err) => ready(Err(ErrorUnauthorized(err.to_string()))),
        }
    }
}
