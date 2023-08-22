use sqlx::{FromRow, Row, Pool, Sqlite};

#[derive(Default, Debug, FromRow)]
pub struct News {
    news_id: u32,
    ch_id: u16,
    header: String,
    fulltext: String,
    date: u32,
    favorite: bool,
}

impl News {

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    async fn add(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let query = format!(
            "INSERT INTO news (ch_id, header, fulltext, date, favorite) VALUES ({:?}, {:?}, {:?}, {:?}, {:?});",
            self.ch_id, self.header, self.fulltext, self.date, self.favorite
        );
        sqlx::query(&query).execute(pool).await.unwrap();
        Ok(())
    }

    #[allow(dead_code)]
    async fn get(&mut self, news_id: u32, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let query = format!("SELECT * FROM news WHERE news_id='{news_id}';");
    
        let result = sqlx::query(&query)
            .fetch_one(pool)
            .await
            .expect("Row not found");
    
        self.news_id = result.get::<u32, _>("news_id");
        self.ch_id = result.get::<u16, _>("ch_id");

        Ok(())
    }

    // #[allow(dead_code)]
    // async fn update(&self) {

    // }

    // #[allow(dead_code)]
    // async fn delete(&self) {

    // }
}

#[allow(dead_code)]
async fn get_all_news_from_channel(ch_id: u16, pool: &Pool<Sqlite>) -> Result<Vec<News>, sqlx::Error> {
    let query = format!("SELECT * FROM news WHERE ch_id='{ch_id}';");
    let result = sqlx::query(&query)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    let mut news = Vec::new();
    for row in result {
        news.push(
            News {
                news_id: row.get::<u32, _>("news_id"),
                ch_id: row.get::<u16, _>("ch_id"),
                header: row.get::<String, _>("header"),
                fulltext: row.get::<String, _>("fulltext"),
                date: row.get::<u32, _>("date"),
                favorite: row.get::<bool, _>("favorite"),
            }
        );
    };
    Ok(news)
}

#[allow(dead_code)]
async fn get_favorites(pool: &Pool<Sqlite>) -> Result<Vec<News>, sqlx::Error> {
    let query = format!("SELECT * FROM news WHERE favorite='true';");
    let result = sqlx::query(&query)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    let mut news = Vec::new();
    for row in result {
        news.push(
            News {
                news_id: row.get::<u32, _>("news_id"),
                ch_id: row.get::<u16, _>("ch_id"),
                header: row.get::<String, _>("header"),
                fulltext: row.get::<String, _>("fulltext"),
                date: row.get::<u32, _>("date"),
                favorite: row.get::<bool, _>("favorite"),
            }
        );
    };
    Ok(news)
}
