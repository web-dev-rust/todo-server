use actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use diesel_migrations::run_pending_migrations;
use log::{debug, error};
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, CreateTableInput, DynamoDb, DynamoDbClient, KeySchemaElement,
    ListTablesInput, ProvisionedThroughput,
};
use std::env;

#[cfg(feature = "dynamo")]
pub fn client() -> DynamoDbClient {
    DynamoDbClient::new(Region::Custom {
        name: String::from("us-east-1"),
        endpoint: String::from("http://localhost:8000"),
    })
}

#[cfg(not(feature = "dynamo"))]
pub fn client() -> DynamoDbClient {
    DynamoDbClient::new(Region::Custom {
        name: String::from("julia-home"),
        endpoint: String::from("http://dynamodb:8000"),
    })
}

pub static TODO_CARD_TABLE: &str = "TODO_CARDS";

pub fn create_table() {
    let client = client();
    let list_tables_input: ListTablesInput = Default::default();
    run_migrations();

    match client.list_tables(list_tables_input).sync() {
        Ok(list) => {
            match list.table_names {
                Some(table_vec) => {
                    if table_vec.len() > 0 {
                        error!("Table already exists and has more then one item");
                    } else {
                        create_table_input()
                    }
                }
                None => create_table_input(),
            };
        }
        Err(_) => {
            create_table_input();
        }
    }
}

fn run_migrations() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    match run_pending_migrations(&pg_conn) {
        Ok(_) => debug!("auth database created"),
        Err(_) => error!("auth database creation failed"),
    };
}

fn create_table_input() {
    let client = client();

    let create_table_input = CreateTableInput {
        table_name: TODO_CARD_TABLE.to_string(),
        key_schema: vec![KeySchemaElement {
            attribute_name: "id".into(),
            key_type: "HASH".into(),
        }],
        attribute_definitions: vec![AttributeDefinition {
            attribute_name: "id".into(),
            attribute_type: "S".into(),
        }],
        provisioned_throughput: Some(ProvisionedThroughput {
            read_capacity_units: 1,
            write_capacity_units: 1,
        }),
        ..CreateTableInput::default()
    };

    match client.create_table(create_table_input).sync() {
        Ok(output) => {
            debug!("Table created {:?}", output);
        }
        Err(error) => {
            error!("Could not create table due to error: {:?}", error);
        }
    }
}

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn db_executor_address() -> Addr<DbExecutor> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    SyncArbiter::start(4, move || DbExecutor(pool.clone()))
}
