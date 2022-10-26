use base64_url::{decode, encode, unescape};
use biscuit_auth::PrivateKey;
use lpmng_mq;
use serde_json;
use std::{convert::Infallible, path::Path};
use warp::{self, Filter, Rejection, Reply};

use crate::{
    auth::{build_token, check_admin, hash},
    db::DbHandler,
    models::{Credentials, Session, User},
};

#[derive(Clone)]
pub struct ApiHandler {
    pub db: DbHandler,
    pub auth_key: PrivateKey,
    pub admin_key: String,
    pub client_key: String,
    pub router: lpmng_mq::client::Client,
}

pub fn is_admin(auth_token: String, private_key: PrivateKey) -> bool {
    let mut split = auth_token.split(" ");

    if split.clone().count() != 2 {
        return false;
    }

    if split.clone().nth(0).unwrap() != "Bearer" {
        return false;
    }

    check_admin(split.nth(1).unwrap().into(), private_key)
}

pub async fn login_post(
    json: serde_json::Value,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(login) = json.get("login") {
        if let Some(password) = json.get("password") {
            if (login.as_str().unwrap() == "admin"
                && password.as_str().unwrap() == handler.admin_key)
                || (login.as_str().unwrap() == "client"
                    && password.as_str().unwrap() == handler.client_key)
            {
                match build_token("admin".into(), 0, handler.auth_key) {
                    Some(t) => {
                        return Ok(warp::reply::json(&Credentials {
                            biscuit: t,
                            role: "admin".into(),
                            user_id: None,
                        }))
                    }
                    None => return Err(warp::reject()),
                }
            }

            let auth = handler
                .db
                .check_password(
                    login.as_str().unwrap().into(),
                    password.as_str().unwrap().into(),
                )
                .await;
            if auth.is_some() {
                let (role, id) = auth.expect("Can't be null");

                match build_token(role.to_owned(), id, handler.auth_key) {
                    Some(t) => {
                        return Ok(warp::reply::json(&Credentials {
                            biscuit: t,
                            role,
                            user_id: Some(encode(&id.to_string())),
                        }))
                    }
                    None => return Err(warp::reject()),
                }
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

pub async fn sessions_get(
    auth_token: String,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !is_admin(auth_token, handler.auth_key) {
        return Err(warp::reject());
    }

    let res = handler.db.get_sessions().await;

    match res {
        Some(json) => Ok(warp::reply::json(&json)),
        None => Err(warp::reject()),
    }
}

pub async fn session_get(
    id: i32,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

pub async fn sessions_post(
    session: Session,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match session.id {
        Some(_) => false,
        None => handler.db.insert_session(session).await,
    };

    if res {
        Ok(warp::reply())
    } else {
        Err(warp::reject())
    }
}

pub async fn users_get(
    auth_token: String,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !is_admin(auth_token, handler.auth_key) {
        return Err(warp::reject());
    }

    let res = handler.db.get_users().await;

    match res {
        Some(json) => Ok(warp::reply::json(&json)),
        None => Err(warp::reject()),
    }
}

pub async fn user_get(
    id: String,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    let s = String::from_utf8(decode(&unescape(&id).into_owned()).unwrap()).unwrap();
    let res = handler
        .db
        .get_user(u128::from_str_radix(&s, 10).unwrap())
        .await;

    match res {
        Some(json) => Ok(warp::reply::json(&json)),
        None => Err(warp::reject()),
    }
}

pub async fn user_post(
    user: User,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    if user.username == "admin" || user.username == "client" {
        return Err(warp::reject());
    }

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
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler.clone()))
        .and_then(sessions_get);

    let get = warp::get()
        .and(warp::path("sessions"))
        .and(warp::path::param())
        .and(with_handler(handler.clone()))
        .and_then(session_get);

    let post = warp::post()
        .and(warp::path("sessions"))
        .and(warp::body::json())
        .and(with_handler(handler))
        .and_then(sessions_post);

    get.or(list).or(post)
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
