#![feature(allocator_api)]
#![feature(box_into_inner)]

// use slint::ComponentHandle;
mod db;
mod channel;
mod ui;
mod utils;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let db = db::init().await.unwrap();

    // 1. Get channel from web
    let new_channel = channel::get_channel_by_url("https://samlab.ws/rss".to_owned()).await.unwrap();
    new_channel.add_to_db(&db).await;

    // 2. Get channel from DB
    let ch = channel::get_channel_from_db("http://samlab.ws/", &db).await?;
    println!("1 - {:?}", &ch.title);
    println!("2 - {:?}", &ch.link);
    std::fs::write("new", &ch.image)?;

    Ok(())
    // let app = ui::MainWindow::new().unwrap();
    // app.run().unwrap();
}