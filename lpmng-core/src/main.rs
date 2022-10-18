mod api;
mod auth;
mod db;
mod models;

use api::{api_routes, public_route, ApiHandler};
use warp::Filter;

#[tokio::main]
async fn main() {
    let db_handler = db::DbHandler::connect().await.unwrap();
    println!("[INFO] database successfully connected");

    println!("[INFO] http server starting...");
    warp::serve(public_route().or(api_routes(ApiHandler {
        db: db_handler,
        auth_key: "dd".into(),
    })))
    .run(([127, 0, 0, 1], 3030))
    .await;
}
