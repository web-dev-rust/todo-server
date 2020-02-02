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
    todo_api_web::model::{State, TodoCard},
};
use actix_web::web;
use uuid::Uuid;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        todo_api::model::{StateDb, TaskDb, TodoCardDb},
        todo_api_web::model::{State, Task, TodoCard},
    };
    use actix_web::web::Json;

    #[test]
    fn converts_json_to_db() {
        let id = uuid::Uuid::new_v4();
        let owner = uuid::Uuid::new_v4();
        let json = Json(TodoCard {
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
