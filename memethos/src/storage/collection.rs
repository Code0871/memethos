use uuid::Uuid;

// TODO: think through the structure and how to fit it into the architecture
pub struct Collection {
    pub guid: Uuid,
    pub name: String,
} 