pub struct NewBlog {
    pub title: String,
    pub content: String,
}

impl NewBlog {
    pub fn is_valid(&self) -> bool {
        if self.content.is_empty() || self.title.is_empty() {
            false
        } else {
            true
        }
    }
}
