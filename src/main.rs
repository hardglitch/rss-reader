mod db;
mod core;

#[tokio::main]
async fn main() {

    db::get_or_create_db().await;
    db::create_tables().await;

    // let res = example_feed().await.unwrap();
    // let items = res.items();
    // println!("{:?}", &items[0].title);
    // println!("{:?}", &items[0].description);

    // HelloWorld::new().unwrap().run().unwrap();
}