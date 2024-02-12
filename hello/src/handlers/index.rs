use axum::response::Html;

async fn index() -> Html<&'static str> {
  Html("<h1>Hello from Rust!</h1>")
}