#[macro_use]
extern crate juniper;

use juniper::{RootNode, http::graphql};

#[derive(GraphQLObject)]
#[graphql(description="Todo struct")]
struct Todo {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}


#[juniper::object(description = "A todo")]
impl Todo {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn todos() -> Vec<Todo> {
        vec![
            Todo {
                id: 1,
                title: "Watching Backetball".to_string(),
                description: "Watching the NBA finals".to_string(),
               completed: false, 
            },
            Todo {
                id: 2,
                title: "Watching football".to_string(),
                description: "Watching the NFL finals".to_string(),
               completed: false, 
            }
        ]
    }
}



