use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};


pub struct Database {
    url: String,
}

impl Default for Database {
    fn default() -> Self {
        Self { url: "sqlite://sqlite.db".to_owned() }
    }
}

impl Database {
    async fn create(&self) -> Result<(), sqlx::Error> {
        if !Sqlite::database_exists(&self.url).await.unwrap_or(false) {
            Sqlite::create_database(&self.url).await?
        }
        Ok(())
    }

    async fn get_pool(&self) -> Pool<Sqlite> {
        match SqlitePool::connect(&self.url).await {
            Ok(pool) => pool,
            Err(e) => panic!("Could not establish DB connection: {}", e)
        }
    }

    async fn create_tables(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut query = "CREATE TABLE IF NOT EXISTS channels (
            ch_id INTEGER PRIMARY KEY NOT NULL,
            title VARCHAR(100) NOT NULL,
            link TEXT UNIQUE NOT NULL,
            image BLOB
        );";
        sqlx::query(query).execute(pool).await?;
    
        query = "CREATE TABLE IF NOT EXISTS news (
            news_id INTEGER PRIMARY KEY NOT NULL,
            ch_id INTEGER NOT NULL,
            header VARCHAR(250),
            fulltext TEXT,
            date INTEGER,
            favorite BOOLEAN,
            FOREIGN KEY(ch_id) REFERENCES channels(ch_id)
        );";
        sqlx::query(query).execute(pool).await?;

        Ok(())
    }
}


pub async fn def_pool() -> Pool<Sqlite> {
    let db = Database::default();
    db.get_pool().await
}


pub async fn init() {
    let db = Database::default();
    db.create().await.expect("Database not created");
    let pool = db.get_pool().await;
    db.create_tables(&pool).await.expect("Tables not created");
}
