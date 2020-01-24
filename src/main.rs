#[macro_use] extern crate serde;

use actix_web::{App, HttpServer};

mod todo_api_web;

use todo_api_web::{
    routes::app_routes
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(app_routes)
    })
    .workers(num_cpus::get() + 2)
    .bind("127.0.0.1:4000")
    .unwrap()
    .run()
    .await
}
