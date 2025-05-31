use actix_web::{HttpResponse, ResponseError};
use rusqlite::Error as SqliteError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(SqliteError),
    NotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::NotFound => write!(f, "Resource not found"),
        }
    }
}

impl From<SqliteError> for AppError {
    fn from(error: SqliteError) -> Self {
        AppError::DatabaseError(error)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(_) => HttpResponse::InternalServerError().json("Internal server error"),
            AppError::NotFound => HttpResponse::NotFound().json("Not found"),
        }
    }
}