// use actix_web::error::BlockingError;
use actix_web::HttpResponse;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::{DatabaseError, NotFound};
use serde_derive::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    OperationCanceled,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::RecordAlreadyExists => write!(f, "This recored violates a unique constraint"),
            ServiceError::RecordNotFound => write!(f, "This record does not exists"),
            ServiceError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            ServiceError::OperationCanceled => write!(f, "The running operation was canceled"),
        }
    }
}

impl From<diesel::result::Error> for ServiceError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            DatabaseError(UniqueViolation, _) => ServiceError::RecordAlreadyExists,
            NotFound => ServiceError::RecordNotFound,
            _ => ServiceError::DatabaseError(e),
        }
    }
}

// impl From<BlockingError> for AppError {
//     fn from(e: BlockingError) -> Self {
//         match e {
//             _ => AppError::OperationCanceled,
//         }
//     }
// }

#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
}

impl actix_web::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let err = format!("{}", self);
        let mut builder = match self {
            ServiceError::RecordAlreadyExists => HttpResponse::BadRequest(),
            ServiceError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse { err })
    }


}
