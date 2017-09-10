use uuid::Uuid;
pub struct Entity {
    components: Vec<Uuid>,
}

impl Entity {
    pub fn new(components: Vec<Uuid>) -> Self {
        Entity {
            components,
        }
    }
}
