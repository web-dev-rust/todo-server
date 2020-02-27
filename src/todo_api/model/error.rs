use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum DbError {
    UserNotCreated,
    DatabaseConflit,
    CannotFindUser,
    TryAgain,
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::UserNotCreated => write!(f, "User could not be created"),
            DbError::DatabaseConflit => write!(f, "There are conflits in database"),
            DbError::CannotFindUser => write!(f, "User could not be found"),
            DbError::TryAgain => write!(f, "Expire date could not be updated"),
        }
    }
}

impl Error for DbError {
    fn description(&self) -> &str {
        match self {
            DbError::UserNotCreated => "User could not be created, check for possible conflits",
            DbError::DatabaseConflit => "There are conflits in database",
            DbError::CannotFindUser => "User could not be found",
            DbError::TryAgain => "Expire date could not be updated",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        Some(self)
    }
}
