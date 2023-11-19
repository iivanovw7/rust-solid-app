use crate::handlers::{auth, health, user};
use crate::middleware::auth::Auth as AuthMiddleware;
use actix_web::web::{scope, ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        // Healthcheck
        .service(health::get_health)
        .service(
            // Protected api
            scope("/api/v1")
                .wrap(AuthMiddleware)
                .service(
                    // Auth scope
                    scope("/auth").service(auth::login).service(auth::logout),
                )
                .service(
                    // User scope
                    scope("/user")
                        .service(user::get_user)
                        .service(user::get_users)
                        .service(user::delete_user)
                        .service(user::update_user)
                        .service(user::create_user),
                ),
        );
}
