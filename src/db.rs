use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool };

const DB_URL: &str = "sqlite://sqlite.db";

pub(crate) async fn get_or_create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

pub(crate) async fn create_tables() {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let mut query = "CREATE TABLE IF NOT EXISTS channels (
        ch_id INTEGER PRIMARY KEY NOT NULL,
        name VARCHAR(100) NOT NULL,
        url TEXT UNIQUE NOT NULL
    );";
    sqlx::query(query)
        .execute(&db)
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
        .execute(&db)
        .await
        .unwrap();
}
