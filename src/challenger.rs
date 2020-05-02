use uuid::Uuid;

pub enum Role {
    Administrator,
    User,
}

pub struct Challenger {
    id: Uuid,
    username: String,
    role: Role,
    answer: Vec<Uuid>,
}