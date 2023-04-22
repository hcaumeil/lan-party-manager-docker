mod api;
mod auth;
mod db;
mod models;

use api::{api_routes, public_route, ApiHandler};
use biscuit_auth::KeyPair;
use lpmng_mq::client::Client;
use warp::Filter;

fn env_abort(env: &'static str) -> impl Fn(std::env::VarError) -> String {
    move |e| {
        eprintln!("[ERROR] ${env} is not set ({})", e);
        std::process::exit(1);
    }
}

fn env_get(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(env_abort(env))
}

#[tokio::main]
async fn main() {
    let admin_key = env_get("ADMIN_KEY");
    let client_key = env_get("CLIENT_KEY");
    let router_address = env_get("ROUTER_ADDRESS");
    let port = match std::env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap_or(3030),
        Err(_) => 3030,
    };

    println!("[INFO] api keys have been found");

    let db_handler = db::DbHandler::connect().await.unwrap();
    println!("[INFO] database successfully connected");

    println!("[INFO] http server starting...");
    warp::serve(
        public_route().or(api_routes(ApiHandler {
            db: db_handler,
            auth_key: KeyPair::new().private(),
            admin_key,
            client_key,
            router: Client::connect(&router_address)
                .await
                .expect("lpmng router has not been found"),
        })),
    )
    .run(([127, 0, 0, 1], port))
    .await;
}
