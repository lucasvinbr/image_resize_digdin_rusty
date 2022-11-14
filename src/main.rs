#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, DroppedFile};
use image::GenericImageView;

#[derive(Default)]
struct MyApp {
    actions_log: Vec<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for dropped_file in &ui.input().raw.dropped_files {
                self.actions_log.push(process_file(dropped_file));
            }
            ui.heading("Drag images into this window to resize them to multiples of 4");

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    for log_entry in &self.actions_log {
                        ui.label(log_entry);
                    }
                });
            })
        });
    }
}

fn process_file(dropped_file: &DroppedFile) -> String {
    let file_path = String::from(
        dropped_file
            .path
            .to_owned()
            .unwrap()
            .to_str()
            .unwrap_or("invalid path"),
    );
    if file_path == "invalid path" {
        format!("Could not parse path: {}", file_path)
    } else {
        let img_process_attempt = image::open(&file_path);

        if let Ok(img) = img_process_attempt {
            // The dimensions method returns the images width and height.
            let (img_x, img_y) = img.dimensions();
            println!("dimensions {:?}", img.dimensions());

            let (adjusted_x, adjusted_y) = (closest_multiple_4(img_x), closest_multiple_4(img_y));

            if img_x != adjusted_x || img_y != adjusted_y {
                let resized_img = img.resize_exact(
                    adjusted_x,
                    adjusted_y,
                    image::imageops::FilterType::Gaussian,
                );
                let img_save_attempt =
                    resized_img.save_with_format(&file_path, image::ImageFormat::Png);

                if let Err(e) = img_save_attempt {
                    format!("{}: failed to save resized image - {}", &file_path, e)
                } else {
                    format!("{}: resized successfully", &file_path)
                }
            } else {
                format!("{}: no resizing needed (already multiple of 4)", file_path)
            }
        } else {
            let err = img_process_attempt.unwrap_err();

            format!("{}: processing failed - {}", file_path, err)
        }
    }
}

fn closest_multiple_4(num: u32) -> u32 {
    let adjusted_num = num - (num % 4);

    if adjusted_num > 0 {
        adjusted_num
    }else {
        4
    }
}

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "Image Resize Digdin Rusty version",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
