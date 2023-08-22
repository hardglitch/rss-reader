use image::EncodableLayout;
use sqlx::{FromRow, Row, Pool, Sqlite};
use std::error::Error;
use url::Url;
// use rss;


#[derive(Debug, FromRow, Default)]
pub struct Channel {
    pub ch_id: u16,
    pub title: String,
    pub link: String,
    pub image: Vec<u8>,
}

impl Channel {

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub async fn add(&self, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error>> {
        let img = serde_json::to_string(self.image.as_slice())?;
        let query = format!(
            "INSERT INTO channels (title, link, image) VALUES ({:?}, {:?}, {:?});",
            self.title, self.link, img
        );

        sqlx::query(&query).execute(pool).await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get(&mut self, ch_id: u16, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error>> {
        let query = format!("SELECT * FROM channels WHERE id='{ch_id}';");
    
        let result = sqlx::query(&query)
            .fetch_one(pool)
            .await
            .expect("Row not found");
    
        self.ch_id = result.get::<u16, _>("ch_id");
        self.title = result.get::<String, _>("title");
        self.link = result.get::<String, _>("link");
        self.image = serde_json::from_str(result.get::<&str, _>("image"))?;

        Ok(())
    }

    // #[allow(dead_code)]
    // pub async fn update(&self, pool: &Pool<Sqlite>) {
    
    // }

    // #[allow(dead_code)]
    // pub async fn delete(&self, pool: &Pool<Sqlite>) {

    // }


    #[allow(dead_code)]
    pub async fn get_by_url(&mut self, url: &str) -> Result<(), Box<dyn Error>> {
        let parsed_url = Url::parse(url)?;
    
        let mut favicon_url = String::new();
        favicon_url.push_str(parsed_url.scheme());
        favicon_url.push_str("://");
        favicon_url.push_str(&parsed_url.host().unwrap().to_string());
        favicon_url.push_str("/favicon.ico");
    
        self.image = reqwest::get(&favicon_url)
            .await?
            .bytes()
            .await?
            .as_bytes()
            .to_owned();
    
        let content = reqwest::get(url).await?.bytes().await?;
        let channel = rss::Channel::read_from(&content[..])?;
    
        self.title = channel.title().to_owned();
        self.link = channel.link().to_owned();
        
        Ok(())
    }
}


pub async fn get_all_channels(pool: &Pool<Sqlite>) -> Result<Vec<Channel>, Box<dyn Error>> {
    let query = format!("SELECT * FROM channels;");

    let result = sqlx::query(&query)
        .fetch_all(pool)
        .await
        .expect("Row not found");

    let mut chs = Vec::new();
    for row in result {
        chs.push(
            Channel {
                ch_id: row.get::<u16, _>("ch_id"),
                title: row.get::<String, _>("title"),
                link: row.get::<String, _>("link"),
                image: serde_json::from_str(row.get::<&str, _>("image"))?,
            }
        );
    };
    Ok(chs)
}
