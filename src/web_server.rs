use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use axum::{
    routing::{get, post},
    Json, 
    Router,
};

pub struct WebServer {
    addr: SocketAddr
}

#[derive(Serialize, Deserialize)]
struct EchoRequest {
    message: String
}

impl WebServer {
    pub fn new (ip: [u8; 4], port: u16) -> WebServer {
        WebServer {
            addr: SocketAddr::from((ip, port))
        }
    }

    pub async fn begin(&self) {
        let app = Router::new()
            .route("/", get(|| async { "Hello world" }))
            .nest("/api", self.api_routes());

        let listener = tokio::net::TcpListener::bind(self.addr).await.unwrap();
        println!("Listening on port {}", self.addr);

        axum::serve(listener, app).await.unwrap();
    }

    fn api_routes(&self) -> Router {
        self.api_get_routes().merge(self.api_post_routes())
    }
    
    fn api_get_routes(&self) -> Router {
        Router::new()
            .route("/user", get(|| async { "Hi" }))
    }
    
    fn api_post_routes(&self) -> Router {
        Router::new()
            .route("/login", post(handle_login))
    }
}

async fn handle_login(Json(payload): Json<EchoRequest>) -> Json<EchoRequest> {
    Json(EchoRequest { 
        message: format!("Recieved: {}", payload.message)
    })
}