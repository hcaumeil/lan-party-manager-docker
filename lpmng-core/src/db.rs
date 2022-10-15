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
}
