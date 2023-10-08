use crate::config::{Config, CONFIG};
use actix_web::web;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PoolError},
};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(config: Config) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::new(config.database_url);

    Pool::builder().build(manager)
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let pool = init_pool(CONFIG.clone()).expect("Failed to create connection pool");

    cfg.app_data(pool);
}
