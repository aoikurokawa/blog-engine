pub mod comments;
pub mod posts;
pub mod users;

pub use comments::*;
pub use posts::*;
pub use users::*;

use crate::errors::AppError;

use actix_web::HttpResponse;

fn convert<T, E>(res: Result<T, E>) -> Result<HttpResponse, AppError>
where
    T: serde::Serialize,
    AppError: From<E>,
{
    res.map(|d| HttpResponse::Ok().json(d)).map_err(Into::into)
}
