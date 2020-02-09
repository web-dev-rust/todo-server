use crate::todo_api::db::helpers::client;

#[derive(Clone)]
pub struct Clients {
    pub dynamo: rusoto_dynamodb::DynamoDbClient,
}

impl Clients {
    pub fn new() -> Self {
        Self { dynamo: client() }
    }
}
