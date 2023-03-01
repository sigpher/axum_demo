use crate::database::conn_db;
use crate::model;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::error::Error;

pub async fn root() -> &'static str {
    "hello world"
}

pub async fn create_user(
    Json(payload): Json<model::CreateUser>,
) -> (StatusCode, Json<model::User>) {
    let user = model::User {
        id: 0,
        username: payload.username,
        email: payload.email,
        dept: payload.dept,
        password: payload.password,
    };
    let pool = conn_db("test.db").await;
    let id = insert_user(&pool, &user).await.unwrap();
    println!("{}", id);
    (StatusCode::CREATED, Json(user))
}

pub async fn insert_user(pool: &SqlitePool, user: &model::User) -> Result<i64, Box<dyn Error>> {
    let mut conn = pool.acquire().await?;
    let sql = r#"
    INSERT INTO users (username, email, dept, password) VALUES (?, ?, ?, ?);
    "#;
    let id = sqlx::query(sql)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.dept)
        .bind(&user.password)
        .execute(&mut conn)
        .await?
        .last_insert_rowid();
    Ok(id)
}

// pub struct User {
//     pub id: u32,
//     pub username: String,
//     pub email: String,
//     pub dept: String,
//     pub password: String,
// }

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u32,
    username: String,
    email: String,
    dept: String,
}
