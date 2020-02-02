use crate::todo_api::{db::helpers::TODO_CARD_TABLE, model::TodoCardDb};
use rusoto_dynamodb::PutItemInput;
use uuid::Uuid;

#[cfg(not(feature = "dynamo"))]
pub fn put_todo(todo_card: TodoCardDb) -> Option<Uuid> {
    use crate::todo_api::db::helpers::client;
    use rusoto_dynamodb::DynamoDb;

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

#[cfg(feature = "dynamo")]
pub fn put_todo(todo_card: TodoCardDb) -> Option<Uuid> {
    let _ = PutItemInput {
        table_name: TODO_CARD_TABLE.to_string(),
        item: todo_card.clone().into(),
        ..PutItemInput::default()
    };
    Some(todo_card.id)
}
