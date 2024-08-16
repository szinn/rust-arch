use uuid::Uuid;

#[derive(Debug)]
pub struct Item {
    pub id: i64,
    pub version: i32,
    pub uuid: Uuid,
    pub text: String,
}
