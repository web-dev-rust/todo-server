use actix_web::{HttpResponse, web, Responder};
use uuid::Uuid;
use crate::todo_api_web::model::{TodoCard, TodoIdResponse};

pub async fn create_todo(info: web::Json<TodoCard>) -> impl Responder {
    println!("{:?}", info);
    let new_id = Uuid::new_v4();
    HttpResponse::Created()
        .content_type("application/json")
        .body(serde_json::to_string(&TodoIdResponse::new(new_id)).expect("failed to serialize ContactsBatchResponseId")
    )
}