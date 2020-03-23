use crate::todo_api::{adapter, db::helpers::TODO_CARD_TABLE, model::TodoCardDb};
use crate::todo_api_web::model::todo::{TodoCard, TodoCardUpdate};
use log::{debug, error};
use rusoto_dynamodb::{DynamoDbClient, PutItemInput, ScanInput, UpdateItemInput};
use uuid::Uuid;

#[cfg(not(feature = "dbtest"))]
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

#[cfg(not(feature = "dbtest"))]
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

#[cfg(not(feature = "dbtest"))]
pub fn get_todo_by_id(id: String, client: DynamoDbClient) -> Option<TodoCard> {
    use rusoto_dynamodb::{AttributeValue, DynamoDb};
    use std::collections::HashMap;

    let mut _map = HashMap::new();
    let mut attr = AttributeValue::default();
    attr.s = Some(id);
    _map.insert(String::from(":id"), attr);

    let scan_item = ScanInput {
        limit: Some(100i64),
        table_name: TODO_CARD_TABLE.to_string(),
        filter_expression: Some("id = :id".into()),
        expression_attribute_values: Some(_map),
        ..ScanInput::default()
    };

    match client.scan(scan_item).sync() {
        Ok(resp) => {
            let todo_id = adapter::scanoutput_to_todocards(resp);
            if todo_id.first().is_some() {
                debug!("Scanned {:?} todo cards", todo_id);
                Some(todo_id.first().unwrap().to_owned())
            } else {
                error!("Could find todocard with ID.");
                None
            }
        }
        Err(e) => {
            error!("Could not scan todocard due to error {:?}", e);
            None
        }
    }
}

#[cfg(not(feature = "dbtest"))]
pub fn update_todo_info(id: String, info: TodoCardUpdate, client: DynamoDbClient) -> bool {
    use rusoto_dynamodb::{AttributeValue, DynamoDb};
    use std::collections::HashMap;

    let expression = adapter::update_expression(&info);
    let attribute_values = adapter::expression_attribute_values(&info);
    let mut _map = HashMap::new();
    let mut attr = AttributeValue::default();
    attr.s = Some(id);
    _map.insert(String::from("id"), attr);

    let update = UpdateItemInput {
        table_name: TODO_CARD_TABLE.to_string(),
        key: _map,
        update_expression: expression,
        expression_attribute_values: attribute_values,
        ..UpdateItemInput::default()
    };

    match client.update_item(update).sync() {
        Ok(_) => true,
        Err(e) => {
            println!("failed due to {:?}", e);
            false
        }
    }
}

#[cfg(feature = "dbtest")]
pub fn update_todo_info(id: String, info: TodoCardUpdate, client: DynamoDbClient) -> bool {
    use rusoto_dynamodb::{AttributeValue, DynamoDb};
    use std::collections::HashMap;

    let expression = adapter::update_expression(&info);
    let attribute_values = adapter::expression_attribute_values(&info);
    let mut _map = HashMap::new();
    let mut attr = AttributeValue::default();
    attr.s = Some(id);
    _map.insert(String::from("id"), attr);

    let update = UpdateItemInput {
        table_name: TODO_CARD_TABLE.to_string(),
        key: _map,
        update_expression: expression,
        expression_attribute_values: attribute_values,
        ..UpdateItemInput::default()
    };

    true
}

#[cfg(feature = "dbtest")]
pub fn get_todos(_: DynamoDbClient) -> Option<Vec<TodoCard>> {
    use crate::todo_api_web::model::todo::{State, Task};
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

#[cfg(feature = "dbtest")]
pub fn put_todo(_: DynamoDbClient, todo_card: TodoCardDb) -> Option<Uuid> {
    let _ = PutItemInput {
        table_name: TODO_CARD_TABLE.to_string(),
        item: todo_card.clone().into(),
        ..PutItemInput::default()
    };
    Some(todo_card.id)
}

#[cfg(feature = "dbtest")]
pub fn get_todo_by_id(id: String, client: DynamoDbClient) -> Option<TodoCard> {
    use rusoto_dynamodb::{AttributeValue, DynamoDb};
    use std::collections::HashMap;
    use crate::todo_api_web::model::todo::{State, Task};

    let mut _map = HashMap::new();
    let mut attr = AttributeValue::default();
    attr.s = Some(id);
    _map.insert(String::from(":id"), attr);

    let scan_item = ScanInput {
        limit: Some(100i64),
        table_name: TODO_CARD_TABLE.to_string(),
        filter_expression: Some("id = :id".into()),
        expression_attribute_values: Some(_map),
        ..ScanInput::default()
    };

    Some(
        TodoCard {
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
        }
    )
}
