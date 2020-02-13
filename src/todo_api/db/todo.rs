use crate::todo_api::{adapter, db::helpers::TODO_CARD_TABLE, model::TodoCardDb};
use crate::todo_api_web::model::todo::TodoCard;
use log::{debug, error};
use rusoto_dynamodb::{DynamoDbClient, PutItemInput, ScanInput};
use uuid::Uuid;

#[cfg(not(feature = "dynamo"))]
pub fn put_todo(client: DynamoDbClient, todo_card: TodoCardDb) -> Option<Uuid> {
    use rusoto_dynamodb::DynamoDb;

    let put_item = PutItemInput {
        table_name: TODO_CARD_TABLE.to_string(),
        item: todo_card.clone().into(),
        ..PutItemInput::default()
    };

    match client.put_item(put_item).sync() {
        Ok(_) => {
            debug!("item created with id {:?}", todo_card.id);
            Some(todo_card.id)
        }
        Err(e) => {
            error!("error when creating item {:?}", e);
            None
        }
    }
}

#[cfg(not(feature = "dynamo"))]
pub fn get_todos(client: DynamoDbClient) -> Option<Vec<TodoCard>> {
    use rusoto_dynamodb::DynamoDb;

    let scan_item = ScanInput {
        limit: Some(100i64),
        table_name: TODO_CARD_TABLE.to_string(),
        ..ScanInput::default()
    };

    match client.scan(scan_item).sync() {
        Ok(resp) => {
            let todocards = adapter::scanoutput_to_todocards(resp);
            debug!("Scanned {:?} todo cards", todocards);
            Some(todocards)
        }
        Err(e) => {
            error!("Could not scan todocards due to error {:?}", e);
            None
        }
    }
}

#[cfg(feature = "dynamo")]
pub fn get_todos(_: DynamoDbClient) -> Option<Vec<TodoCard>> {
    use crate::todo_api_web::model::{State, Task};
    use rusoto_dynamodb::DynamoDb;

    let _ = ScanInput {
        limit: Some(100i64),
        table_name: TODO_CARD_TABLE.to_string(),
        ..ScanInput::default()
    };

    Some(vec![TodoCard {
        id: Some(uuid::Uuid::parse_str("be75c4d8-5241-4f1c-8e85-ff380c041664").unwrap()),
        title: String::from("This is a card"),
        description: String::from("This is the description of the card"),
        owner: uuid::Uuid::parse_str("ae75c4d8-5241-4f1c-8e85-ff380c041442").unwrap(),
        tasks: vec![
            Task {
                title: String::from("title 1"),
                is_done: true,
            },
            Task {
                title: String::from("title 2"),
                is_done: true,
            },
            Task {
                title: String::from("title 3"),
                is_done: false,
            },
        ],
        state: State::Doing,
    }])
}

#[cfg(feature = "dynamo")]
pub fn put_todo(_: DynamoDbClient, todo_card: TodoCardDb) -> Option<Uuid> {
    let _ = PutItemInput {
        table_name: TODO_CARD_TABLE.to_string(),
        item: todo_card.clone().into(),
        ..PutItemInput::default()
    };
    Some(todo_card.id)
}
