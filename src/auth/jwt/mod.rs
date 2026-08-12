use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub user_id: i32,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

pub struct JwtManager {
    secret: String,
}

impl JwtManager {
    pub fn new(secret: String) -> Self {
        JwtManager { secret }
    }
    pub fn generate_token(
        &self,
        user_id: i32,
        email: String,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let expiration = now + Duration::hours(2);

        let claims = Claims {
            sub: user_id.to_string(),
            user_id,
            email,
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };
        let key = EncodingKey::from_secret(self.secret.as_ref());
        encode(&Header::default(), &claims, &key)
    }
    pub fn verify(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let key = DecodingKey::from_secret(self.secret.as_ref());
        let data = decode(&token, &key, &Validation::default())?;
        Ok(data.claims)
    }
}
