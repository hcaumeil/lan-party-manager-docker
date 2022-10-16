use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    ip4: String,
    mac: String,
    user_id: Option<i32>,
    internet: bool,
}

impl Session {
    pub fn new(ip4: String, mac: String) -> Self {
        Self {
            ip4,
            mac,
            user_id: None,
            internet: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<u128>,
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
