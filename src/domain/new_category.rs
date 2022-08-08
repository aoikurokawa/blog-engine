pub struct NewCategory {
    pub name: String,
}

impl NewCategory {
    pub fn is_valid(&self) -> bool {
        if self.name.is_empty() {
            false
        } else {
            true
        }
    }
}
