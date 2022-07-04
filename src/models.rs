use crate::schema::posts;
use crate::schema::categories;
use serde_derive::{Deserialize, Serialize};

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
    pub name: String,
}

