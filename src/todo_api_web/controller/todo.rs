use actix_web::{HttpResponse, web, Responder};
use crate::{
    todo_api::{
        db::todo::put_todo,
        adapter
    },
    todo_api_web::model::{TodoCard, TodoIdResponse}
};


pub async fn create_todo(info: web::Json<TodoCard>) -> impl Responder {
    let todo_card = adapter::todo_json_to_db(info, uuid::Uuid::new_v4());

    match put_todo(todo_card) {
        None => HttpResponse::BadRequest().body("Failed to create todo card"),
        Some(id) => HttpResponse::Created()
            .content_type("application/json")
            .body(serde_json::to_string(&TodoIdResponse::new(id)).expect("Failed to serialize todo card"))
    }
}