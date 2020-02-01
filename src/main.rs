#[macro_use] extern crate serde;

use actix_web::{App, HttpServer};

mod todo_api_web;
mod todo_api;

use todo_api_web::{
    routes::app_routes
};

use todo_api::db::helpers::create_table;

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().configure(app_routes)
//     })
//     .workers(num_cpus::get() + 2)
//     .bind("127.0.0.1:4000")
//     .unwrap()
//     .run()
//     .await
// }

fn main() {
    create_table();
}