use diesel_async::{
    pooled_connection::{
        deadpool::Pool, AsyncDieselConnectionManager, PoolError,
    },
    AsyncPgConnection,
};

use rocket::{http::Status, serde::json::Json};
use diesel::result::Error as DieselError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    status: Status,
    description: String
}

impl ErrorResponse {
    pub fn new<T>(status: Status, description: T) -> Self 
    where T: Into<String> {
        Self { status, description: description.into() }
    }

    pub fn internal_error<T>(error: T) -> Self
    where T: std::error::Error + std::fmt::Display {
        Self::new(Status::InternalServerError, format!("{error}"))
    }
}

impl From<DieselError> for ErrorResponse {
    fn from(value: DieselError) -> Self {
        match value {
            DieselError::NotFound => ErrorResponse::new(Status::NotFound, Status::NotFound.reason_lossy().to_string()),
            err => ErrorResponse::new(Status::InternalServerError, format!("{err}"))
        }
        
    }
}

impl From<PoolError> for ErrorResponse {
    fn from(value: PoolError) -> Self {
        Self::internal_error(value)
    }
}

pub type Db = AsyncDieselConnectionManager<AsyncPgConnection>;
pub type DbPool = Pool<AsyncPgConnection>;
pub type JsonResponse<T> = Result<Json<T>, Json<ErrorResponse>>;
