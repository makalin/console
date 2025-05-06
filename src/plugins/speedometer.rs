use egui::Ui;
use crate::plugin::Plugin;
use crate::telemetry::TelemetryData;

pub struct SpeedometerPlugin {
    speed: f64,
    rpm: f64,
}

impl SpeedometerPlugin {
    pub fn new() -> Self {
        SpeedometerPlugin { speed: 0.0, rpm: 0.0 }
    }
}

impl Plugin for SpeedometerPlugin {
    fn init(&mut self) {
        // Initialization logic if needed
    }

    fn update(&mut self, data: &TelemetryData) {
        self.speed = data.speed;
        self.rpm = data.rpm;
    }

    fn render(&self, ui: &mut Ui) {
        ui.heading("Speedometer");
        ui.label(format!("Speed: {:.1} km/h", self.speed));
        ui.label(format!("RPM: {:.0}", self.rpm));
    }
}

#[no_mangle]
pub extern "C" fn init_plugin() -> Box<dyn Plugin> {
    Box::new(SpeedometerPlugin::new())
} 