use juniper::{EmptyMutation, RootNode};

#[derive(GraphQLEnum)]
enum Tag {
    Blockchain,
    DeFi, 
    DAO,
}

struct Blog {
    id: i32,
    title: String, 
    date: u32,
    tags: Vec<Tag>,
}

#[juniper::object(description = "A Blog")]
impl Blog {
    pub fn id(&self) -> i32 {
        self.id
    }
}


