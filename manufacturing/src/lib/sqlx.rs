use sqlx::{error::DatabaseError, SqlitePool};

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";

pub trait SqliteConnection: Send + Sync + 'static {
    fn pool(&self) -> &SqlitePool;
}

pub enum Error {
    Sqlite { inner: SqliteError },
    RowNotFound,
    Unknown,
}

impl From<&sqlx::Error> for Error {
    fn from(value: &sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(db_err) => Self::Sqlite {
                inner: db_err.into(),
            },
            sqlx::Error::RowNotFound => Self::RowNotFound,
            _ => Self::Unknown,
        }
    }
}

pub enum SqliteError {
    UniqueConstraintViolationCode,
    Unknown { message: String },
}

impl From<&Box<dyn DatabaseError>> for SqliteError {
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
