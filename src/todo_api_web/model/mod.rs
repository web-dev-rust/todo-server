use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub is_done: bool,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    Todo,
    Doing,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoCard {
    pub title: String,
    pub description: String,
    pub owner: Uuid,
    pub tasks: Vec<Task>,
    pub state: State,
}

#[derive(Serialize, Deserialize)]
pub struct TodoIdResponse {
    id: Uuid,
}

impl TodoIdResponse {
    pub fn new(id: Uuid) -> Self {
        TodoIdResponse { id: id }
    }

    #[allow(dead_code)]
    pub fn get_id(self) -> String {
        format!("{}", self.id)
    }
}
