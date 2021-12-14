use chrono::NaiveDateTime;
use thiserror::Error;
#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("mysql query error: {error:?}")]
    DieselError {
        error: diesel::result::Error,
    },

    #[error("time parse error: {error:?}")]
    TimeParseError {
        error: chrono::ParseError,
    },

    #[error("get conn failed from r2d2 pool: {error:?}")]
    GetConnectionFailed {
        error: r2d2::Error,
    },
}

impl From<diesel::result::Error> for QueryError {
    fn from(error: diesel::result::Error) -> Self {
        QueryError::DieselError { error }
    }
}

impl From<chrono::ParseError> for QueryError {
    fn from(error: chrono::ParseError) -> Self {
        QueryError::TimeParseError { error }
    }
}

impl From<r2d2::Error> for QueryError {
    fn from(error: r2d2::Error) -> Self {
        QueryError::GetConnectionFailed { error }
    }
}