mod db;
mod channel;


#[tokio::main]
async fn main() {

    let db = db::init().await.unwrap();
    let new_channel = channel::get_channel_by_url("https://samlab.ws/rss/").await.unwrap();
    new_channel.add_channel_to_db(&db).await;

    // HelloWorld::new().unwrap().run().unwrap();
}