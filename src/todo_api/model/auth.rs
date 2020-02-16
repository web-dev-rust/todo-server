use crate::schema::*;
use bcrypt::{verify, BcryptResult};

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable)]
#[table_name = "auth_user"]
pub struct User {
    pub email: String,
    pub id: uuid::Uuid,
    #[cfg(test)] pub password: String,
    #[cfg(not(test))] password: String,
    #[cfg(test)] pub expires_at: chrono::NaiveDateTime,
    #[cfg(not(test))] expires_at: chrono::NaiveDateTime,
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

    pub fn verify(&self, pswd: String) -> BcryptResult<bool> {
        verify(pswd,&self.password)
    }

    pub fn get_id(self) -> String {
        self.id.to_string()
    }

    #[cfg(test)]
    pub fn is_user_valid(self, email: &str, password: &str) {
        assert_eq!(self.email, String::from(email));
        assert!(verify(password, &self.password).unwrap());
        assert!(self.id.to_string().len() == 36);
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
