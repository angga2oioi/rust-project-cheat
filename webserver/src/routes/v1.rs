use actix_web::web::{ resource, get,post, ServiceConfig };

use crate::controllers::v1::{ user_controller::UserController };

pub fn register(config: &mut ServiceConfig) {
    config
        .service(
            resource("/users")
            .route(get().to(UserController::index))
            .route(post().to(UserController::post_user))
        )
        .service(resource("/users/{id}").route(get().to(UserController::get_id)));
}