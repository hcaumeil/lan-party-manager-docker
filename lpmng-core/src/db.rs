use crate::models::{Session, User};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool, Postgres, Transaction};

#[derive(Clone, Debug)]
pub struct DbHandler {
    pool: PgPool,
}

impl DbHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn connect() -> Option<Self> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|e| {
            eprintln!("[ERROR] $DATABASE_URL is not set ({})", e);
            std::process::exit(1);
        });

        match PgPoolOptions::new().connect(&url).await {
            Ok(p) => return Some(Self::new(p)),
            Err(_) => return None,
        }
    }

    pub async fn insert_user(&self, user: User) -> bool {
        let mut tx = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => {
                return false;
            }
        };

        match sqlx::query!(
            r#"
INSERT INTO users (username, firstname, lastname, email, password, phone, role, is_allowed)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
            user.username,
            user.firstname,
            user.lastname,
            user.email,
            user.password,
            user.phone,
            user.role,
            user.is_allowed
        )
        .execute(&mut tx)
        .await
        {
            Ok(_) => {}
            Err(_) => return false,
        };

        return match tx.commit().await {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    pub async fn get_users(&self) -> Option<Vec<User>> {
        let mut res: Vec<User> = Vec::new();

        match sqlx::query!(
            r#"
                SELECT * FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(q) => {
                for x in &q {
                    res.push(User {
                        username: x.username.to_string(),
                        firstname: x.firstname.to_string(),
                        lastname: x.lastname.to_string(),
                        email: x.email.to_string(),
                        password: x.password.to_string(),
                        phone: x.phone.to_string(),
                        role: x.role.to_string(),
                        is_allowed: x.is_allowed,
                    });
                }
            }
            Err(e) => return None,
        };

        Some(res)
    }
}
