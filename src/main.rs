#[macro_use] extern crate serde;

use actix_web::{web, App, HttpResponse, HttpServer};

mod todo_api_web;

use todo_api_web::controller::{
    pong, readiness,
    todo::create_todo
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/")
                .service(
                    web::scope("api/")
                        .route("create", web::post().to(create_todo))
                )
                .route("ping", web::get().to(pong))
                .route("~/ready", web::get().to(readiness))
                .route("", web::get().to(|| HttpResponse::NotFound())),
        )
    })
    .workers(num_cpus::get() + 2)
    .bind("127.0.0.1:4000")
    .unwrap()
    .run()
    .await
}
