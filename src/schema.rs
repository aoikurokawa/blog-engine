// #[macro_use]
// extern crate juniper;

use juniper::RootNode;

// #[derive(GraphQLObject)]
// #[graphql(description="Todo struct")]
struct Todo {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}

#[juniper::graphql_object(description = "A todo")]
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

// dummy data
#[juniper::graphql_object]
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
            },
        ]
    }
}

pub struct MutationRoot;

#[derive(juniper::GraphQLInputObject)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[juniper::graphql_object]
impl MutationRoot {
    fn create_todo(new_todo: NewTodo) -> Todo {
        Todo {
            id: 1,
            title: new_todo.title,
            description: new_todo.description,
            completed: new_todo.completed,
        }
    }
}

pub struct SubscriptionRoot;

#[juniper::graphql_object]
impl SubscriptionRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn create_schema() -> Schema {
    return Schema::new(QueryRoot, MutationRoot, SubscriptionRoot);
}
