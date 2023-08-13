// #![feature(allocator_api)]
// #![feature(box_into_inner)]
// #![feature(fn_traits)]
// #![feature(async_fn_in_trait)]

// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod channel;
mod db;
mod app;
use eframe::egui;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    //0. Create connection to DB
    // let db = db::init().await.unwrap();

    // 1. Get channel from web
    // let new_channel = channel::get_channel_by_url("https://samlab.ws/rss".to_owned()).await?;
    // new_channel.add_to_db(&db).await;

    // 2. Get channel from DB
    // let ch = channel::get_channel_from_db("http://samlab.ws/", &db).await?;

    // let img = image::io::Reader::new(std::io::Cursor::new(&ch.image))
    //     .with_guessed_format()?
    //     .decode()?;
    // let img_buff = img.into_rgba8().as_raw().as_slice();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "rss-reader", 
        options, 
        Box::new(|ctx| Box::new(app::App::new(ctx)))
    )?;

    Ok(())
}
