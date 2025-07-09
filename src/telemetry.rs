use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryData {
    pub speed: f64,
    pub rpm: f64,
    pub engine_temp: f64,
    pub fuel_level: f64,
    pub battery_voltage: f64,
    pub oil_pressure: f64,
    pub throttle_position: f64,
    pub brake_pressure: f64,
    pub gear: i32,
    pub timestamp: u64,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub acceleration: f64,
    pub brake_temperature: f64,
    pub tire_pressure_fl: f64,
    pub tire_pressure_fr: f64,
    pub tire_pressure_rl: f64,
    pub tire_pressure_rr: f64,
}

impl TelemetryData {
    /// Create a new TelemetryData instance with current timestamp
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            timestamp,
            ..Default::default()
        }
    }

    /// Update timestamp to current time
    pub fn update_timestamp(&mut self) {
        self.timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Check if engine is running based on RPM
    pub fn is_engine_running(&self) -> bool {
        self.rpm > 100.0
    }

    /// Get engine status as string
    pub fn engine_status(&self) -> &'static str {
        if self.is_engine_running() {
            "Running"
        } else {
            "Stopped"
        }
    }

    /// Calculate fuel efficiency (MPG approximation)
    pub fn fuel_efficiency(&self) -> f64 {
        if self.speed > 0.0 && self.rpm > 0.0 {
            // Simple approximation - in real implementation this would be more complex
            let load_factor = self.throttle_position / 100.0;
            let rpm_factor = if self.rpm > 3000.0 { 0.8 } else { 1.0 };
            (self.speed * rpm_factor) / (self.rpm * load_factor * 0.01)
        } else {
            0.0
        }
    }

    /// Check if any tire pressure is low (below 30 PSI)
    pub fn has_low_tire_pressure(&self) -> bool {
        self.tire_pressure_fl < 30.0 ||
        self.tire_pressure_fr < 30.0 ||
        self.tire_pressure_rl < 30.0 ||
        self.tire_pressure_rr < 30.0
    }

    /// Get the lowest tire pressure
    pub fn lowest_tire_pressure(&self) -> f64 {
        [
            self.tire_pressure_fl,
            self.tire_pressure_fr,
            self.tire_pressure_rl,
            self.tire_pressure_rr,
        ].iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }

    /// Check if engine temperature is in normal range
    pub fn is_engine_temp_normal(&self) -> bool {
        self.engine_temp >= 160.0 && self.engine_temp <= 220.0
    }

    /// Get engine temperature status
    pub fn engine_temp_status(&self) -> &'static str {
        if self.engine_temp < 160.0 {
            "Cold"
        } else if self.engine_temp > 220.0 {
            "Hot"
        } else {
            "Normal"
        }
    }

    /// Calculate acceleration in m/s²
    pub fn acceleration_ms2(&self) -> f64 {
        self.acceleration * 9.81 // Convert G to m/s²
    }

    /// Get gear as string representation
    pub fn gear_string(&self) -> String {
        match self.gear {
            -1 => "R".to_string(),
            0 => "N".to_string(),
            1..=6 => self.gear.to_string(),
            _ => "?".to_string(),
        }
    }

    /// Check if vehicle is in motion
    pub fn is_moving(&self) -> bool {
        self.speed > 1.0
    }

    /// Get speed in different units
    pub fn speed_kmh(&self) -> f64 {
        self.speed * 1.60934 // Convert MPH to KMH
    }

    pub fn speed_ms(&self) -> f64 {
        self.speed * 0.44704 // Convert MPH to m/s
    }

    /// Validate telemetry data for reasonable ranges
    pub fn is_valid(&self) -> bool {
        self.speed >= 0.0 && self.speed <= 200.0 &&
        self.rpm >= 0.0 && self.rpm <= 10000.0 &&
        self.engine_temp >= 0.0 && self.engine_temp <= 300.0 &&
        self.fuel_level >= 0.0 && self.fuel_level <= 100.0 &&
        self.battery_voltage >= 8.0 && self.battery_voltage <= 16.0 &&
        self.oil_pressure >= 0.0 && self.oil_pressure <= 100.0 &&
        self.throttle_position >= 0.0 && self.throttle_position <= 100.0 &&
        self.brake_pressure >= 0.0 && self.brake_pressure <= 2000.0 &&
        self.gear >= -1 && self.gear <= 6
    }

    /// Create a summary of critical alerts
    pub fn get_alerts(&self) -> Vec<String> {
        let mut alerts = Vec::new();
        
        if self.engine_temp > 220.0 {
            alerts.push("Engine temperature high!".to_string());
        }
        
        if self.has_low_tire_pressure() {
            alerts.push("Low tire pressure detected".to_string());
        }
        
        if self.battery_voltage < 11.0 {
            alerts.push("Low battery voltage".to_string());
        }
        
        if self.oil_pressure < 10.0 && self.is_engine_running() {
            alerts.push("Low oil pressure".to_string());
        }
        
        if self.fuel_level < 10.0 {
            alerts.push("Low fuel level".to_string());
        }
        
        alerts
    }
}

