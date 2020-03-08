use crate::{
    todo_api::{
        adapter,
        db::todo::{get_todos, put_todo, get_todo_by_id},
    },
    todo_api_web::model::{
        http::Clients,
        todo::{TodoCard, TodoCardsResponse, TodoIdResponse},
    },
};
use actix_web::{web, HttpResponse, Responder};
use log::error;

pub async fn create_todo(state: web::Data<Clients>, info: web::Json<TodoCard>) -> impl Responder {
    let todo_card = adapter::todo_json_to_db(info, uuid::Uuid::new_v4());

    match put_todo(state.dynamo.clone(), todo_card) {
        None => {
            error!("Failed to create todo card");
            HttpResponse::BadRequest().body("Failed to create todo card")
        }
        Some(id) => HttpResponse::Created()
            .content_type("application/json")
            .json(TodoIdResponse::new(id))
    }
}

pub async fn show_all_todo(state: web::Data<Clients>) -> impl Responder {
    match get_todos(state.dynamo.clone()) {
        None => {
            error!("Failed to read todo cards");
            HttpResponse::InternalServerError().body("Failed to read todo cards")
        }
        Some(todos) => HttpResponse::Ok().content_type("application/json")
            .json(TodoCardsResponse { cards: todos })
    }
}

pub async fn show_by_id(id: web::Path<String>, state: web::Data<Clients>) -> impl Responder {
    let uuid = id.to_string();

    if uuid::Uuid::parse_str(&uuid).is_err() {
        return HttpResponse::BadRequest().body("Id must be a Uuid::V4");
    }

    match get_todo_by_id(uuid, state.dynamo.clone()) {
        None => {
            error!("Failed to read todo cards");
            HttpResponse::NotFound().finish()
        }
        Some(todo_id) => HttpResponse::Ok().content_type("application/json")
            .json(todo_id)
    }
}