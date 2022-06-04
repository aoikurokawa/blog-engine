use juniper;
use juniper::FieldResult;

#[derive(GraphQLEnum, Clone, Debug)]
enum Tag {
    Blockchain,
    DeFi, 
    DAO,
}

#[derive(Debug, Clone, juniper::GraphQLObject)]
struct Blog {
    id: i32,
    title: String, 
    date: u32,
    tags: Vec<Tag>,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
#[graphql(name = "NewBlog", description = "New Blog!")]
pub struct NewBlog {
    pub title: String, 
    pub tags: Vec<Tag>,
}

#[juniper::graphql_object(Context = Context)]
impl Blog {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn date(&self) -> u32 {
        self.date
    }
}
