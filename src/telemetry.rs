use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct TelemetryData {
    pub speed: f64,
    pub rpm: f64,
} 