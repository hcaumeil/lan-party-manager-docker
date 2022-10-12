mod api;

use std::path::Path;
use warp::{self, Filter, Rejection, Reply};

fn public_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    assert!(
        Path::new("./src/public/").exists(),
        "[ASSERTION] unable to find the static html directory"
    );

    warp::get().and(warp::fs::dir("./src/public/"))
}

#[tokio::main]
async fn main() {
    warp::serve(public_route())
        .run(([127, 0, 0, 1], 3030))
        .await;
}
