use crate::config::config;
use diesel::{PgConnection, r2d2::ConnectionManager};

// auto-connect to DB, keep pool global
lazy_static::lazy_static! {
    pub static ref DB_CONN_POOL: Pool = connect_database();
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PooledConnection =
    r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

// METHODS
pub fn get() -> Result<PooledConnection, r2d2::Error> {
    DB_CONN_POOL.get()
}

fn connect_database() -> Pool {
    let cfg = config();

    let manager = ConnectionManager::<PgConnection>::new(&cfg.database_url);

    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    return pool;
}
