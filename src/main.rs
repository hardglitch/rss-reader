// #![feature(allocator_api)]
// #![feature(box_into_inner)]
// #![feature(fn_traits)]
// #![feature(async_fn_in_trait)]

// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod channel;
mod db;
mod app;
mod news;
use eframe::{egui, NativeOptions};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    // let pool = db::def_pool().await;
    // let new_channel = channel::get_channel_by_url("https://www.softexia.com/feed".to_owned()).await?;
    // new_channel.add(&pool).await;

    let options = NativeOptions {
        min_window_size: Some(egui::vec2(600.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "rss-reader", 
        options, 
        Box::new(|cc| {
            Box::new(app::App::new(cc))
        })
    )?;

    Ok(())
}
