mod api;
mod auth;
mod db;
mod models;

use api::{api_routes, public_route, ApiHandler};
use biscuit_auth::KeyPair;
use warp::Filter;

#[tokio::main]
async fn main() {
    let admin_key = std::env::var("ADMIN_KEY").unwrap_or_else(|e| {
        eprintln!("[ERROR] $ADMIN_KEY is not set ({})", e);
        std::process::exit(1);
    });

    let client_key = std::env::var("CLIENT_KEY").unwrap_or_else(|e| {
        eprintln!("[ERROR] $CLIENT_KEY is not set ({})", e);
        std::process::exit(1);
    });

    println!("[INFO] api keys have been found");

    let db_handler = db::DbHandler::connect().await.unwrap();
    println!("[INFO] database successfully connected");

    println!("[INFO] http server starting...");
    warp::serve(public_route().or(api_routes(ApiHandler {
        db: db_handler,
        auth_key: KeyPair::new().private(),
        admin_key,
        client_key,
    })))
    .run(([127, 0, 0, 1], 3030))
    .await;
}
