use actix_web::{HttpResponse, web, Responder};
use log::{error};
use crate::{
    todo_api::{
        db::todo::{put_todo, get_todos},
        adapter
    },
    todo_api_web::model::{TodoCard, TodoIdResponse, TodoCardsResponse}
};


pub async fn create_todo(info: web::Json<TodoCard>) -> impl Responder {
    let todo_card = adapter::todo_json_to_db(info, uuid::Uuid::new_v4());

    match put_todo(todo_card) {
        None => {
            error!("Failed to create todo card");
            HttpResponse::BadRequest().body("Failed to create todo card")
        },
        Some(id) => HttpResponse::Created()
            .content_type("application/json")
            .body(serde_json::to_string(&TodoIdResponse::new(id)).expect("Failed to serialize todo card"))
    }
}

pub async fn show_all_todo() -> impl Responder {
    match get_todos() {
        None => {
            error!("Failed to read todo cards");
            HttpResponse::InternalServerError().body("Failed to read todo cards")
        },
        Some(todos) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&TodoCardsResponse{cards: todos}).expect("Failed to serialize todo cards")),
    }
}