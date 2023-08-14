use eframe::epaint::Vec2;
use tokio::runtime;
use eframe::CreationContext;
use egui_extras::RetainedImage;
use eframe::egui;
use crate::channel;
use crate::db;


pub struct App {
    rt: runtime::Runtime,
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

        Self { rt }
    }
}


impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let chs = self.rt.block_on(async move {
            let pool = db::def_pool().await;
            channel::get_channels_from_db(&pool).await.unwrap()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            for ch in chs {
                let img = RetainedImage::from_image_bytes(
                    "favicon.ico",
                    ch.image.as_slice(),
                )
                .unwrap();
                ui.vertical(|ui| {
                    img
                        .show_max_size(ui, Vec2{x:16.0, y:16.0})
                        .on_hover_text(&ch.title);
                });
            }
        });
    }
}
