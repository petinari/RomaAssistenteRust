use crate::Result;
use crate::{web::AUTH_TOKEN, Error};
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use tower_cookies::Cookies;

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("Middleware: mw_require_auth");
    let cookie = cookies.get(AUTH_TOKEN);

    match cookie {
        Some(cookie) => {
            println!("Cookie Found: {:?}", cookie);
            return Ok(next.run(req).await);
        }
        None => {
            println!("Cookie Not Found");
        }
    }

    Ok(Error::NoCookieWasFoundAndAuthIsRequired.into_response())
}
