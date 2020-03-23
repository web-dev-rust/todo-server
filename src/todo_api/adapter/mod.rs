pub mod auth;

#[macro_export]
macro_rules! val {
    (B => $bval:expr) => {{
        let mut attr = AttributeValue::default();
        attr.bool = Some($bval);
        attr
    }};
    (L => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.l = Some($val);
        attr
    }};
    (S => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.s = $val;
        attr
    }};
    (M => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.m = Some($val);
        attr
    }};
}

use crate::{
    todo_api::model::{StateDb, TaskDb, TodoCardDb},
    todo_api_web::model::todo::{State, Task, TodoCard, TodoCardUpdate},
};
use actix_web::web;
use rusoto_dynamodb::{ScanOutput, AttributeValue};
use uuid::Uuid;
use std::collections::HashMap;

pub fn todo_json_to_db(card: web::Json<TodoCard>, id: Uuid) -> TodoCardDb {
    TodoCardDb {
        id: id,
        title: card.title.clone(),
        description: card.description.clone(),
        owner: card.owner,
        tasks: card
            .tasks
            .iter()
            .map(|t| TaskDb {
                is_done: t.is_done,
                title: t.title.clone(),
            })
            .collect::<Vec<TaskDb>>(),
        state: match card.state {
            State::Doing => StateDb::Doing,
            State::Done => StateDb::Done,
            State::Todo => StateDb::Todo,
        },
    }
}

pub fn scanoutput_to_todocards(scan: ScanOutput) -> Vec<TodoCard> {
    scan.items
        .unwrap()
        .into_iter()
        .map(|item| TodoCard {
            id: Some(uuid::Uuid::parse_str(&item.get("id").unwrap().s.clone().unwrap()).unwrap()),
            owner: uuid::Uuid::parse_str(&item.get("owner").unwrap().s.clone().unwrap()).unwrap(),
            title: item.get("title").unwrap().s.clone().unwrap(),
            description: item.get("description").unwrap().s.clone().unwrap(),
            state: State::from(item.get("state_db").unwrap().s.clone().unwrap()),
            tasks: item
                .get("tasks")
                .unwrap()
                .l
                .clone()
                .unwrap()
                .iter()
                .map(|t| Task {
                    title: t
                        .clone()
                        .m
                        .unwrap()
                        .get("title")
                        .unwrap()
                        .s
                        .clone()
                        .unwrap(),
                    is_done: t
                        .clone()
                        .m
                        .unwrap()
                        .get("is_done")
                        .unwrap()
                        .bool
                        .clone()
                        .unwrap(),
                })
                .collect::<Vec<Task>>(),
        })
        .collect::<Vec<TodoCard>>()
}


pub fn update_expression(info: &TodoCardUpdate) -> Option<String> {
    let data = info.clone();
    match (data.description, data.state) {
        (Some(_), Some(_)) => Some(String::from("SET description = :d, state_db = :s")),
        (_, Some(_)) => Some(String::from("SET state_db = :s")),
        (Some(_), _) => Some(String::from("SET description = :d")),
        _ => None
    }
}

pub fn expression_attribute_values(info: &TodoCardUpdate) -> Option<HashMap<String, AttributeValue>> {
    let data = info.clone();
    match (data.description, data.state) {
        (Some(desc), Some(state)) => {
            let mut _map = HashMap::new();
            let mut attr_d = AttributeValue::default();
            attr_d.s = Some(String::from(desc));
            let mut attr_s = AttributeValue::default();
            attr_s.s = Some(String::from(state.to_string()));
            _map.insert(String::from(":d"), attr_d);
            _map.insert(String::from(":s"), attr_s);
            Some(_map)
        },
        (_, Some(state)) => {
            let mut _map = HashMap::new();
            let mut attr = AttributeValue::default();
            attr.s = Some(String::from(state.to_string()));
            _map.insert(String::from(":s"), attr);
            Some(_map)
        },
        (Some(desc), _) => {
            let mut _map = HashMap::new();
            let mut attr = AttributeValue::default();
            attr.s = Some(String::from(desc));
            _map.insert(String::from(":d"), attr);
            Some(_map)
        },
        _ => None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        todo_api::model::{StateDb, TaskDb, TodoCardDb},
        todo_api_web::model::todo::{State, Task, TodoCard},
    };
    use actix_web::web::Json;

    #[test]
    fn converts_json_to_db() {
        let id = uuid::Uuid::new_v4();
        let owner = uuid::Uuid::new_v4();
        let json = Json(TodoCard {
            id: None,
            title: "title".to_string(),
            description: "description".to_string(),
            owner: owner,
            state: State::Done,
            tasks: vec![Task {
                is_done: true,
                title: "title".to_string(),
            }],
        });
        let expected = TodoCardDb {
            id: id,
            title: "title".to_string(),
            description: "description".to_string(),
            owner: owner,
            state: StateDb::Done,
            tasks: vec![TaskDb {
                is_done: true,
                title: "title".to_string(),
            }],
        };
        assert_eq!(todo_json_to_db(json, id), expected);
    }
}

#[cfg(test)]
mod scan_to_cards {
    use super::scanoutput_to_todocards;
    use crate::todo_api_web::model::todo::{State, Task, TodoCard};
    use rusoto_dynamodb::{AttributeValue, ScanOutput};

