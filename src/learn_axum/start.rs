#[allow(dead_code)]

use tokio::time::Duration;
use tokio::time::sleep;
use axum::{routing::get, Router};

pub async fn initialization()
{
    let app = Router::new()
                .route("/", get(home));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn home()-> &'static str
{
    "home page"
}