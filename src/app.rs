use eframe::{epaint::Vec2, CreationContext, egui};
use egui_extras::RetainedImage;
use tokio::runtime;
use crate::{channel, db};
// use std::sync::Arc;


pub struct App {
    rt: runtime::Runtime,
}

impl App {

    pub fn new(ctx: &CreationContext) -> Self {
        ctx.egui_ctx.set_pixels_per_point(1.0);

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.egui_ctx.set_fonts(fonts);

        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async move {
            db::init().await;
        });

        // let chs = Arc::new(tokio::sync::Mutex::new(Vec::<channel::Channel>::new()));
        // rt.spawn(async move {
        //     let pool = db::def_pool().await;
        //     // let mut chs_c = chs.lock().await;
        //     chs = channel::get_all_channels(&pool).await.unwrap();
        // });

        Self { rt }
    }
}


impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let chs = self.rt.block_on(async move {
            let pool = db::def_pool().await;
            channel::get_all_channels(&pool).await.unwrap()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let main_height = Box::new(ui.available_height());
            let main_width = Box::new(ui.available_width());
            const AREA_1_WIDTH: f32 = 16.0;
            const ICON_SIZE: f32 = 16.0;

            ui.horizontal(|ui| {

                // Area 1
                ui.vertical(|ui| {
                    ui.set_width(AREA_1_WIDTH);
                    ui.set_height(*main_height);


                    let area_1_1 = |ui: &mut egui::Ui| {
                        let list_text = egui::RichText::new(egui_phosphor::regular::LIST).size(16.0);
                        let list_btn = egui::Label::new(list_text);
                        if ui.add(list_btn.sense(egui::Sense::click())).clicked() {
                            println!("Clicked 1");
                        };
        
                        let star_text = egui::RichText::new(egui_phosphor::regular::STAR).size(16.0);
                        let star_btn = egui::Label::new(star_text);
                        if ui.add(star_btn.sense(egui::Sense::click())).clicked() {
                            println!("Clicked 2");
                        };
        
                        let plus_text = egui::RichText::new(egui_phosphor::regular::PLUS).size(16.0);
                        let plus_btn = egui::Label::new(plus_text);
                        if ui.add(plus_btn.sense(egui::Sense::click())).clicked() {
                            println!("Clicked 3");
                        };
                    };
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), area_1_1);


                    let area_1_2 = |ui: &mut egui::Ui| {

                        for ch in chs {
                    
                            let ch_icon = |ui| {

                                let img = RetainedImage::from_image_bytes(
                                    "favicon.ico",
                                    ch.image.as_slice(),
                                )
                                .unwrap();
        
                                if img
                                    .show_max_size(ui, Vec2{x:ICON_SIZE, y:ICON_SIZE})
                                    .on_hover_text(&ch.title)
                                    .interact(egui::Sense::click())
                                    .clicked() {
                                        println!("{:?}", &ch.title);
                                    }
                            };
                            ch_icon(ui);
                        }
                    };
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), area_1_2);
    
                });

                
                // Areas 2 and 3
                ui.vertical(|ui| {

                    egui::ScrollArea::vertical()
                        .id_source("Area_2")
                        .max_height(200.)
                        .show(ui, |ui| {

                            ui.set_min_height(200.);
                            ui.set_min_width(*main_width - AREA_1_WIDTH);

                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                            ui.label("Area 2");
                    });

                    ui.add_space(10.);

                    egui::ScrollArea::vertical()
                        .id_source("Area_3")
                        .max_height(100.)
                        .show(ui, |ui| {

                            ui.set_min_height(200.);
                            ui.set_min_width(*main_width - AREA_1_WIDTH);

                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                            ui.label("Area 3");
                    });

                });

            });
        });
    }
}
