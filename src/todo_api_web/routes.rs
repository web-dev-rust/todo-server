use crate::todo_api_web::controller::{
    auth::{login, logout, signup_user},
    pong, readiness,
    todo::{create_todo, show_all_todo, show_by_id, update_todo},
};
use actix_web::{web, HttpResponse};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            .service(
                web::scope("api/")
                    .route("create", web::post().to(create_todo))
                    .route("index", web::get().to(show_all_todo))
                    .route("show/{id}", web::get().to(show_by_id))
                    .route("update/{id}", web::put().to(update_todo)),
            )
            .service(
                web::scope("auth/")
                    .route("signup", web::post().to(signup_user))
                    .route("login", web::post().to(login))
                    .route("logout", web::delete().to(logout)),
            )
            .route("ping", web::get().to(pong))
            .route("~/ready", web::get().to(readiness))
            .route("", web::get().to(|| HttpResponse::NotFound())),
    );
}
