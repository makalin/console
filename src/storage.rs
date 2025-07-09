use std::fs;
use std::path::Path;
use serde_json;
use crate::telemetry::TelemetryData;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Storage {
    pub file_path: String,
    pub backup_dir: String,
}

impl Storage {
    pub fn new(file_path: &str) -> Self {
        let backup_dir = format!("{}.backups", file_path);
        Storage { 
            file_path: file_path.to_string(),
            backup_dir,
        }
    }

    pub fn save(&self, data: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    pub fn load(&self) -> Result<TelemetryData, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(&self.file_path)?;
        let data: TelemetryData = serde_json::from_str(&json)?;
        Ok(data)
    }

    /// Save multiple telemetry data points as a session
    pub fn save_session(&self, data_points: &[TelemetryData]) -> Result<(), Box<dyn std::error::Error>> {
        let session_data = serde_json::to_string_pretty(data_points)?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let session_file = format!("{}.session_{}", self.file_path, timestamp);
        fs::write(session_file, session_data)?;
        Ok(())
    }

    /// Load a session file
    pub fn load_session(&self, session_id: &str) -> Result<Vec<TelemetryData>, Box<dyn std::error::Error>> {
        let session_file = format!("{}.session_{}", self.file_path, session_id);
        let json = fs::read_to_string(session_file)?;
        let data: Vec<TelemetryData> = serde_json::from_str(&json)?;
        Ok(data)
    }

    /// Create a backup of current data
    pub fn create_backup(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Create backup directory if it doesn't exist
        if !Path::new(&self.backup_dir).exists() {
            fs::create_dir_all(&self.backup_dir)?;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let backup_file = format!("{}/backup_{}.json", self.backup_dir, timestamp);
        
        if Path::new(&self.file_path).exists() {
            fs::copy(&self.file_path, &backup_file)?;
        }
        
        Ok(backup_file)
    }

    /// List all available backups
    pub fn list_backups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        if !Path::new(&self.backup_dir).exists() {
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();
        for entry in fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Some(name) = path.file_name() {
                    backups.push(name.to_string_lossy().to_string());
                }
            }
        }
        backups.sort();
        Ok(backups)
    }

    /// Restore from a backup
    pub fn restore_backup(&self, backup_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let backup_path = format!("{}/{}", self.backup_dir, backup_name);
        if !Path::new(&backup_path).exists() {
            return Err("Backup file not found".into());
        }
        
        fs::copy(backup_path, &self.file_path)?;
        Ok(())
    }

    /// Export data to CSV format
    pub fn export_to_csv(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.load()?;
        let csv_content = format!(
            "timestamp,speed,rpm,engine_temp,fuel_level,battery_voltage,oil_pressure,throttle_position,brake_pressure,gear,acceleration,brake_temperature,tire_pressure_fl,tire_pressure_fr,tire_pressure_rl,tire_pressure_rr\n{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            data.timestamp,
            data.speed,
            data.rpm,
            data.engine_temp,
            data.fuel_level,
            data.battery_voltage,
            data.oil_pressure,
            data.throttle_position,
            data.brake_pressure,
            data.gear,
            data.acceleration,
            data.brake_temperature,
            data.tire_pressure_fl,
            data.tire_pressure_fr,
            data.tire_pressure_rl,
            data.tire_pressure_rr
        );
        
        fs::write(output_path, csv_content)?;
        Ok(())
    }

    /// Get storage statistics
    pub fn get_stats(&self) -> Result<StorageStats, Box<dyn std::error::Error>> {
        let mut stats = StorageStats::default();
        
        // Main file stats
        if Path::new(&self.file_path).exists() {
            let metadata = fs::metadata(&self.file_path)?;
            stats.main_file_size = metadata.len();
            stats.main_file_modified = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();
        }
        
        // Backup stats
        if Path::new(&self.backup_dir).exists() {
            stats.backup_count = fs::read_dir(&self.backup_dir)?.count();
        }
        
        Ok(stats)
    }

    /// Clean old backups (keep only the last N backups)
    pub fn clean_old_backups(&self, keep_count: usize) -> Result<usize, Box<dyn std::error::Error>> {
        let backups = self.list_backups()?;
        if backups.len() <= keep_count {
            return Ok(0);
        }
        
        let to_delete = backups.len() - keep_count;
        for backup in backups.iter().take(to_delete) {
            let backup_path = format!("{}/{}", self.backup_dir, backup);
            fs::remove_file(backup_path)?;
        }
        
        Ok(to_delete)
    }

    /// Check if storage is healthy
    pub fn is_healthy(&self) -> Result<bool, Box<dyn std::error::Error>> {
        // Check if main file is readable
        if Path::new(&self.file_path).exists() {
            let _ = self.load()?;
        }
        
        // Check if backup directory is accessible
        if Path::new(&self.backup_dir).exists() {
            let _ = fs::read_dir(&self.backup_dir)?;
        }
        
        Ok(true)
    }

    /// Get file size in human readable format
    pub fn get_file_size_human(&self) -> Result<String, Box<dyn std::error::Error>> {
        if !Path::new(&self.file_path).exists() {
            return Ok("0 B".to_string());
        }
        
        let metadata = fs::metadata(&self.file_path)?;
        let bytes = metadata.len();
        
        let units = ["B", "KB", "MB", "GB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < units.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        Ok(format!("{:.1} {}", size, units[unit_index]))
    }
}

#[derive(Default, Debug)]
pub struct StorageStats {
    pub main_file_size: u64,
    pub main_file_modified: u64,
    pub backup_count: usize,
}

impl StorageStats {
    pub fn is_recent(&self, max_age_seconds: u64) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        current_time - self.main_file_modified < max_age_seconds
    }
}

/// Utility functions for data compression and optimization
pub mod utils {
    use super::*;

    /// Compress telemetry data by removing redundant information
    pub fn compress_telemetry_data(data_points: &[TelemetryData]) -> Vec<TelemetryData> {
        if data_points.len() <= 2 {
            return data_points.to_vec();
        }
        
        let mut compressed = Vec::new();
        compressed.push(data_points[0].clone());
        
        for i in 1..data_points.len() - 1 {
            let prev = &data_points[i - 1];
            let current = &data_points[i];
            let next = &data_points[i + 1];
            
            // Only keep points that show significant change
            if has_significant_change(prev, current) || has_significant_change(current, next) {
                compressed.push(current.clone());
            }
        }
        
        compressed.push(data_points.last().unwrap().clone());
        compressed
    }

    fn has_significant_change(a: &TelemetryData, b: &TelemetryData) -> bool {
        (a.speed - b.speed).abs() > 1.0 ||
        (a.rpm - b.rpm).abs() > 100.0 ||
        (a.engine_temp - b.engine_temp).abs() > 5.0 ||
        (a.fuel_level - b.fuel_level).abs() > 2.0 ||
        a.gear != b.gear
    }

    /// Calculate data integrity hash
    pub fn calculate_data_hash(data: &TelemetryData) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        data.timestamp.hash(&mut hasher);
        ((data.speed * 100.0) as u64).hash(&mut hasher);
        ((data.rpm * 10.0) as u64).hash(&mut hasher);
        ((data.engine_temp * 10.0) as u64).hash(&mut hasher);
        data.gear.hash(&mut hasher);
        
        hasher.finish()
    }

    /// Validate data integrity
    pub fn validate_data_integrity(data: &TelemetryData, expected_hash: u64) -> bool {
        calculate_data_hash(data) == expected_hash
    }
} 