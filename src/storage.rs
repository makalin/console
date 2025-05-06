use std::fs;
use serde_json;
use crate::telemetry::TelemetryData;

pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new(file_path: &str) -> Self {
        Storage { file_path: file_path.to_string() }
    }

    pub fn save(&self, data: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(data)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    pub fn load(&self) -> Result<TelemetryData, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(&self.file_path)?;
        let data: TelemetryData = serde_json::from_str(&json)?;
        Ok(data)
    }
} 