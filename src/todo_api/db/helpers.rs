use log::{debug, error};
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, CreateTableInput, DynamoDb, DynamoDbClient, KeySchemaElement,
    ListTablesInput, ProvisionedThroughput,
};

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
            // println!("Output: {:?}", output);
        }
        Err(error) => {
            error!("Could not create table due to error: {:?}", error);
            // println!("Error: {:?}", error);
        }
    }
}
