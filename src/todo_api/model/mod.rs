use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;
use uuid::Uuid;

pub mod auth;
pub mod core;
pub mod error;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskDb {
    pub is_done: bool,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StateDb {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TodoCardDb {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub owner: Uuid,
    pub tasks: Vec<TaskDb>,
    pub state: StateDb,
}

impl TodoCardDb {
    #[allow(dead_code)]
    pub fn get_id(self) -> Uuid {
        self.id
    }
}

impl TaskDb {
    fn to_db_val(self) -> AttributeValue {
        let mut tasks_hash = HashMap::new();
        tasks_hash.insert("title".to_string(), val!(S => Some(self.title.clone())));
        tasks_hash.insert("is_done".to_string(), val!(B => self.is_done));
        val!(M => tasks_hash)
    }
}

impl std::fmt::Display for StateDb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<HashMap<String, AttributeValue>> for TodoCardDb {
    fn into(self) -> HashMap<String, AttributeValue> {
        let mut todo_card = HashMap::new();
        todo_card.insert("id".to_string(), val!(S => Some(self.id.to_string())));
        todo_card.insert("title".to_string(), val!(S => Some(self.title)));
        todo_card.insert("description".to_string(), val!(S => Some(self.description)));
        todo_card.insert("owner".to_string(), val!(S => Some(self.owner.to_string())));
        todo_card.insert("state".to_string(), val!(S => Some(self.state.to_string())));
        todo_card.insert("tasks".to_string(), 
            val!(L => self.tasks.into_iter().map(|t| t.to_db_val()).collect::<Vec<AttributeValue>>()));
        todo_card
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_db_to_db_val() {
        let actual = TaskDb {
            title: "blob".to_string(),
            is_done: true,
        }
        .to_db_val();
        let mut tasks_hash = HashMap::new();
        tasks_hash.insert("title".to_string(), val!(S => Some("blob".to_string())));
        tasks_hash.insert("is_done".to_string(), val!(B => true));
        let expected = val!(M => tasks_hash);

        assert_eq!(actual, expected);
    }

    #[test]
    fn todo_card_db_to_db_val() {
        let id = uuid::Uuid::new_v4();
        let actual: HashMap<String, AttributeValue> = TodoCardDb {
            id: id,
            title: "title".to_string(),
            description: "description".to_string(),
            owner: id,
            state: StateDb::Done,
            tasks: vec![TaskDb {
                is_done: true,
                title: "title".to_string(),
            }],
        }
        .into();
        let mut expected = HashMap::new();
        expected.insert("id".to_string(), val!(S => Some(id.to_string())));
        expected.insert("title".to_string(), val!(S => Some("title".to_string())));
        expected.insert(
            "description".to_string(),
            val!(S => Some("description".to_string())),
        );
        expected.insert("owner".to_string(), val!(S => Some(id.to_string())));
        expected.insert(
            "state".to_string(),
            val!(S => Some(StateDb::Done.to_string())),
        );
        expected.insert(
            "tasks".to_string(),
            val!(L => vec![TaskDb {is_done: true, title: "title".to_string()}.to_db_val()]),
        );

        assert_eq!(actual, expected);
    }
}
