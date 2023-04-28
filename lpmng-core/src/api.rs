use std::net::SocketAddr;
use std::{convert::Infallible, path::Path};

use base64_url::{decode, encode, unescape};
use biscuit_auth::PrivateKey;
use chrono::Utc;
use regex::Regex;
use serde_json;
use warp::{self, Filter, Rejection, Reply};

use lpmng_mq;
use lpmng_mq::client::agent::RouterRequest;

use crate::models::UserPatch;
use crate::{
    auth::{build_token, check_admin, check_id, hash},
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

pub fn is_user(id: String, auth_token: String, private_key: PrivateKey) -> bool {
    let mut split = auth_token.split(" ");

    if split.clone().count() != 2 {
        return false;
    }

    if split.clone().nth(0).unwrap() != "Bearer" {
        return false;
    }

    check_id(id, split.nth(1).unwrap().into(), private_key)
}

pub async fn login_post(
    json: serde_json::Value,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    return if let Some(login) = json.get("login") {
        if let Some(password) = json.get("password") {
            if (login.as_str().unwrap() == "admin"
                && password.as_str().unwrap() == handler.admin_key)
                || (login.as_str().unwrap() == "client"
                    && password.as_str().unwrap() == handler.client_key)
            {
                return match build_token("admin".into(), "0".to_string(), handler.auth_key) {
                    Some(t) => Ok(warp::reply::json(&Credentials {
                        biscuit: t,
                        role: "admin".into(),
                        user_id: None,
                    })),
                    None => Err(warp::reject()),
                };
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

                match build_token(role.to_owned(), id.clone(), handler.auth_key) {
                    Some(t) => Ok(warp::reply::json(&Credentials {
                        biscuit: t,
                        role,
                        user_id: Some(encode(&id)),
                    })),
                    None => Err(warp::reject()),
                }
            } else {
                Err(warp::reject())
            }
        } else {
            Err(warp::reject())
        }
    } else {
        Err(warp::reject())
    };
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
    id: String,
    auth_token: String,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id = String::from_utf8(decode(&unescape(&id).into_owned()).unwrap()).unwrap();

    if !is_admin(auth_token.clone(), handler.clone().auth_key)
        && !is_user(id.clone(), auth_token, handler.auth_key)
    {
        return Err(warp::reject());
    }

    Ok(warp::reply::json(
        &handler.db.get_session_by_user_id(id).await,
    ))
}

pub async fn session_post(
    session: Session,
    auth_token: String,
    addr: Option<SocketAddr>,
    mut handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    if session.user_id.clone().is_none()
        || !is_user(
            session.user_id.clone().expect("Should have some user_id"),
            auth_token,
            handler.auth_key,
        )
    {
        return Err(warp::reject());
    }

    let addr = match addr {
        None => return Err(warp::reject()),
        Some(a) => a.ip().to_string(),
    };

    let old_session = handler
        .db
        .get_session_by_user_id(session.user_id.clone().expect("Can't be none"))
        .await;
    let authorized = handler
        .db
        .get_user(session.user_id.clone().expect("Can't be none"))
        .await
        .expect("Can't be none")
        .is_allowed;

    let res = match old_session {
        None => match session.id {
            None => {
                handler
                    .db
                    .insert_session(Session {
                        id: None,
                        ip4: addr,
                        user_id: session.user_id,
                        internet: false,
                        date_time: Utc::now().naive_utc(),
                    })
                    .await
            }
            Some(_) => false,
        },
        Some(old) => match session.id {
            None => false,
            Some(id) => {
                if old.ip4 != addr {
                    if old.internet {
                        handler
                            .router
                            .send(RouterRequest {
                                action: "remove".to_string(),
                                body: old.ip4,
                            })
                            .await;
                        if authorized && old.internet {
                            handler
                                .router
                                .send(RouterRequest {
                                    action: "add".to_string(),
                                    body: addr.clone(),
                                })
                                .await;
                            handler
                                .db
                                .update_session(Session {
                                    id: Some(id),
                                    ip4: addr,
                                    user_id: old.user_id,
                                    internet: true,
                                    date_time: Utc::now().naive_utc(),
                                })
                                .await
                        } else {
                            false
                        }
                    } else {
                        if authorized {
                            handler
                                .router
                                .send(RouterRequest {
                                    action: "add".to_string(),
                                    body: addr.clone(),
                                })
                                .await;
                            handler
                                .db
                                .update_session(Session {
                                    id: Some(id),
                                    ip4: addr,
                                    user_id: old.user_id,
                                    internet: true,
                                    date_time: Utc::now().naive_utc(),
                                })
                                .await
                        } else {
                            false
                        }
                    }
                } else {
                    if authorized && session.internet {
                        if old.internet {
                            false
                        } else {
                            handler
                                .router
                                .send(RouterRequest {
                                    action: "add".to_string(),
                                    body: addr.clone(),
                                })
                                .await;
                            handler
                                .db
                                .update_session(Session {
                                    id: Some(id),
                                    ip4: addr,
                                    user_id: old.user_id,
                                    internet: true,
                                    date_time: Utc::now().naive_utc(),
                                })
                                .await
                        }
                    } else {
                        if session.internet {
                            false
                        } else {
                            handler
                                .router
                                .send(RouterRequest {
                                    action: "remove".to_string(),
                                    body: addr.clone(),
                                })
                                .await;
                            handler
                                .db
                                .update_session(Session {
                                    id: Some(id),
                                    ip4: addr,
                                    user_id: old.user_id,
                                    internet: false,
                                    date_time: Utc::now().naive_utc(),
                                })
                                .await
                        }
                    }
                }
            }
        },
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
    auth_token: String,
    handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id = String::from_utf8(decode(&unescape(&id).into_owned()).unwrap()).unwrap();

    if !is_admin(auth_token.clone(), handler.clone().auth_key)
        && !is_user(id.clone(), auth_token, handler.auth_key)
    {
        return Err(warp::reject());
    }

    let res = handler.db.get_user(id.clone()).await;
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

    let string_checker = |s: String| s.is_empty() || s.starts_with(" ");

    let email_regex = Regex::new(
        r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            ",
    )
    .unwrap();

    let mut object = user.clone();
    object.password = hash(user.password.clone());
    object.role = "user".to_string();
    object.is_allowed = false;

    if string_checker(user.lastname)
        || string_checker(user.username)
        || string_checker(user.password)
        || string_checker(user.firstname)
        || !email_regex.is_match(&user.email)
    {
        return Err(warp::reject());
    }

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

pub async fn user_patch(
    user: UserPatch,
    auth_token: String,
    mut handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !is_admin(auth_token, handler.auth_key) {
        return Err(warp::reject());
    }

    match handler.db.get_user(user.id.clone()).await {
        None => return Err(warp::reject()),
        Some(u) => {
            let new = User {
                id: Some(user.id.clone()),
                username: user.username.unwrap_or(u.username),
                firstname: user.firstname.unwrap_or(u.firstname),
                lastname: user.lastname.unwrap_or(u.lastname),
                email: user.email.unwrap_or(u.email),
                password: u.password,
                phone: user.phone.unwrap_or(u.phone),
                role: user.role.unwrap_or(u.role),
                is_allowed: user.is_allowed.unwrap_or(u.is_allowed),
            };
            handler.db.update_user(new).await;
            if user.is_allowed == Some(false) {
                match handler.db.get_session_by_user_id(user.id).await {
                    None => {}
                    Some(session) => {
                        if session.internet {
                            handler
                                .router
                                .send(RouterRequest {
                                    action: "remove".to_string(),
                                    body: session.ip4.clone(),
                                })
                                .await;
                            if !handler
                                .db
                                .update_session(Session {
                                    id: session.id,
                                    ip4: session.ip4,
                                    user_id: session.user_id,
                                    internet: false,
                                    date_time: Utc::now().naive_utc(),
                                })
                                .await
                            {
                                return Err(warp::reject());
                            }
                        }
                    }
                }
            }
            Ok(warp::reply())
        }
    }
}

pub async fn user_delete(
    user: UserPatch,
    auth_token: String,
    mut handler: ApiHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !is_admin(auth_token, handler.auth_key) {
        return Err(warp::reject());
    }

    match handler.db.get_user(user.id.clone()).await {
        None => return Err(warp::reject()),
        Some(u) => {
            if u.is_allowed {
                match handler.db.get_session_by_user_id(user.id.clone()).await {
                    None => {}
                    Some(session) => {
                        if session.internet {
                            handler
                                .router
                                .send(RouterRequest {
                                    action: "remove".to_string(),
                                    body: session.ip4.clone(),
                                })
                                .await;
                        }
                        if !handler.db.delete_session(session.id.unwrap()).await {
                            return Err(warp::reject());
                        }
                    }
                }
            }

            if handler.db.delete_user(user.id).await {
                Ok(warp::reply())
            } else {
                Err(warp::reject())
            }
        }
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
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler.clone()))
        .and_then(session_get);

    let post = warp::post()
        .and(warp::path("sessions"))
        .and(warp::body::json())
        .and(warp::header::<String>("Authorization"))
        .and(warp::addr::remote())
        .and(with_handler(handler))
        .and_then(session_post);

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
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler.clone()))
        .and_then(user_get);

    let post = warp::post()
        .and(warp::path("users"))
        .and(warp::body::json())
        .and(with_handler(handler.clone()))
        .and_then(user_post);

    let patch = warp::patch()
        .and(warp::path("users"))
        .and(warp::body::json())
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler.clone()))
        .and_then(user_patch);

    let delete = warp::delete()
        .and(warp::path("users"))
        .and(warp::body::json())
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler))
        .and_then(user_delete);

    get.or(list).or(post).or(patch).or(delete)
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

pub async fn get_ip(addr: Option<SocketAddr>) -> Result<impl warp::Reply, warp::Rejection> {
    match addr {
        None => Err(warp::reject()),
        Some(ip) => Ok(warp::reply::html(ip.ip().to_string())),
    }
}

pub fn get_ip_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("myip"))
        .and(warp::addr::remote())
        .and_then(get_ip)
}

pub fn api_routes(
    handler: ApiHandler,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("api").and(
        sessions_routes(handler.clone())
            .or(users_routes(handler.clone()))
            .or(login_routes(handler))
            .or(get_ip_route()),
    )
}

pub fn public_route(
    public: String,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    assert!(
        Path::new(&public).exists(),
        "[ASSERTION] unable to find the static html directory"
    );

    warp::get().and(warp::fs::dir(public))
}
