use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NewItem {
    pub text: String,
}

#[derive(Debug)]
pub struct Item {
    pub id: i64,
    pub version: i32,
    pub uuid: Uuid,
    pub text: String,
}
