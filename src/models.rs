use crate::schema::categories;
use crate::schema::posts;
use diesel::Insertable;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Associations, Serialize, Debug, Insertable)]
#[belongs_to(Category)]
pub struct Post {
    pub category_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}


#[derive(Queryable, Serialize, Deserialize, Debug, Insertable)]
#[table_name = "categories"]
pub struct Category {
    pub name: String,
}

// impl Serialize for chrono::NaiveDateTime {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         serializer.serialize
//     }
// }
