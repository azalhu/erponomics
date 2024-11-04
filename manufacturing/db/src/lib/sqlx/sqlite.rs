use sqlx::{error::DatabaseError, SqlitePool};

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";

pub trait Connection: Send + Sync + 'static {
    fn pool(&self) -> &SqlitePool;
}

pub enum Error {
    UniqueConstraintViolationCode,
    Unknown { message: String },
}

impl From<&Box<dyn DatabaseError>> for Error {
    fn from(value: &Box<dyn DatabaseError>) -> Self {
        if let Some(code) = value.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION_CODE {
                return Self::UniqueConstraintViolationCode;
            }
        }

        Self::Unknown {
            message: value.message().to_string(),
        }
    }
}
