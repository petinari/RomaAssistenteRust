use axum::{Json, Router};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result};

#[derive(serde::Deserialize, Debug)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String,
}

pub fn routes() -> Router {
    axum::Router::new().route("/login", axum::routing::post(login))
}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    cookies.add(Cookie::new(crate::web::AUTH_TOKEN, "123456"));

    if payload.username == "admin" && payload.pwd == "admin" {
        println!("Login Success");
        return Ok(body);
    }

    Err(Error::LoginFail)
}
