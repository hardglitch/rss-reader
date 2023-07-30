use std::error::Error;
use rss;
use sqlx::{Pool, Sqlite};


#[derive(Debug)]
pub struct Channel {
    pub ch_id: i64,
    pub title: String,
    pub link: String,
}

impl Channel {
    pub async fn add_channel_to_db(&self, db: &Pool<Sqlite>) {
        let query = format!("INSERT INTO channels (title, link) VALUES ({:?}, {:?});", self.title, self.link);
        sqlx::query(&query)
            .execute(db)
            .await
            .unwrap();
    }
}

pub async fn get_channel_by_url(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = rss::Channel::read_from(&content[..])?;
    Ok(channel_converter(channel).unwrap())
}

fn channel_converter(channel: rss::Channel) -> Result<Channel, Box<dyn Error>> {
    let conv_ch = Channel {
        ch_id: 0,
        title: String::from(channel.title()),
        link: String::from(channel.link()), 
    };
    Ok(conv_ch)
}
