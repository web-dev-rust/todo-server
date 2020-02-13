use crate::schema::*;
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "auth_user"]
pub struct User {
    email: String,
    id: uuid::Uuid,
    password: String,
    expires_at: chrono::NaiveDateTime,
}

impl User {
    pub fn from(email: String, password: String) -> Self {
        let utc = crate::todo_api::db::helpers::one_day_from_now();

        Self {
            email: email,
            id: uuid::Uuid::new_v4(),
            password: password,
            expires_at: utc.naive_utc(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;

    #[test]
    fn user_is_correctly_created() {
        let user = User::from(String::from("email"), String::from("password"));
        let rx = Regex::new("[0-9]{4}-[0-1]{1}[0-9]{1}-[0-3]{1}[0-9]{1} [0-2]{1}[0-9]{1}:[0-6]{1}[0-9]{1}:[0-6]{1}[0-9]{1}").unwrap();

        assert_eq!(user.email, String::from("email"));
        assert_eq!(user.password, String::from("password"));
        assert!(uuid::Uuid::parse_str(&user.id.to_string()).is_ok());
        assert!(rx.is_match(&format!("{}", user.expires_at.format("%Y-%m-%d %H:%M:%S"))));
    }
}
