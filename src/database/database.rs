use sqlx::{Pool, Sqlite, SqlitePool};

pub async fn conn_db(db_path: &str) -> Pool<Sqlite> {
    let pool = SqlitePool::connect(db_path).await.unwrap();
    pool
}

