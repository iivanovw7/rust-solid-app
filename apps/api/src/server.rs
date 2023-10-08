use crate::database::add_pool;
use crate::routes::routes;
use crate::state::new_state;
use crate::{config::CONFIG, swagger::get_swagger_service};
use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;

use crate::auth::{get_cors_service, get_identity_service};

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let data = new_state::<String>();

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(add_pool)
            .wrap(Logger::default())
            .wrap(get_cors_service())
            .wrap(get_identity_service())
            .configure(add_pool)
            .configure(routes)
            .configure(get_swagger_service)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };

    server.run().await
}
