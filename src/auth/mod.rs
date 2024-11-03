use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize}
};

use crate::AppConfig;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: u32,
    pub role: String,
    pub exp: u64,
}

pub struct AuthenticatedUser {
    pub id: u32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
       let config = req.rocket().state::<AppConfig>().expect("AppConfig must be initialized");

       if let Some(token) = req.headers().get_one("token") {
          match decode::<Claims>(
              token,
              &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
              &Validation::new(jsonwebtoken::Algorithm::HS256),
          ) {
              Ok(data) => Outcome::Success(AuthenticatedUser { id: data.claims.sub }),
              Err(_) => Outcome::Error((Status::Unauthorized, "Invalid token".to_string())),
          }
        } else {
            Outcome::Error((Status::Unauthorized, "Token absent".to_string()))
        }
    }
}