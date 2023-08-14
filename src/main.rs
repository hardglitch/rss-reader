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
    // let db = db::Database::default();
    // let pool = db.get_pool().await;

    // 1. Get channel from web
    // let new_channel = channel::get_channel_by_url("https://www.softexia.com/feed".to_owned()).await?;
    // new_channel.add_to_db(&pool).await;

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
