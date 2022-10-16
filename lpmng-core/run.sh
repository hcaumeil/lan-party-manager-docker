export DATABASE_URL=postgres://corpau@localhost/lpmng
sqlx migrate run
cargo sqlx prepare
cargo r
