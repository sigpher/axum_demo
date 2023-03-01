use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub dept: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub dept: String,
    pub password: String,
}