/// Utility functions for telemetry data processing
pub mod utils {
    use super::TelemetryData;
    use std::collections::VecDeque;

    /// Calculate moving average of telemetry values
    pub fn moving_average(data_points: &VecDeque<f64>, window_size: usize) -> f64 {
        if data_points.is_empty() || window_size == 0 {
            return 0.0;
        }
        
        let window = data_points.iter().rev().take(window_size);
        let sum: f64 = window.sum();
        sum / window_size.min(data_points.len()) as f64
    }

    /// Interpolate between two telemetry data points
    pub fn interpolate_telemetry(
        start: &TelemetryData,
        end: &TelemetryData,
        factor: f64,
    ) -> TelemetryData {
        TelemetryData {
            speed: start.speed + (end.speed - start.speed) * factor,
            rpm: start.rpm + (end.rpm - start.rpm) * factor,
            engine_temp: start.engine_temp + (end.engine_temp - start.engine_temp) * factor,
            fuel_level: start.fuel_level + (end.fuel_level - start.fuel_level) * factor,
            battery_voltage: start.battery_voltage + (end.battery_voltage - start.battery_voltage) * factor,
            oil_pressure: start.oil_pressure + (end.oil_pressure - start.oil_pressure) * factor,
            throttle_position: start.throttle_position + (end.throttle_position - start.throttle_position) * factor,
            brake_pressure: start.brake_pressure + (end.brake_pressure - start.brake_pressure) * factor,
            gear: start.gear, // Gear doesn't interpolate
            timestamp: start.timestamp + ((end.timestamp - start.timestamp) as f64 * factor) as u64,
            latitude: interpolate_option(start.latitude, end.latitude, factor),
            longitude: interpolate_option(start.longitude, end.longitude, factor),
            altitude: interpolate_option(start.altitude, end.altitude, factor),
            acceleration: start.acceleration + (end.acceleration - start.acceleration) * factor,
            brake_temperature: start.brake_temperature + (end.brake_temperature - start.brake_temperature) * factor,
            tire_pressure_fl: start.tire_pressure_fl + (end.tire_pressure_fl - start.tire_pressure_fl) * factor,
            tire_pressure_fr: start.tire_pressure_fr + (end.tire_pressure_fr - start.tire_pressure_fr) * factor,
            tire_pressure_rl: start.tire_pressure_rl + (end.tire_pressure_rl - start.tire_pressure_rl) * factor,
            tire_pressure_rr: start.tire_pressure_rr + (end.tire_pressure_rr - start.tire_pressure_rr) * factor,
        }
    }

    fn interpolate_option(start: Option<f64>, end: Option<f64>, factor: f64) -> Option<f64> {
        match (start, end) {
            (Some(s), Some(e)) => Some(s + (e - s) * factor),
            (Some(s), None) => Some(s),
            (None, Some(e)) => Some(e),
            (None, None) => None,
        }
    }

    /// Calculate distance between two GPS coordinates (Haversine formula)
    pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 6371000.0; // Earth's radius in meters
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin() * (dlat / 2.0).sin() +
                lat1.to_radians().cos() * lat2.to_radians().cos() *
                (dlon / 2.0).sin() * (dlon / 2.0).sin();
        let c = 2.0 * a.sqrt().asin();
        r * c
    }
} 