    fn attr_values() -> std::collections::HashMap<String, AttributeValue> {
        let mut tasks_hash = std::collections::HashMap::new();
        tasks_hash.insert(
            "title".to_string(),
            AttributeValue {
                b: None,
                bool: None,
                bs: None,
                l: None,
                m: None,
                n: None,
                ns: None,
                null: None,
                s: Some("blob".to_string()),
                ss: None,
            },
        );
        tasks_hash.insert(
            "is_done".to_string(),
            AttributeValue {
                b: None,
                bool: Some(true),
                bs: None,
                l: None,
                m: None,
                n: None,
                ns: None,
                null: None,
                s: None,
                ss: None,
            },
        );
        let mut hash = std::collections::HashMap::new();
        hash.insert(
            "title".to_string(),
            AttributeValue {
                b: None,
                bool: None,
                bs: None,
                l: None,
                m: None,
                n: None,
                ns: None,
                null: None,
                s: Some("title".to_string()),
                ss: None,
            },
        );
        hash.insert(
            "description".to_string(),
            AttributeValue {
                b: None,
                bool: None,
                bs: None,
                l: None,
                m: None,
                n: None,
                ns: None,
                null: None,
                s: Some("description".to_string()),
                ss: None,
            },
        );
        hash.insert(
            "owner".to_string(),
            AttributeValue {
                b: None,
                bool: None,
                bs: None,
                l: None,
                m: None,
                n: None,
                ns: None,
                null: None,
                s: Some("90e700b0-2b9b-4c74-9285-f5fc94764995".to_string()),
                ss: None,
            },
        );
        hash.insert(
            "id".to_string(),
            AttributeValue {
                b: None,
                bool: None,
                bs: None,
                l: None,
                m: None,
                n: None,
                ns: None,
                null: None,
                s: Some("646b670c-bb50-45a4-ba08-3ab684bc4e95".to_string()),
                ss: None,
            },
        );
        hash.insert(
            "state_db".to_string(),
            AttributeValue {
                b: None,
                bool: None,
                bs: None,
                l: None,
                m: None,
                n: None,
                ns: None,
                null: None,
                s: Some("Done".to_string()),
                ss: None,
            },
        );
        hash.insert(
            "tasks".to_string(),
            AttributeValue {
                b: None,
                bool: None,
                bs: None,
                l: Some(vec![AttributeValue {
                    b: None,
                    bool: None,
                    bs: None,
                    l: None,
                    m: Some(tasks_hash),
                    n: None,
                    ns: None,
                    null: None,
                    s: None,
                    ss: None,
                }]),
                m: None,
                n: None,
                ns: None,
                null: None,
                s: None,
                ss: None,
            },
        );
        hash
    }

    fn scan_with_one() -> ScanOutput {
        let hash = attr_values();

        ScanOutput {
            consumed_capacity: None,
            count: Some(1),
            items: Some(vec![hash]),
            last_evaluated_key: None,
            scanned_count: Some(1),
        }
    }

    fn scan_with_two() -> ScanOutput {
        let hash = attr_values();

        ScanOutput {
            consumed_capacity: None,
            count: Some(2),
            items: Some(vec![hash.clone(), hash]),
            last_evaluated_key: None,
            scanned_count: Some(2),
        }
    }

    #[test]
    fn scanoutput_has_one_item() {
        let scan = scan_with_one();
        let todos = vec![TodoCard {
            title: "title".to_string(),
            description: "description".to_string(),
            state: State::Done,
            id: Some(uuid::Uuid::parse_str("646b670c-bb50-45a4-ba08-3ab684bc4e95").unwrap()),
            owner: uuid::Uuid::parse_str("90e700b0-2b9b-4c74-9285-f5fc94764995").unwrap(),
            tasks: vec![Task {
                is_done: true,
                title: "blob".to_string(),
            }],
        }];

        assert_eq!(scanoutput_to_todocards(scan), todos)
    }

    #[test]
    fn scanoutput_has_two_items() {
        let scan = scan_with_two();
        let todo = TodoCard {
            title: "title".to_string(),
            description: "description".to_string(),
            state: State::Done,
            id: Some(uuid::Uuid::parse_str("646b670c-bb50-45a4-ba08-3ab684bc4e95").unwrap()),
            owner: uuid::Uuid::parse_str("90e700b0-2b9b-4c74-9285-f5fc94764995").unwrap(),
            tasks: vec![Task {
                is_done: true,
                title: "blob".to_string(),
            }],
        };
        let todos = vec![todo.clone(), todo];

        assert_eq!(scanoutput_to_todocards(scan), todos)
    }
}

#[cfg(test)]
mod update_expression_test {
    use super::update_expression;
    use crate::todo_api_web::model::todo::{State, TodoCardUpdate};

    #[test]
    fn description_and_state() {
        let todo_update = TodoCardUpdate {description: Some("haiushdusd".to_string()), state: Some(State::Doing)};
        let expected = Some(String::from("SET description = :d, state_db = :s"));

        assert_eq!(expected, update_expression(&todo_update));
    }

    #[test]
    fn description() {
        let todo_update = TodoCardUpdate {description: Some("haiushdusd".to_string()), state: None};
        let expected = Some(String::from("SET description = :d"));

        assert_eq!(expected, update_expression(&todo_update));
    }

    #[test]
    fn state() {
        let todo_update = TodoCardUpdate {description: None, state: Some(State::Doing)};
        let expected = Some(String::from("SET state_db = :s"));

        assert_eq!(expected, update_expression(&todo_update));
    }

    #[test]
    fn none() {
        let todo_update = TodoCardUpdate {description: None, state: None};
        let expected = None;

        assert_eq!(expected, update_expression(&todo_update));
    }
}