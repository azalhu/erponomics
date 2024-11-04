pub mod sqlite;

pub enum Error {
    Sqlite { inner: sqlite::Error },
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
