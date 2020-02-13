#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SignUp {
    pub email: String,
    pub password: String,
}
