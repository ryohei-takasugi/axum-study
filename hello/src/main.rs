use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use components::component::;
use handlers::index;

mod handlers;
mod components;

#[tokio::main]
async fn main() {
    // build our application with a route
    let router = Router::new().route("/", get());

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    println!("Request received\nSending response.");
    // Html("<h1>Hello from Rust!</h1>")
    page()
}