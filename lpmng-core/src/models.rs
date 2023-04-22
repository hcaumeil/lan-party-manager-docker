use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: Option<String>,
    pub ip4: String,
    pub user_id: Option<String>,
    pub internet: bool,
    pub date_time: NaiveDateTime,
}

impl Session {
    pub fn new(ip4: String) -> Self {
        Self {
            id: None,
            ip4,
            user_id: None,
            internet: false,
            date_time: NaiveDateTime::MIN,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub role: String,
    pub is_allowed: bool,
}

impl User {
    pub fn new(
        username: String,
        firstname: String,
        lastname: String,
        email: String,
        password: String,
        phone: String,
        role: String,
    ) -> Self {
        Self {
            id: None,
            username,
            firstname,
            lastname,
            email,
            password,
            phone,
            role,
            is_allowed: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub biscuit: String,
    pub role: String,
    pub user_id: Option<String>,
}
