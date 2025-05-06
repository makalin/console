use egui::Ui;
use crate::telemetry::TelemetryData;

pub trait Plugin {
    fn init(&mut self);
    fn update(&mut self, data: &TelemetryData);
    fn render(&self, ui: &mut Ui);
} 