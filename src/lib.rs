#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

mod schema;
pub mod todo_api;
pub mod todo_api_web;
