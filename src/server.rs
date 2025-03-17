use crate::settings::CONFIG;
use anyhow::Result;
use axum::{
    Router,
    extract::Multipart,
    response::Json,
    routing::{get, post},
};
use log::{info, warn};
use serde_json::{Value, json};
use tokio::net::TcpListener;

pub struct Server;

impl Server {
    // create a new axum server
    pub async fn serve() {
        let settings = CONFIG.clone();
        let host = settings.server.host;
        let port = settings.server.port;
        let listener = TcpListener::bind(format!("{}:{}", host, port))
            .await
            .unwrap();

        let app = Router::new()
            .route("/", get(index))
            .route("/upload", post(upload));

        info!("Server started on {}", format!("{}:{}", host, port));
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}

async fn index() -> String {
    "Hello, World!".to_string()
}

async fn upload(mut multipart: Multipart) -> Json<Value> {
    let Ok((name, data)) = parse_multipart(&mut multipart).await else {
        return Json(json!({ "error": "Error parsing uploaded file" }));
    };
    info!("name and data: {:?}", (name, data));
    Json(json!({ "message": "hello" }))
}

async fn parse_multipart(multipart: &mut Multipart) -> Result<(String, String)> {
    let mut data = String::new();
    let mut name = String::new();

    while let Ok(Some(part)) = multipart.next_field().await {
        match part.name() {
            Some("name") => {
                name = part.text().await?;
            }
            Some("data") => {
                data = part.text().await?;
            }
            _ => {
                warn!("Unknown part: {}", part.name().unwrap());
            }
        }
    }
    Ok((name, data))
}
