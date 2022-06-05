use crate::errors::AppError;
use crate::schema::comments;
use crate::schema::posts;
use crate::schema::users;
use diesel::prelude::*;
use diesel::result::Error;
use serde_derive::{Deserialize, Serialize};

// type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Serialize, Debug, PartialEq, Deserialize, Insertable)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub email: String,
}

pub fn find_user(conn: &PgConnection, id: i32) -> Result<User, Error> {
    users::table
        .find(id)
        .select((users::username, users::email))
        .first::<User>(conn)
        .map_err(Into::into)
}

#[derive(Queryable, Associations, Serialize, Debug, Insertable)]
#[belongs_to(User)]
pub struct Post {
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Associations, Serialize, Debug)]
#[belongs_to(User)]
#[belongs_to(Post)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub body: String,
}
