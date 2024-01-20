use crate::prelude::*;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
}
