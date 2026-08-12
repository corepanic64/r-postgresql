use std::env;

use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use dotenv::dotenv;

use crate::{auth::jwt::JwtManager, model::DefaultResponse};

pub async fn auth_layer(request: Request, next: Next) -> Response {
    let auth = request.headers().get("Authorization");
    if let Some(token) = auth {
        dotenv().ok();
        let secret_key = env::var("SECRET_KEY").unwrap();
        let mng = JwtManager::new(secret_key);
        let n = token.to_str().unwrap();
        let is_auth = mng.verify(n);
        match is_auth {
            Ok(r) => {
                println!("user is {:?}", r.email);
                next.run(request).await
            }
            Err(_) => {
                println!("WRONG TOKEN");
                unathorized("WRONG TOKEN")
            }
        }
    } else {
        return unathorized("NO TOKEN PROVIDEDD");
    }
}

pub fn unathorized(msg: &'static str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(DefaultResponse {
            success: false,
            message: msg,
        }),
    )
        .into_response()
}
