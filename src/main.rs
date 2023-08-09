// #![feature(allocator_api)]
// #![feature(box_into_inner)]
// #![feature(fn_traits)]

// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod channel;
mod utils;
use std::error::Error;
use eframe::egui;
use egui_extras::RetainedImage;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    //0. Create connection to DB
    let db = db::init().await.unwrap();

    // 1. Get channel from web
    // let new_channel = channel::get_channel_by_url("https://samlab.ws/rss".to_owned()).await?;
    // new_channel.add_to_db(&db).await;

    // 2. Get channel from DB
    let ch = channel::get_channel_from_db("http://samlab.ws/", &db).await?;

    let img = image::io::Reader::new(std::io::Cursor::new(&ch.image)).with_guessed_format()?.decode()?;
    let img_buff = img.into_rgba8().as_raw().as_slice();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "rss-reader", 
        options, 
        Box::new(|_cc| Box::<App>::default()),
    )?;

    struct App {
        image: RetainedImage
    }


    impl Default for App {

        fn default() -> Self {
            Self {
                image: RetainedImage::from_image_bytes(
                    "favicon.ico",
                    image::load_from_memory_with_format(img_buff, image::ImageFormat::Jpeg)?,
                )
                .unwrap()
            }
        }
    }

    impl eframe::App for App {

        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("This is an image:");
                self.image.show(ui);
            });
        }
    }

    Ok(())

}