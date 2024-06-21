use axum::{Json, Router};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result};

use super::AUTH_TOKEN;

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

    // FIXME: Implement real auth-token generation/signature.
    let mut cookie = Cookie::new(AUTH_TOKEN, "user-1.exp.sign");
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    if payload.username == "admin" && payload.pwd == "admin" {
        println!("Login Success");
        return Ok(body);
    }

    Err(Error::LoginFail)
}
