use std::{convert::Infallible, path::Path};
use warp::{self, Filter, Rejection, Reply};

use crate::{db::DbHandler, models::User};

pub async fn sessions_get(handler: DbHandler) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

pub async fn session_get(id: i32, handler: DbHandler) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

pub async fn users_get(handler: DbHandler) -> Result<impl warp::Reply, warp::Rejection> {
    let res = handler.get_users().await;

    match res {
        Some(json) => Ok(warp::reply::json(&json)),
        None => Err(warp::reject()),
    }
}

pub async fn user_get(id: u128, handler: DbHandler) -> Result<impl warp::Reply, warp::Rejection> {
    let res = handler.get_user(id).await;

    match res {
        Some(json) => Ok(warp::reply::json(&json)),
        None => Err(warp::reject()),
    }
}

pub async fn user_post(
    user: User,
    handler: DbHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match user.id {
        Some(_) => false,
        None => handler.insert_user(user).await,
    };

    if res {
        Ok(warp::reply())
    } else {
        Err(warp::reject())
    }
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
        .and(warp::path("sessions"))
        .and(warp::path::param())
        .and(with_handler(handler))
        .and_then(session_get);

    list.or(get)
}

pub fn users_routes(
    handler: DbHandler,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let list = warp::get()
        .and(warp::path("users"))
        .and(with_handler(handler.clone()))
        .and_then(users_get);

    let get = warp::get()
        .and(warp::path("users"))
        .and(warp::path::param())
        .and(with_handler(handler.clone()))
        .and_then(user_get);

    let post = warp::post()
        .and(warp::path("users"))
        .and(warp::body::json())
        .and(with_handler(handler))
        .and_then(user_post);

    get.or(list).or(post)
}

pub fn api_routes(
    handler: DbHandler,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("api").and(sessions_routes(handler.clone()).or(users_routes(handler)))
}

pub fn public_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    assert!(
        Path::new("./src/public/").exists(),
        "[ASSERTION] unable to find the static html directory"
    );

    warp::get().and(warp::fs::dir("./src/public/"))
}
