use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Task {
    pub is_done: bool,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum State {
    Todo,
    Doing,
    Done,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TodoCard {
    pub id: Option<Uuid>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TodoCardUpdate {
    pub description: Option<String>,
    pub state: Option<State>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct TodoCardsResponse {
    pub cards: Vec<TodoCard>,
}

impl State {
    pub fn from(s: String) -> State {
        match &s[..] {
            "Doing" => State::Doing,
            "Done" => State::Done,
            _ => State::Todo,
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
