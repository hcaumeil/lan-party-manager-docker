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

pub struct User {
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    tel: String,
    role: String,
    is_allowed: bool,
}

impl User {
    pub fn new(
        username: String,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        tel: String,
        role: String,
    ) -> Self {
        Self {
            username,
            first_name,
            last_name,
            email,
            password,
            tel,
            role,
            is_allowed: false,
        }
    }
}
