use sqlx::Pool;
use tokio::runtime;
use eframe::CreationContext;
use egui_extras::RetainedImage;
use eframe::egui;
use crate::channel;
use crate::db;
use sqlx::Sqlite;


pub struct App {
    // image: RetainedImage
    rt: runtime::Runtime,
    // db: Pool<Sqlite>
}


// impl Default for App {
//     fn default() -> Self {
//         Self {
//             image: RetainedImage::from_image_bytes(
//                 "favicon.ico",
//                 image::load_from_memory_with_format(img_buff, image::ImageFormat::Png)?,
//             )
//             .unwrap()
//         }
//     }
// }


impl App {

    pub fn new(ctx: &CreationContext) -> Self {
        ctx.egui_ctx.set_pixels_per_point(1.25);

        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.spawn(async move {
            db::init().await;
        });

        Self { rt: rt }
    }

    async fn get_channel(&self, db: &Pool<Sqlite>) -> channel::Channel {
        channel::get_channel_from_db("http://samlab.ws/", db).await.unwrap()
    }

    async fn get_image_as_bytes(&self, ch: &channel::Channel) -> Vec<u8> {
        let img = image::io::Reader::new(
            std::io::Cursor::new(&ch.image))
            .with_guessed_format().unwrap()
            .decode().unwrap();
        let img_buff = img.into_rgba8();
        img_buff.as_raw().to_owned()
    }
}


impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

            let db = db::Database::default();
            let pool = self.rt.block_on(db.get_pool());
            let ch = self.rt.block_on(self.get_channel(&pool));

            egui::CentralPanel::default().show(ctx, |ui| {

            //     let db = db::Database::default();
            //     let ch = self.get_channel(&db.get_pool().await).await;
    
            // } );

            ui.heading(&ch.title);
            let img = RetainedImage::from_image_bytes(
                "favicon.ico",
                &ch.image.as_slice(),
            )
            .unwrap();
            img.show(ui);
        });
    }
}
