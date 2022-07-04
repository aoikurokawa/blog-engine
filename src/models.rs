use crate::schema::posts;
use crate::schema::categories;
use diesel::prelude::*;
use diesel::result::Error;
use serde_derive::{Deserialize, Serialize};


// pub fn find_user(conn: &PgConnection, id: i32) -> Result<User, Error> {
//     users::table
//         .find(id)
//         .select((users::username, users::email))
//         .first::<User>(conn)
//         .map_err(Into::into)
// }

#[derive(Queryable, Associations, Serialize, Debug, Insertable)]
#[belongs_to(Category)]
pub struct Post {
    pub category_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Insertable)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

