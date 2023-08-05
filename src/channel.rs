use std::error::Error;
use image::EncodableLayout;
use rss;
use sqlx::{SqlitePool, FromRow, Row};
use url::Url;


#[derive(Debug, FromRow)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub image: Vec<u8>,
}

impl Channel {

    pub async fn add_to_db(&self, db: &SqlitePool) {
        let img = serde_json::to_string(self.image.as_slice()).unwrap();
        let query = format!("INSERT INTO channels (title, link, image) VALUES ({:?}, {:?}, {:?});", self.title, self.link, img);
        
        sqlx::query(&query)
            .execute(db)
            .await
            .unwrap();
    }
}

pub async fn get_channel_from_db(link: &str, db: &SqlitePool) -> Result<Channel, Box<dyn Error>> {
    let query = format!("SELECT * FROM channels WHERE link='{link}';");

    let result = sqlx::query(&query)
        .fetch_one(db)
        .await
        .expect("Row not found");

    let bytes: Vec<u8> = serde_json::from_str(result.get(3)).unwrap();
    Ok(
        Channel {
            title: result.get(1),
            link: result.get(2),
            image: bytes
        }
    )
}


pub async fn get_channel_by_url(url: String) -> Result<Channel, Box<dyn Error>> {
    let parsed_url = Url::parse(&url)?;

    let mut favicon_url = "".to_owned();
    favicon_url.push_str(&parsed_url.scheme());
    favicon_url.push_str("://");
    favicon_url.push_str(&parsed_url.host().unwrap().to_string());
    favicon_url.push_str("/favicon.ico");

    let content_ico = reqwest::get(&favicon_url).await?.bytes().await?.as_bytes().to_owned();

    let content = reqwest::get(url).await?.bytes().await?;
    let channel = rss::Channel::read_from(&content[..])?;

    Ok ( 
        Channel {
            title: String::from(channel.title()),
            link: String::from(channel.link()),
            image: content_ico
        }
     )
}
