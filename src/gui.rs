use crate::arnold::{extract_watermark_inner, xor_images};
use anyhow::{ensure, Result};
use eframe::Frame;
use egui::epaint::TextureManager;
use egui::mutex::RwLock;
use egui::{ColorImage, Context, ImageData, ScrollArea, TextureId, TextureOptions, Vec2, Visuals};
use image::io::Reader;
use image::{ImageBuffer, RgbImage};
use std::path::PathBuf;
use std::sync::Arc;

pub struct CatApp {
    xorred: RgbImage,
    extracted: RgbImage,
    size: [usize; 2],
    shown_image: Option<TextureId>,
    offset_x: i32,
    offset_y: i32,
    texture_options: TextureOptions,
}

impl CatApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        orig_path: PathBuf,
        watermark_path: PathBuf,
    ) -> Result<Self> {
        let orig = Reader::open(orig_path)?.decode()?.to_rgb8();
        let watermarked = Reader::open(watermark_path)?.decode()?.to_rgb8();

        ensure!(
            orig.height() == watermarked.height(),
            "Image height must be the same"
        );
        ensure!(
            orig.width() == watermarked.width(),
            "Image height must be the same"
        );

        let xorred = xor_images(&orig, &watermarked);
        let (w, h) = (xorred.width(), xorred.height());
        let extracted = ImageBuffer::new(w, h);

        // dark mode :)
        cc.egui_ctx.set_visuals(Visuals::dark());

        Ok(Self {
            xorred,
            extracted,
            size: [w as usize, h as usize],
            shown_image: None,
            offset_x: 1,
            offset_y: 1,
            texture_options: TextureOptions::LINEAR,
        })
    }

    fn extract_watermark(&mut self, tex_manager: Arc<RwLock<TextureManager>>) {
        // get a writable handle to the texture manager
        let mut texman = tex_manager.write();

        // free the previous texture
        if let Some(id) = self.shown_image {
            texman.free(id);
        }

        extract_watermark_inner(
            &self.xorred,
            &mut self.extracted,
            self.offset_x,
            self.offset_y,
        );

        let image = ColorImage::from_rgb(self.size, self.extracted.as_raw());
        self.shown_image = Some(texman.alloc(
            String::from("current_image"),
            ImageData::Color(image),
            self.texture_options,
        ));
    }
}

impl eframe::App for CatApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(id) = self.shown_image {
                ScrollArea::both().show(ui, |ui| {
                    ui.image(id, Vec2::new(self.size[0] as f32, self.size[1] as f32));
                });
            }
        });
        egui::Window::new("parameters").show(ctx, |ui| {
            let mut changed = false;
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    let ox_drag = egui::DragValue::new(&mut self.offset_x)
                        .clamp_range(0..=self.xorred.height() as i32)
                        .prefix("offset_x: ");
                    changed |= ui.add(ox_drag).changed();
                    if ui.button("+").clicked() {
                        self.offset_x += 1;
                        changed = true;
                    }
                    if ui.button("-").clicked() {
                        self.offset_x -= 1;
                        changed = true;
                    }
                });
                ui.horizontal(|ui| {
                    let oy_drag = egui::DragValue::new(&mut self.offset_y)
                        .clamp_range(0..=self.xorred.height() as i32)
                        .prefix("offset_y: ");
                    changed |= ui.add(oy_drag).changed();
                    if ui.button("+").clicked() {
                        self.offset_y += 1;
                        changed = true;
                    }
                    if ui.button("-").clicked() {
                        self.offset_y -= 1;
                        changed = true;
                    }
                });
                ui.horizontal(|ui| {
                    changed |= ui
                        .radio_value(&mut self.texture_options, TextureOptions::LINEAR, "linear")
                        .changed();
                    changed |= ui
                        .radio_value(
                            &mut self.texture_options,
                            TextureOptions::NEAREST,
                            "nearest",
                        )
                        .changed();
                });
            });

            if changed {
                self.extract_watermark(ctx.tex_manager())
            }
        });
    }
}
