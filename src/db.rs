use std::error::Error;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Pool };


const DB_URL: &str = "sqlite://sqlite.db";

pub(crate) async fn init() -> Result<Pool<Sqlite>, Box<dyn Error>> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    create_tables(&db).await;

    Ok(db)
}


async fn create_tables(db: &Pool<Sqlite>) {
    let mut query = "CREATE TABLE IF NOT EXISTS channels (
        ch_id INTEGER PRIMARY KEY NOT NULL,
        title VARCHAR(100) NOT NULL,
        link TEXT UNIQUE NOT NULL
    );";
    sqlx::query(query)
        .execute(db)
        .await
        .unwrap();

    query = "CREATE TABLE IF NOT EXISTS news (
        news_id INTEGER PRIMARY KEY NOT NULL,
        ch_id INTEGER NOT NULL,
        header VARCHAR(250),
        fulltext TEXT,
        date INTEGER,
        favorite BOOLEAN,
        FOREIGN KEY(ch_id) REFERENCES channels(ch_id)
    );";
    sqlx::query(query)
        .execute(db)
        .await
        .unwrap();
}
