use std::error::Error;

#[derive(Debug)]
pub enum DbError {
    UserNotCreated,
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::UserNotCreated => write!(f, "User could not be created"),
        }
    }
}

impl Error for DbError {
    fn description(&self) -> &str {
        match self {
            DbError::UserNotCreated => "User could not be created, check for possible conflits",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        Some(self)
    }
}
