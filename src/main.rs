use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::telemetry::TelemetryData;
use crate::ui::{Dashboard, Section, SectionContent, PlayerContent};

mod plugin;
mod telemetry;
mod storage;
mod ui;

pub struct ConsoleApp {
    telemetry_data: Arc<Mutex<TelemetryData>>,
    dashboard: Option<Dashboard>,
    ui_error: Option<String>,
    font_loaded: bool,
}

impl ConsoleApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = ConsoleApp {
            telemetry_data: Arc::new(Mutex::new(TelemetryData::default())),
            dashboard: None,
            ui_error: None,
            font_loaded: false,
        };
        app.load_dashboard();
        // Custom font setup will be done in update()
        app
    }

    fn load_dashboard(&mut self) {
        match std::fs::read_to_string("ui_layout.xml") {
            Ok(xml) => match crate::ui::Dashboard::from_xml(&xml) {
                Ok(dashboard) => {
                    self.dashboard = Some(dashboard);
                    self.ui_error = None;
                }
                Err(e) => {
                    eprintln!("[UI XML ERROR] Failed to parse ui_layout.xml: {e:?}\nXML Content:\n{xml}");
                    self.dashboard = None;
                    self.ui_error = Some(format!("Failed to parse ui_layout.xml: {e}\nSee terminal for details."));
                }
            },
            Err(e) => {
                eprintln!("[UI XML ERROR] Failed to read ui_layout.xml: {e:?}");
                self.dashboard = None;
                self.ui_error = Some(format!("Failed to read ui_layout.xml: {e}"));
            }
        }
    }

    fn custom_color_for_section(id: &str) -> egui::Color32 {
        match id {
            "messages" => egui::Color32::from_rgb(40, 40, 80),
            "carCondition" => egui::Color32::from_rgb(60, 40, 40),
            "timeCondition" => egui::Color32::from_rgb(40, 60, 40),
            "speedometer" => egui::Color32::from_rgb(40, 40, 60),
            "media" => egui::Color32::from_rgb(60, 60, 40),
            "stats" => egui::Color32::from_rgb(40, 60, 60),
            _ => egui::Color32::from_rgb(30, 30, 40),
        }
    }

    fn render_section(&self, ui: &mut egui::Ui, section: &Section) {
        egui::Frame::group(ui.style())
            .fill(Self::custom_color_for_section(&section.id))
            .stroke(egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE))
            .rounding(egui::Rounding::same(12.0))
            .inner_margin(egui::style::Margin::same(12.0))
            .show(ui, |ui| {
                match section.id.as_str() {
                    "messages" => {
                        ui.heading("Messages");
                        for content in &section.content {
                            if let SectionContent::Message { text } = content {
                                ui.label(text);
                            }
                        }
                    }
                    "carCondition" => {
                        ui.heading("Car Condition");
                        for content in &section.content {
                            match content {
                                SectionContent::Warning { text } => {
                                    ui.colored_label(egui::Color32::YELLOW, text);
                                }
                                SectionContent::Tire { pressure, location } => {
                                    ui.label(format!("Tire {}: {} psi", location, pressure));
                                }
                                _ => {}
                            }
                        }
                    }
                    "timeCondition" => {
                        ui.heading("Time/Map");
                        for content in &section.content {
                            match content {
                                SectionContent::Arrival { text } => {
                                    ui.label(text);
                                }
                                SectionContent::Map { content: map_content } => {
                                    for map in map_content {
                                        if let Some(route) = &map.route {
                                            ui.label(format!("Route: {}", route));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    "speedometer" => {
                        ui.heading("Speedometer");
                        for content in &section.content {
                            match content {
                                SectionContent::Speed { value, unit } => {
                                    ui.label(format!("Speed: {} {}", value, unit));
                                }
                                SectionContent::Rpm { value } => {
                                    ui.label(format!("RPM: {}", value));
                                }
                                _ => {}
                            }
                        }
                    }
                    "media" => {
                        ui.heading("Media Player");
                        for content in &section.content {
                            if let SectionContent::Player { content: player_content } = content {
                                for p in player_content {
                                    match p {
                                        PlayerContent::Status { text } => { ui.label(format!("Status: {}", text)); },
                                        PlayerContent::Track { text } => { ui.label(format!("Track: {}", text)); },
                                        PlayerContent::Volume { level } => { ui.label(format!("Volume: {}", level)); },
                                    }
                                }
                            }
                        }
                    }
                    "stats" => {
                        ui.heading("Stats");
                        for content in &section.content {
                            match content {
                                SectionContent::Time { text } => { ui.label(format!("Time: {}", text)); },
                                SectionContent::TotalDistance { value, unit } => { ui.label(format!("Total Distance: {} {}", value, unit)); },
                                SectionContent::Lap { distance, unit, number } => { ui.label(format!("Lap {}: {} {}", number, distance, unit)); },
                                _ => {},
                            }
                        }
                    }
                    _ => {
                        ui.heading(&section.id);
                        ui.label("Unknown section");
                    }
                }
            });
    }

    fn setup_custom_fonts(&mut self, ctx: &egui::Context) {
        use egui::{FontFamily, FontData, FontDefinitions};
        let mut fonts = FontDefinitions::default();
        if let Ok(data) = std::fs::read("fonts/YourFont.ttf") {
            fonts.font_data.insert(
                "my_font".to_owned(),
                FontData::from_owned(data),
            );
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "my_font".to_owned());
            ctx.set_fonts(fonts);
            self.font_loaded = true;
        }
    }
}

impl eframe::App for ConsoleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set up custom font once
        if !self.font_loaded {
            self.setup_custom_fonts(ctx);
        }
        // Simulate telemetry update
        {
            let mut data = self.telemetry_data.lock().unwrap();
            data.speed += 0.1;
            data.rpm += 10.0;
            if data.rpm > 8000.0 {
                data.rpm = 1000.0;
            }
        }

        if let Some(ref dashboard) = self.dashboard {
            // Each section in its own draggable window
            for section in &dashboard.sections {
                egui::Window::new(&section.id)
                    .default_width(320.0)
                    .default_height(220.0)
                    .show(ctx, |ui| {
                        self.render_section(ui, section);
                    });
            }
        } else if let Some(ref err) = self.ui_error {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.colored_label(egui::Color32::RED, err);
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("No dashboard layout loaded.");
            });
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Console",
        options,
        Box::new(|cc| Box::new(ConsoleApp::new(cc))),
    );
}
