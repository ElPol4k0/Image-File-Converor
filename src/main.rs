use eframe::egui;
use eframe::epi;
use image::{DynamicImage, ImageOutputFormat};
use std::fs::File;

struct MyApp {
    input_path: String,
    output_path: String,
    new_width: u32,
    new_height: u32,
    format: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_path: String::new(),
            output_path: String::new(),
            new_width: 800,
            new_height: 600,
            format: "jpeg".to_string(),
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Image Processor"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image Processor");

            ui.horizontal(|ui| {
                ui.label("Input Path:");
                ui.text_edit_singleline(&mut self.input_path);
                if ui.button("Browse...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.input_path = path.display().to_string();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Output Path:");
                ui.text_edit_singleline(&mut self.output_path);
                if ui.button("Browse...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        self.output_path = path.display().to_string();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(egui::Slider::new(&mut self.new_width, 1..=4000));
            });

            ui.horizontal(|ui| {
                ui.label("Height:");
                ui.add(egui::Slider::new(&mut self.new_height, 1..=4000));
            });

            ui.horizontal(|ui| {
                ui.label("Format:");
                egui::ComboBox::from_label("")
                    .selected_text(&self.format)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.format, "png".to_string(), "PNG");
                        ui.selectable_value(&mut self.format, "jpeg".to_string(), "JPEG");
                        ui.selectable_value(&mut self.format, "bmp".to_string(), "BMP");
                        ui.selectable_value(&mut self.format, "ico".to_string(), "ICO");
                        ui.selectable_value(&mut self.format, "webp".to_string(), "WebP");
                    });
            });

            if ui.button("Process Image").clicked() {
                process_image(
                    &self.input_path,
                    &self.output_path,
                    self.new_width,
                    self.new_height,
                    &self.format,
                );
            }
        });
    }
}

fn main() {
    let app = MyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

fn process_image(
    input_path: &str,
    output_path: &str,
    new_width: u32,
    new_height: u32,
    format: &str,
) {
    if let Ok(image) = image::open(input_path) {
        let resized_image = image.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);
        save_image(&resized_image, output_path, format);
    } else {
        eprintln!("Failed to open image at path: {}", input_path);
    }
}

fn save_image(image: &DynamicImage, output_path: &str, format: &str) {
    let output_format = match format {
        "png" => ImageOutputFormat::Png,
        "jpeg" => ImageOutputFormat::Jpeg(80),
        "bmp" => ImageOutputFormat::Bmp,
        "ico" => ImageOutputFormat::Ico,
        
        _ => ImageOutputFormat::Jpeg(80), // Default to JPEG if format is unknown
    };

    //TODO: Ladebalken hinzufügen
    //TODO: Fehlerbehandlung verbessern
    //TODO: Dateiüberschreibung verhindern
    //TODO: UI/UX verbessern

    // Überprüfen, ob der Pfad bereits die korrekte Dateierweiterung hat, andernfalls hinzufügen
    let path = if output_path.ends_with(format) {
        output_path.to_string()
    } else {
        format!("{}.{}", output_path, format)
    };

    // Tatsächliche Verwendung des 'path' beim Erstellen der Datei
    let mut output_file = match File::create(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create output file: {}", e);
            return;
        }
    };

    if let Err(e) = image.write_to(&mut output_file, output_format) {
        eprintln!("Failed to save image: {}", e);
    }
}

