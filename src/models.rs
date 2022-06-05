use crate::errors::AppError;
use crate::schema::users;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

// type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Serialize, Debug, PartialEq, Deserialize, Insertable)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub email: String,
}

// pub enum UserKey<'a> {
//     Username(&'a str),
//     ID(i32),
// }

// pub fn find_user<'a>(conn: &PgConnection, key: UserKey<'a>) -> Result<User> {
//     match key {
//         UserKey::Username(name) => users::table
//             .filter(users::username.eq(name))
//             .select((users::id, users::username))
//             .first::<User>(conn)
//             .map_err(AppError::from),
//         UserKey::ID(id) => users::table
//             .find(id)
//             .select((users::id, users::username))
//             .first::<User>(conn)
//             .map_err(Into::into),
//     }
// }
