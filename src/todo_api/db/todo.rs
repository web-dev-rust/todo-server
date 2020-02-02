use crate::todo_api::{
    db::helpers::{client, TODO_CARD_TABLE},
    model::TodoCardDb,
};
use rusoto_dynamodb::{DynamoDb, PutItemInput};
use uuid::Uuid;

pub fn put_todo(todo_card: TodoCardDb) -> Option<Uuid> {
    let client = client();
    let put_item = PutItemInput {
        table_name: TODO_CARD_TABLE.to_string(),
        item: todo_card.clone().into(),
        ..PutItemInput::default()
    };

    match client.put_item(put_item).sync() {
        Ok(_) => Some(todo_card.id),
        Err(_) => None,
    }
}
