// #![feature(allocator_api)]
// #![feature(box_into_inner)]

// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod channel;
mod utils;
use std::error::Error;
use eframe::egui;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // let db = db::init().await.unwrap();

    // 1. Get channel from web
    // let new_channel = channel::get_channel_by_url("https://samlab.ws/rss".to_owned()).await?;
    // new_channel.add_to_db(&db).await;

    // 2. Get channel from DB
    // let ch = channel::get_channel_from_db("http://samlab.ws/", &db).await?;
    // let img = image::io::Reader::new(std::io::Cursor::new(&ch.image)).with_guessed_format()?.decode()?;
    // let image = img.into_rgba8();

    eframe::run_native("rss-reader", eframe::NativeOptions::default(), Box::new(|cc| Box::new(App::new(cc))))?;

    struct App {}

    impl App {  
        fn new(_cc: &eframe::CreationContext<'_>) -> Self {
            App {}
        }
    }

    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(r#"Label"#);
            });
        }
    }

    Ok(())

}