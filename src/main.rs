use eframe::egui;
use libloading::{Library, Symbol};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::path::Path;

mod plugin;
mod telemetry;
mod storage;

use plugin::Plugin;
use telemetry::TelemetryData;
use storage::Storage;

pub struct ConsoleApp {
    plugins: Vec<Box<dyn Plugin>>,
    telemetry_data: Arc<Mutex<TelemetryData>>,
    storage: Storage,
    screenshot: Option<egui::TextureHandle>,
}

impl ConsoleApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let telemetry_data = Arc::new(Mutex::new(TelemetryData::default()));
        let storage = Storage::new("data.json");
        let mut app = ConsoleApp {
            plugins: Vec::new(),
            telemetry_data,
            storage,
            screenshot: None,
        };
        app.load_plugins();
        app.load_screenshot(cc);
        app
    }

    fn load_plugins(&mut self) {
        let plugins_dir = Path::new("plugins");
        if !plugins_dir.exists() {
            std::fs::create_dir_all(plugins_dir).expect("Failed to create plugins directory");
            return;
        }
        for entry in std::fs::read_dir(plugins_dir).expect("Failed to read plugins directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "dll" || ext == "so") {
                unsafe {
                    let lib = Library::new(&path).expect("Failed to load plugin");
                    let init: Symbol<unsafe extern "C" fn() -> Box<dyn Plugin>> = lib.get(b"init_plugin").expect("Failed to get init_plugin symbol");
                    let plugin = init();
                    self.plugins.push(plugin);
                }
            }
        }
    }

    fn load_screenshot(&mut self, cc: &eframe::CreationContext<'_>) {
        let image = image::open("design/ui.jpg").expect("Failed to load screenshot");
        let image_data = image.to_rgba8();
        let size = [image_data.width() as _, image_data.height() as _];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image_data);
        self.screenshot = Some(cc.egui_ctx.load_texture("screenshot", color_image, egui::TextureFilter::Linear));
    }
}

impl epi::App for ConsoleApp {
    fn name(&self) -> &str {
        "Console"
    }

    fn update(&mut self, ctx: &egui::Context) {
        // Simulate telemetry update
        {
            let mut data = self.telemetry_data.lock().unwrap();
            data.speed += 0.1;
            data.rpm += 10.0;
            if data.rpm > 8000.0 {
                data.rpm = 1000.0;
            }
        }

        // Update plugins
        let data = self.telemetry_data.lock().unwrap();
        for plugin in &mut self.plugins {
            plugin.update(&data);
        }

        // Render UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Console Dashboard");
            if let Some(texture) = &self.screenshot {
                ui.image(texture, texture.size_vec2());
            }
            for plugin in &self.plugins {
                plugin.render(ui);
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Console",
        options,
        Box::new(|cc| Box::new(ConsoleApp::new(cc))),
    );
}
