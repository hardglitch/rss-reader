use std::error::Error;
use rss::Channel;


#[allow(dead_code)]
pub async fn example_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://samlab.ws/rss/")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

