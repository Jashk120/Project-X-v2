mod app;
mod modules;
mod shared;

use shared::config::env::get_port;
use std::net::SocketAddr;

#[tokio::main]
async fn main(){
    dotenvy::dotenv().ok();

    let app = app::build_app();

    let port = get_port();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind tcp listener");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}