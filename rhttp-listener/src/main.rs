use std::env;
use std::error::Error;
use std::io::Write;
use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    Json,
    Router, routing::post,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn create_user( Json(payload): Json<MetaFile> ) -> (StatusCode, Json<MetaFile>) {
    let user = MetaFile{ name: payload.name.clone(), contents: payload.contents.clone()};

    let path = env::home_dir().unwrap().join("downloads").join(payload.name);
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(payload.contents.as_bytes()).unwrap();


    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize, Serialize)]
struct MetaFile {
    name: String,
    contents: String,
}


// async fn download(Json(payload): Json<MetaFile>) {
