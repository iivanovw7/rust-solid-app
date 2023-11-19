#[cfg(test)]
pub mod tests {
    use crate::config::CONFIG;
    use crate::database::{init_pool, Pool};
    use crate::state::{new_state, AppState};
    use actix_web::web::Data;

    pub fn app_state() -> AppState<'static, String> {
        new_state::<String>()
    }

    pub fn get_pool() -> Pool {
        init_pool(CONFIG.clone()).unwrap()
    }

    pub fn get_data_pool() -> Data<Pool> {
        Data::new(get_pool())
    }
}
