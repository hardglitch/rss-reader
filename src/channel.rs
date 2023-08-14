use image::EncodableLayout;
use sqlx::{FromRow, Row};
use std::error::Error;
use url::Url;
use crate::db;


#[derive(Debug, FromRow)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub image: Vec<u8>,
}

impl Channel {

    #[allow(dead_code)]
    pub async fn add_to_db(&self) {
        let pool = db::def_pool().await;
        let img = serde_json::to_string(self.image.as_slice()).unwrap();
        let query = format!(
            "INSERT INTO channels (title, link, image) VALUES ({:?}, {:?}, {:?});",
            self.title, self.link, img
        );

        sqlx::query(&query).execute(&pool).await.unwrap();
    }
}

#[allow(dead_code)]
pub async fn get_channel_from_db(id: usize) -> Result<Channel, Box<dyn Error>> {
    let query = format!("SELECT * FROM channels WHERE id='{id}';");
    let pool = db::def_pool().await;

    let result = sqlx::query(&query)
        .fetch_one(&pool)
        .await
        .expect("Row not found");

    let bytes: Vec<u8> = serde_json::from_str(result.get(3)).unwrap();
    Ok(Channel {
        title: result.get(1),
        link: result.get(2),
        image: bytes,
    })
}

pub async fn get_channels_from_db() -> Result<Vec<Channel>, Box<dyn Error>> {
    let query = format!("SELECT * FROM channels;");
    let pool = db::def_pool().await;

    let result = sqlx::query(&query)
        .fetch_all(&pool)
        .await
        .expect("Row not found");

    let mut chs = Vec::new();
    for row in result {
        let bytes: Vec<u8> = serde_json::from_str(row.get(3)).unwrap();
        chs.push(
            Channel {
                title: row.get(1),
                link: row.get(2),
                image: bytes,
            }
        );
    };
    Ok(chs)
}

#[allow(dead_code)]
pub async fn get_channel_by_url(url: String) -> Result<Channel, Box<dyn Error>> {
    let parsed_url = Url::parse(&url)?;

    let mut favicon_url = String::new();
    favicon_url.push_str(parsed_url.scheme());
    favicon_url.push_str("://");
    favicon_url.push_str(&parsed_url.host().unwrap().to_string());
    favicon_url.push_str("/favicon.ico");

    let content_ico = reqwest::get(&favicon_url)
        .await?
        .bytes()
        .await?
        .as_bytes()
        .to_owned();

    let content = reqwest::get(url).await?.bytes().await?;
    let channel = rss::Channel::read_from(&content[..])?;

    Ok(Channel {
        title: String::from(channel.title()),
        link: String::from(channel.link()),
        image: content_ico,
    })
}
