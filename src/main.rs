use axum::{
    routing::{get, post},
    Json, 
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .nest("/api", api_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on port {addr}");

    axum::serve(listener, app).await.unwrap();
}

fn api_routes() -> Router {
    api_get_routes().merge(api_post_routes())
}

fn api_get_routes() -> Router {
    Router::new()
        .route("/user", get(|| async { "Hi" }))
}

fn api_post_routes() -> Router {
    Router::new()
        .route("/login", post(handle_login))
}

#[derive(Serialize, Deserialize)]
struct EchoRequest {
    message: String
}

async fn handle_login(Json(payload): Json<EchoRequest>) -> Json<EchoRequest> {
    Json(EchoRequest { 
        message: format!("Recieved: {}", payload.message)
    })
}