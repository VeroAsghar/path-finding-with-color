use anyhow::Result;
use color_tracking::{filter_color, Processor};
use eframe::egui;
use eframe::epaint::{ColorImage, ImageData, TextureHandle};
use egui_extras::RetainedImage;
use image::imageops::resize;
use image::imageops::FilterType::Gaussian;
use image::{DynamicImage, ImageBuffer, Rgb};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 1000.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_| Box::new(App::default())),
    )
}

struct App {
    texture: Option<TextureHandle>,
    image: DynamicImage,
    processor: Processor,
    w: u32,
    h: u32,
}

impl Default for App {
    fn default() -> Self {
        let frame = image::io::Reader::open("calibration/vehicle2.png")
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let (w, h) = (640, 360);
        let frame = resize(&frame, w, h, Gaussian);
        let frame = DynamicImage::ImageRgba8(frame).to_rgb8();
        let proc = Processor {
            blur: 1.0,
            rgb1: [120, 120, 0],
            rgb2: [255, 255, 130],
        };

        Self {
            image: DynamicImage::ImageRgb8(frame),
            processor: proc,
            w,
            h,
            texture: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let lower = ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.processor.rgb1[0], 0..=255).text("r"));
                ui.add(egui::Slider::new(&mut self.processor.rgb1[1], 0..=255).text("g"));
                ui.add(egui::Slider::new(&mut self.processor.rgb1[2], 0..=255).text("b"));
            });
            let upper = ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.processor.rgb2[0], 0..=255).text("R"));
                ui.add(egui::Slider::new(&mut self.processor.rgb2[1], 0..=255).text("G"));
                ui.add(egui::Slider::new(&mut self.processor.rgb2[2], 0..=255).text("B"));
            });

            if ui.button("update").clicked() {
                let mut xs = Vec::new();
                let mut ys = Vec::new();
                filter_color(
                    &self.image.as_rgb8().unwrap(),
                    &mut xs,
                    &mut ys,
                    self.processor.rgb1,
                    self.processor.rgb2,
                );
                let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(self.w, self.h);
                for (x, y) in xs.into_iter().zip(&ys) {
                    output.put_pixel(x, *y, Rgb([255u8, 255, 255]));
                }
                self.texture = Some(ui.ctx().load_texture(
                    "image",
                    ColorImage::from_rgb(
                        [self.w as usize, self.h as usize],
                        output.as_flat_samples().as_slice(),
                    ),
                    Default::default(),
                ));
            }

            // Show the image:
            if let Some(texture) = self.texture.as_ref() {
                ui.image(texture, texture.size_vec2());
            } else {
                ui.spinner();
            }
        });
    }
}
