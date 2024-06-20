use std::net::Ipv4Addr;

use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{middleware, Router};
use model::ModelController;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

mod error;
mod model;
mod web;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() {
    let mc = ModelController::new().await.unwrap();

    let routes_api = web::routes_ticket::routes(mc)
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(web::routes_login::routes())
        .merge(Router::new().route("/hello", get(hello)))
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new());

    let addr = TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), 8080))
        .await
        .unwrap();
    println!("Server running on http://{:?}", addr.local_addr().unwrap());

    axum::serve(addr, routes_all.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> impl IntoResponse {
    println!("Hello World from handler");

    Html("Hello <strong> World!!! Robson </strong>")
}

async fn main_response_mapper(res: Response) -> Response {
    println!("Response Mapper");
    println!();
    res
}
