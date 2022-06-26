use std::net::SocketAddr;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use tracing::metadata::LevelFilter;

use adder_rust::{add, add_two_random};

#[tracing::instrument]
async fn add_router(Path((a, b)): Path<(usize, usize)>) -> impl IntoResponse {
    tracing::info!("summing {} and {}", a, b);

    let res: String = format!("{} + {} = {}", a, b, add(a, b));

    printer_rust::print(res.as_str());
    (StatusCode::OK, res)
}

#[tracing::instrument]
async fn add_random_router() -> impl IntoResponse {
    tracing::info!("summing random numbers");

    let res: String = format!("{}", add_two_random());

    printer_rust::print(res.as_str());
    (StatusCode::OK, res)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .pretty()
        .init();

    let app: Router = Router::new()
        .route("/random", get(add_random_router))
        .route("/add/:a/:b", get(add_router));

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
