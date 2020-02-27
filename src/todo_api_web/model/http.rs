use crate::todo_api::db::helpers::{client, db_executor_address, DbExecutor};
use actix::Addr;

#[derive(Clone)]
pub struct Clients {
    pub dynamo: rusoto_dynamodb::DynamoDbClient,
    pub postgres: Addr<DbExecutor>,
}

impl Clients {
    pub fn new() -> Self {
        Self {
            dynamo: client(),
            postgres: db_executor_address(),
        }
    }
}
