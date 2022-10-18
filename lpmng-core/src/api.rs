use biscuit_auth::PrivateKey;
use serde_json;
use std::{convert::Infallible, path::Path};
use warp::{self, Filter, Rejection, Reply};

use crate::{
    auth::{build_token, check_admin, hash},
    db::DbHandler,
    models::User,
};

#[derive(Clone)]
pub struct ApiHandler {
    pub db: DbHandler,
    pub auth_key: PrivateKey,
}

pub async fn login_post(
    json: serde_json::Value,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(login) = json.get("login") {
        if let Some(password) = json.get("password") {
            let role = handler
                .db
                .check_password(
                    login.as_str().unwrap().into(),
                    password.as_str().unwrap().into(),
                )
                .await;
            if role.is_some() {
                return Ok(warp::reply::json(&build_token(
                    role.expect("Can't be null"),
                    handler.auth_key,
                )));
            } else {
                return Err(warp::reject());
            }
        } else {
            return Err(warp::reject());
        }
    } else {
        return Err(warp::reject());
    }
}

pub async fn sessions_get(handler: ApiHandler) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

pub async fn session_get(
    id: i32,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

pub async fn users_get(
    auth_token: String,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = handler.db.get_users().await;
    println!("{}", check_admin(auth_token, handler.auth_key));

    match res {
        Some(json) => Ok(warp::reply::json(&json)),
        None => Err(warp::reject()),
    }
}

pub async fn user_get(id: u128, handler: ApiHandler) -> Result<impl warp::Reply, warp::Rejection> {
    let res = handler.db.get_user(id).await;

    match res {
        Some(json) => Ok(warp::reply::json(&json)),
        None => Err(warp::reject()),
    }
}

pub async fn user_post(
    user: User,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut object = user.clone();
    object.password = hash(user.password);

    let res = match user.id {
        Some(_) => handler.db.update_user(object).await,
        None => handler.db.insert_user(object).await,
    };

    if res {
        Ok(warp::reply())
    } else {
        Err(warp::reject())
    }
}

fn with_handler(
    handler: ApiHandler,
) -> impl Filter<Extract = (ApiHandler,), Error = Infallible> + Clone {
    warp::any().map(move || handler.clone())
}

pub fn sessions_routes(
    handler: ApiHandler,
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
    handler: ApiHandler,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let list = warp::get()
        .and(warp::path("users"))
        .and(warp::header::<String>("Authorization"))
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

pub fn login_routes(
    handler: ApiHandler,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(with_handler(handler))
        .and_then(login_post)
}

pub fn api_routes(
    handler: ApiHandler,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("api").and(
        sessions_routes(handler.clone())
            .or(users_routes(handler.clone()))
            .or(login_routes(handler)),
    )
}

pub fn public_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    assert!(
        Path::new("./src/public/").exists(),
        "[ASSERTION] unable to find the static html directory"
    );

    warp::get().and(warp::fs::dir("./src/public/"))
}
