use std::{convert::Infallible, path::Path};
use warp::{self, Filter, Rejection, Reply};

use crate::db::DbHandler;

pub async fn sessions_get(handler: DbHandler) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

pub async fn session_get(id: i32, handler: DbHandler) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

fn with_handler(
    handler: DbHandler,
) -> impl Filter<Extract = (DbHandler,), Error = Infallible> + Clone {
    warp::any().map(move || handler.clone())
}

pub fn sessions_routes(
    handler: DbHandler,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let list = warp::get()
        .and(warp::path("sessions"))
        .and(with_handler(handler.clone()))
        .and_then(sessions_get);

    let get = warp::get()
        .and(warp::path("session"))
        .and(warp::path::param())
        .and(with_handler(handler))
        .and_then(session_get);

    list.or(get)
}

pub fn api_routes(
    handler: DbHandler,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    sessions_routes(handler)
}

pub fn public_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    assert!(
        Path::new("./src/public/").exists(),
        "[ASSERTION] unable to find the static html directory"
    );

    warp::get().and(warp::fs::dir("./src/public/"))
}
